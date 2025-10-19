use tokio::fs::File;
use poise::{CreateReply, serenity_prelude::CreateAttachment};

use crate::{database, Context, Error};

mod ddvt;

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
            extra_text_at_bottom: "\
            Type ?help command for more info on a command.",
            ephemeral: false,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// register and unregister commands
#[poise::command(
    prefix_command,
    slash_command,
    category = "Admin",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    owners_only
)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Dap dau vao tuong, might fail
#[poise::command(
    prefix_command,
    slash_command,
    category = "Fluff",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn dap_dau_vao_tuong(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author().display_name();
    let result = rand::random::<bool>();
    let result_text = if result { "thanh cong" } else { "that bai" };

    let response = format!("{user} da dap dau vao tuong {result_text}.");
    ctx.say(response).await?;
    Ok(())
}

/// Dap dau vao tuong, the game
#[poise::command(
    prefix_command,
    slash_command,
    category = "Game",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    subcommands("damage_check", "play", "health_check"),
    subcommand_required
)]
pub async fn ddvt(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Smash your head against the wall once
#[poise::command(
    prefix_command,
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn play(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author().display_name();
    let id = ctx.author().id;
    let result = rand::random::<bool>();
    let result_text = if result { "thanh cong" } else { "that bai" };
    let damage_dealt = if result { 1.0 } else { 0.5 };
    let damage_taken = 1.0;

    let mut player_data = database::get_player_data(i64::from(id))?;
    player_data = ddvt::regen(player_data);
    let old_health = player_data.health;
    player_data = ddvt::attack(player_data, damage_dealt, damage_taken);
    let new_health = player_data.health;
    database::set_player_data(&player_data)?;

    let attack_result = format!(
        "{user} da dap dau vao tuong {result_text}.\nTuong da nhan {damage_dealt} sat thuong. Ban con lai {0} HP.",
        new_health
    );
    let response = if old_health <= 0.0 {
        "YOU ALREADY DEER".to_string()
    } else if new_health <= 0.0 {
        format!("{attack_result}\nYOU DEER")
    } else {
        attack_result
    };

    ctx.send(CreateReply {
        content: Some(response),
        attachments: if new_health <= 0.0 {
            let dead_image_file = File::open("assets/shes-dead.png").await?;
            let dead_image = CreateAttachment::file(&dead_image_file, "shes-dead.png").await?;
            vec![dead_image]
        } else {
            vec![]
        },
        ..Default::default()
    })
    .await?;
    Ok(())
}

/// Checks your total damage dealt to the wall
#[poise::command(
    prefix_command,
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn damage_check(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author().display_name();
    let id = ctx.author().id;
    let player_data = database::get_player_data(i64::from(id))?;

    let response = format!("{user} da gay ra tong cong {0} sat thuong cho tuong.", player_data.total_damage);
    ctx.say(response).await?;
    Ok(())
}

/// Checks your remaining HP
#[poise::command(
    prefix_command,
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn health_check(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author().display_name();
    let id = ctx.author().id;

    let mut player_data = database::get_player_data(i64::from(id))?;
    player_data = ddvt::regen(player_data);
    database::set_player_data(&player_data)?;

    let response = format!("{user} con lai {0} HP.\n", player_data.health);
    let next_regen = if player_data.health <= 0.0 {
        let respawn_time = player_data.last_attack + (2.0 * player_data.max_health) as i64 * 60;
        format!("Ban se hoi sinh vao luc <t:{respawn_time}:f>.")
    }
    else {
        let next_heal_time = player_data.last_attack + 60;
        format!("Ban se hoi phuc 1 HP vao luc <t:{next_heal_time}:f>.")
    };
    ctx.say(response + &next_regen).await?;
    Ok(())
}