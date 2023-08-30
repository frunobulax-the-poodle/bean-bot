mod db;
mod model;

use crate::{AppError, ComponentAction, Context, Data};
use diesel::result::DatabaseErrorKind;
use log::{error, info};
use model::*;
use poise::serenity_prelude as serenity;
use serenity::{builder::*, model::prelude::*, CacheHttp};

/// Add a message to your favorites
#[poise::command(
    context_menu_command = "Add to Favorites",
    guild_only = true,
    ephemeral = true
)]
pub async fn add(ctx: Context<'_>, msg: Message) -> Result<(), AppError> {
    let new = NewFavorite {
        user_id: ctx.author().id.into(),
        guild_id: ctx.guild_id().unwrap().into(),
        channel_id: ctx.channel_id().into(),
        message_id: msg.id.into(),
    };

    let mut conn = ctx.data().db.get()?;

    if db::find(&mut conn, &new)?.is_some() {
        ctx.say("You already favorited this message.").await?;
        return Ok(());
    }

    match db::add_fav(&mut conn, &new) {
        Ok(_) => Ok(ctx.say("Saved.")),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Ok(ctx.say("You already favorited this message."))
        }
        Err(e) => Err(e),
    }?
    .await?;
    Ok(())
}

/// Post a random message from your or the server's favorites
#[poise::command(slash_command, guild_only = true)]
pub async fn mystery(
    ctx: Context<'_>,
    #[description = "Draw from server's global favorites if set"] global: Option<bool>,
) -> Result<(), AppError> {
    let mut conn = ctx.data().db.get()?;
    loop {
        let fav = db::rand(
            &mut conn,
            Some(ctx.author().id.into()).filter(|_| !global.unwrap_or(false)),
            ctx.guild_id().unwrap().into(),
        )?;

        if let Some(rand) = fav {
            if let Some(msg) = fetch_msg(&ctx, &rand).await? {
                let author_nick = msg
                    .author
                    .nick_in(&ctx, rand.guild_id as u64)
                    .await
                    .unwrap_or(msg.author.name.to_owned());
                let mut embed = CreateEmbed::default().description(&msg.content).author(
                    CreateEmbedAuthor::new(author_nick)
                        .icon_url(msg.author.avatar_url().unwrap_or("".to_string())),
                );
                if let Some(attach) = msg.attachments.iter().find(|a| a.height.is_some()) {
                    embed = embed.image(&attach.url);
                }
                ctx.send(poise::CreateReply::default().embed(embed).components(vec![
                    CreateActionRow::Buttons(vec![
                            CreateButton::new_link(msg.link()).label("Source"),
                            CreateButton::new(format!(
                                        "{}/{}/{}",
                                        ComponentAction::DeleteFromFavorites,
                                        rand.channel_id,
                                        rand.message_id,
                                    ))
                                .style(ButtonStyle::Danger)
                                    .label("Remove from Favorites"),
                            ]),
                ]))
                .await?;
                break;
            } else {
                info!("Favorited message has been deleted, deleting...");
                db::delete(&mut conn, rand.id)?;
            }
        } else {
            ctx.send(
                poise::CreateReply::default()
                    .content("No messages favorited yet!")
                    .ephemeral(true),
            )
            .await?;
            break;
        };
    }
    Ok(())
}

async fn fetch_msg(ctx: &Context<'_>, fav: &FavoritedMessage) -> Result<Option<Message>, AppError> {
    match ctx
        .http()
        .get_message(
            ChannelId::new(fav.channel_id as u64),
            MessageId::new(fav.message_id as u64),
        )
        .await
    {
        Ok(msg) => Ok(Some(msg)),
        Err(serenity::Error::Http(err)) => match err.status_code() {
            Some(serenity::http::StatusCode::NOT_FOUND) => Ok(None),
            _ => {
                error!("Could not fetch favorited message: {:?}", err);
                Err(Box::new(err))
            }
        },
        Err(err) => {
            error!("Could not fetch favorited message: {:?}", err);
            Err(Box::new(err))
        }
    }
}

pub async fn delete(
    ctx: &serenity::Context,
    event: &ComponentInteraction,
    data: &Data,
    args: &[&str],
) -> Result<(), AppError> {
    let channel_id: u64 = args[0].parse()?;
    let message_id: u64 = args[1].parse()?;
    let search = NewFavorite {
        user_id: event.user.id.get() as i64,
        guild_id: event.guild_id.unwrap().get() as i64,
        channel_id: channel_id as i64,
        message_id: message_id as i64,
    };

    let res = if db::delete_by(&mut data.db.get()?, &search)? > 0 {
        "Successfully removed from favorites."
    } else {
        "This message is not in your favorites."
    };

    event
        .create_response(
            ctx,
            serenity::CreateInteractionResponse::Message(
                serenity::CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .content(res),
            ),
        )
        .await?;
    Ok(())
}
