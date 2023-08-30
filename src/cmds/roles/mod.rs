mod db;
mod model;
use std::{collections::HashSet, str::FromStr};

use crate::{AppError, Context};

use diesel::result::DatabaseErrorKind;
use poise::serenity_prelude as serenity;
use serenity::{application::*, builder::*, model::id::RoleId};

use model::*;

#[poise::command(
    slash_command,
    guild_only = true,
    ephemeral = true,
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "MANAGE_ROLES",
    subcommands("new", "del", "rename")
)]
pub async fn rolemenu(_ctx: Context<'_>) -> Result<(), AppError> {
    Ok(())
}

#[poise::command(slash_command, ephemeral = true)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "Name of the role menu"] name: String,
    #[min = 1]
    #[max = 25]
    #[description = "Maximum number of selectable roles"]
    max_selectable: Option<i32>,
) -> Result<(), AppError> {
    let mut conn = ctx.data().db.get()?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;

    if db::find(&mut conn, guild_id, &name)?.is_some() {
        ctx.say(format!("The role menu '{}' already exists", &name))
            .await?;
        return Ok(());
    }

    let id = ctx.id();
    let handle = ctx.send(role_select(&id.to_string(), &name)).await?;

    let res = serenity::ComponentInteractionCollector::new(&ctx)
        .filter(move |d| d.data.custom_id == id.to_string())
        .timeout(std::time::Duration::from_secs(120))
        .await;

    if let Some(interaction) = res {
        if let ComponentInteractionDataKind::RoleSelect { values } = &interaction.data.kind {
            let new = NewRoleMenu {
                guild_id,
                name,
                max_selectable,
                roles: values.into_iter().map(|r| r.get() as i64).collect(),
            };
            log::info!("Creating new role menu {:?}", new);
            // Conflict could happen, whatever
            db::new(&mut conn, &new)?;
            interaction.defer(ctx).await?;
            handle
                .edit(
                    ctx,
                    poise::CreateReply::new()
                        .content("Successfully created role menu")
                        .components(vec![]),
                )
                .await?;
        } else {
            unreachable!();
        }
    } else {
        handle.delete(ctx).await?;
    }

    Ok(())
}

#[poise::command(slash_command, ephemeral = true)]
pub async fn del(
    ctx: Context<'_>,
    #[autocomplete = "comp_rolemenu"]
    #[description = "Name of the role menu to delete"]
    name: String,
) -> Result<(), AppError> {
    let mut conn = ctx.data().db.get()?;
    let deleted = db::delete(&mut conn, ctx.guild_id().unwrap().get() as i64, &name)?;
    let msg = if deleted > 0 {
        format!("Deleted role menu '{}'", &name)
    } else {
        format!("Could not find role menu '{}'", &name)
    };
    ctx.send(poise::CreateReply::new().content(msg)).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral = true)]
pub async fn rename(
    ctx: Context<'_>,
    #[autocomplete = "comp_rolemenu"]
    #[description = "Name of the role menu to rename"]
    from: String,
    #[description = "What to rename it to"] to: String,
) -> Result<(), AppError> {
    let mut conn = ctx.data().db.get()?;
    let guild_id = ctx.guild_id().unwrap().get() as i64;

    let msg = match db::rename(&mut conn, guild_id, &from, &to) {
        Ok(0) => Ok(format!("Could not find role menu '{}'", &from)),
        Ok(_) => Ok(format!("Renamed '{}' to '{}'", &from, &to)),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Ok(format!("The role menu '{}' already exists", &to))
        }
        Err(e) => Err(e),
    }?;
    ctx.say(msg).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    ephemeral = true,
    guild_only = true,
    required_bot_permissions = "MANAGE_ROLES"
)]
pub async fn roles(
    ctx: Context<'_>,
    #[description = "Name of the role menu to"]
    #[autocomplete = "comp_rolemenu"]
    name: String,
) -> Result<(), AppError> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    let mut conn = ctx.data().db.get()?;
    if let Some(menu) = db::find(&mut conn, guild_id, &name)? {
        let mut member = ctx.author_member().await.unwrap().into_owned();
        let user_roles: HashSet<&RoleId> = HashSet::from_iter(member.roles.iter());
        let handle = send_rolemenu(&ctx, &menu, &user_roles).await?;
        let id = ctx.id();

        let res = serenity::ComponentInteractionCollector::new(&ctx)
            .filter(move |d| d.data.custom_id == id.to_string())
            .timeout(std::time::Duration::from_secs(120))
            .await;

        if let Some(interaction) = res {
            if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
                interaction.defer(ctx).await?;

                let value_set: HashSet<RoleId> =
                    HashSet::from_iter(values.iter().map(|str| RoleId::from_str(str).unwrap()));
                let (mut add, mut del): (Vec<_>, Vec<_>) = menu
                    .roles
                    .iter()
                    .map(|id| RoleId::new(*id as u64))
                    .partition(|id| value_set.contains(id));

                // Yes, this is necessary 🤦
                add.retain(|r| !user_roles.contains(r));
                del.retain(|r| user_roles.contains(r));

                member.add_roles(&ctx, &add).await?;
                member.remove_roles(&ctx, &del).await?;

                handle
                    .edit(
                        ctx,
                        poise::CreateReply::new()
                            .content("Successfully assigned roles")
                            .components(vec![]),
                    )
                    .await?;
            } else {
                unreachable!();
            }
        } else {
            handle.delete(ctx).await?;
        }
    } else {
        ctx.say(format!("The role menu '{}' does not exist", &name))
            .await?;
    }
    Ok(())
}

async fn send_rolemenu<'a>(
    ctx: &Context<'a>,
    menu: &RoleMenu,
    user_roles: &HashSet<&RoleId>,
) -> Result<poise::ReplyHandle<'a>, AppError> {
    let guild = ctx.partial_guild().await.unwrap();

    let options: Vec<_> = menu
        .roles
        .iter()
        .map(|id| guild.roles.get(&RoleId::new(*id as u64)))
        .flatten()
        .map(|role| {
            CreateSelectMenuOption::new(&role.name, role.id.get().to_string())
                .default_selection(user_roles.contains(&role.id))
        })
        .collect();
    let max_values = std::cmp::min(menu.roles.len(), menu.max_selectable.unwrap_or(25) as usize);
    let select = CreateSelectMenu::new(
        ctx.id().to_string(),
        CreateSelectMenuKind::String { options },
    )
    .min_values(1)
    .max_values(max_values as u64);

    let res =
        ctx.send(poise::CreateReply::new().components(vec![CreateActionRow::SelectMenu(select)]));
    Ok(res.await?)
}

async fn comp_rolemenu(ctx: Context<'_>, partial: &str) -> Vec<String> {
    let guild_id = ctx.guild_id().unwrap().get() as i64;
    ctx.data()
        .db
        .get()
        .ok()
        .and_then(|mut conn| db::comp_rolemenu(&mut conn, guild_id, partial).ok())
        .unwrap_or(Vec::new())
}

fn role_select(id: &str, name: &str) -> poise::CreateReply {
    poise::CreateReply::default()
        .content(format!(
            "Choose the roles that should be selectable in the rolemenu '{}'",
            &name
        ))
        .ephemeral(true)
        .components(vec![CreateActionRow::SelectMenu(
            CreateSelectMenu::new(id, CreateSelectMenuKind::Role).max_values(25),
        )])
}
