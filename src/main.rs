mod cmds;
use cmds::*;
mod db;

use diesel::{
    prelude::SqliteConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use log::info;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{GatewayIntents, UserId};
use poise::Event;
use std::str::FromStr;
use std::{collections::HashSet, env::var};
use strum_macros::{Display, EnumString, IntoStaticStr};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type ConnType = SqliteConnection;
type Conn = PooledConnection<ConnectionManager<ConnType>>;

pub struct Data {
    db: Pool<ConnectionManager<ConnType>>,
}

#[derive(EnumString, IntoStaticStr, Display)]
pub enum ComponentAction {
    DeleteFromFavorites,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,)
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

async fn on_event(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::InteractionCreate { interaction } => match interaction {
            serenity::Interaction::MessageComponent(i) => {
                let params: Vec<&str> = i.data.custom_id.split('/').collect();
                if let Some((action, args)) = params.split_first() {
                    match ComponentAction::from_str(&action) {
                        Ok(ComponentAction::DeleteFromFavorites) => {
                            fav_msgs::delete(ctx, i, data, &args).await
                        }
                        _ => Ok(()),
                    }
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

async fn pre_command(ctx: Context<'_>) {
    info!("Executing command {}...", ctx.command().qualified_name);
}

fn owners() -> Result<HashSet<UserId>, Error> {
    var("BEAN_BOT_OWNERS").map_or(Ok(HashSet::new()), |arg| {
        arg.split(',')
            .map(|owner| Ok(owner.parse::<u64>()?.into()))
            .collect()
    })
}

async fn app() -> Result<(), Error> {
    let db = db::connect::<ConnType>();
    db::run_pending_migrations(&mut db.get()?);

    let options = poise::FrameworkOptions {
        commands: vec![
            general::help(),
            general::shutdown(),
            general::say(),
            fav_msgs::mystery(),
            fav_msgs::add(),
        ],
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(on_event(ctx, event, framework, user_data))
        },
        on_error: |err| Box::pin(on_error(err)),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        owners: owners()?,
        ..Default::default()
    };
    let token = var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(options)
        .token(token)
        .intents(intents)
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Registered commands and logged in as {}", ready.user.name);
                Ok(Data { db })
            })
        });
    framework.run().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = app().await {
        log::error!("{}", e);
        std::process::exit(1);
    }
}
