use std::collections::HashSet;
use std::env;

use serenity::{
    client::{Context, Client},
    framework::standard::{
        HelpOptions,
        help_commands,
        Args,
        CommandResult,
        CommandGroup,
        StandardFramework,
        macros::{
            command,
            group,
            help,
        },
    },
    model::{channel::Message, id::UserId, misc::Mentionable},
    voice, Result as SerenityResult,
};

mod standard_framerork_config;
use standard_framerork_config::StandardFrameworkConfig;
mod commands;
mod handler;
mod voice_manager;
use voice_manager::prelude::*;

#[help]
#[individual_command_tip =
"Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
fn bot_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main() -> Result<(), Box<std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let mut client = Client::new(&token, handler::Handler)?;

    let configurator = StandardFrameworkConfig::new(&client.cache_and_http.http)?;

    voice_manager::register_in_data(&mut client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| configurator.configure(c))
            .help(&BOT_HELP_HELP_COMMAND)
            .group(&VOICE_GROUP)
            .group(&commands::system::SYSTEM_GROUP)
            .group(&commands::experimental::EXPERIMENTAL_GROUP)
            .before(|_, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );
                true
            })
            .unrecognised_command(|ctx, msg, unknown_command_name| {
                let _ = msg.channel_id.say(&ctx.http, format!(
                    "Could not find command named '{}'",
                    unknown_command_name,
                ));
            })
            .normal_message(|_, message| {
                println!("Message is not a command '{}'", message.content);
            })
            .on_dispatch_error(|ctx, msg, error| {
                if let serenity::framework::standard::DispatchError::OnlyForOwners = error {
                    let _ = msg
                        .channel_id
                        .say(&ctx.http, "This command can only be invoked by owners");
                }
            }),
    );

    client.start()?;
    Ok(())
}

group!({
    name: "voice",
    options: {},
    commands: [join, leave, play_raw, ping]
});

#[command]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        }
    };

    let guild_id = guild.read().id;

    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect_to).is_some() {
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));
    }

    Ok(())
}

#[command]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel"));
    } else {
        check_msg(msg.reply(&ctx, "Not in a voice channel"));
    }

    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!"));
    Ok(())
}

#[command]
fn play_raw(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio"));

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL"));

        return Ok(());
    }

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ytdl(&url) {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg"));

                return Ok(());
            },
        };

        handler.play(source);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song"));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in"));
    }

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
