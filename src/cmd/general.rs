use crate::{AppError, Context};
use rand::seq::SliceRandom;

/// Show this help menu
#[poise::command(slash_command, ephemeral = true)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), AppError> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom:
                "This is an example bot made to showcase features of my custom Discord bot framework",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Gently euthanise Bean Bot in its sleep
#[poise::command(slash_command, owners_only, hide_in_help, ephemeral = true)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), AppError> {
    ctx.say("Shutting down...").await?;
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}

#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn say(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to say"]
    msg: String,
) -> Result<(), AppError> {
    ctx.channel_id().say(&ctx, msg).await?;
    ctx.send(
        poise::CreateReply::default()
            .content("Sent.")
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, rename = "ask-matthias")]
pub async fn ask_matthias(
    ctx: Context<'_>,
    #[rest]
    #[rename = "question"]
    _msg: String,
) -> Result<(), AppError> {
    let msg = build_matthias();
    ctx.send(poise::CreateReply::default().content(msg)).await?;
    Ok(())
}

fn build_matthias() -> String {
    let prefix = "<:phoenix:900483319039402014> | ";
    let options = ["Go ", "Big ", ""];
    let rand: &str = options.choose(&mut rand::thread_rng()).unwrap();
    prefix.to_owned() + rand + "slay!"
}

#[poise::command(prefix_command, slash_command, rename = "8ball")]
pub async fn eight_ball(
    ctx: Context<'_>,
    #[rest]
    #[rename = "question"]
    _msg: String,
) -> Result<(), AppError> {
    let rngesus: f64 = rand::random();
    let msg = if rngesus <= 0.01 {
        build_matthias()
    } else {
        let prefix = "🎱 | ";
        let options = [
            "Yes, definitely.",
            "It is certain.",
            "It is decidedly so.",
            "Without a doubt",
            "Most likely.",
            "You may rely on it.",
            "Signs point to yes",
            "As I see it, yes.",
            "Most likely.",
            "Outlook good.",
            "Yes.",
            "My reply is no.",
            "Outlook not so good.",
            "Very doubtful.",
            "My sources say no.",
            "Don't count on it.",
        ];
        let rand: &str = options.choose(&mut rand::thread_rng()).unwrap();
        prefix.to_owned() + rand
    };
    ctx.send(poise::CreateReply::default().content(msg)).await?;
    Ok(())
}
