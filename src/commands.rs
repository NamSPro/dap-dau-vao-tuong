use crate::{ Error, Context };

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Dap dau vao tuong, might fail
#[poise::command(
    prefix_command,
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn dap_dau_vao_tuong(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let user = ctx.author().display_name();
    let result = rand::random::<bool>();
    let result_text = if result {"thanh cong"} else {"that bai"};
    let damage = if result {1.0} else {0.5};

    let response = format!("{user} da dap dau vao tuong {result_text}.\nTuong da nhan {damage} sat thuong.");
    ctx.say(response).await?;
    Ok(())
}
