use tokio::fs::File;
use poise::{CreateReply, serenity_prelude::CreateAttachment};

use crate::{Context, Error};

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

/// show beta notice
#[poise::command(
    prefix_command,
    slash_command,
    category = "Info",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
)]
pub async fn beta_notice(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This bot is in beta. Wipes may happen at any time and without warning.").await?;
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
    let music = if result {
        File::open("assets/victory.flac").await?
    } else {
        File::open("assets/defeat.flac").await?
    };
    ctx.send(CreateReply {
        content: Some(response),
        attachments: vec![CreateAttachment::file(&music, "bgm.flac").await?],
        ..Default::default()
    })
    .await?;
    Ok(())
}

/// Dap dau vao tuong, the game
#[poise::command(
    prefix_command,
    slash_command,
    category = "Game",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    subcommands("damage_check", "play", "health_check", "upgrade"),
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

    let (old_health, _, _) = ddvt::regen(i64::from(id))?;
    let (result, damage_dealt, new_health) = ddvt::attack(i64::from(id))?;
    let result_text = if result { "thanh cong" } else { "that bai" };

    let attack_result = format!(
        "{user} da dap dau vao tuong {result_text}.\nTuong da nhan {damage_dealt} sat thuong. Ban con lai {0} HP.\nNhan duoc {damage_dealt} vinh du.",
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
    let damage = ddvt::get_total_damage(i64::from(id))?;

    let response = format!("{user} da gay ra tong cong {damage} sat thuong cho tuong.");
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

    let (health, player_max_health, player_last_attack) = ddvt::regen(i64::from(id))?;

    let response = format!("{user} con lai {health}/{player_max_health} HP.\n");
    let next_regen = if health <= 0.0 {
        let respawn_time = player_last_attack + (2.0 * player_max_health) as i64 * 60;
        format!("Ban se hoi sinh vao luc <t:{respawn_time}:f>.")
    }
    else if health < player_max_health {
        let next_heal_time = player_last_attack + 60;
        format!("Ban se hoi phuc 1 HP vao luc <t:{next_heal_time}:f>.")
    }
    else {
        "".to_string()
    };
    ctx.say(response + &next_regen).await?;
    Ok(())
}

/// Upgrades stats
#[poise::command(
    prefix_command,
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn upgrade(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}