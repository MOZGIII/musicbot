#[macro_use]
extern crate serenity;

use std::collections::HashSet;
use std::env;

use serenity::{
    client::{Client, CACHE},
    framework::StandardFramework,
    http,
    model::{channel::Message, misc::Mentionable},
    voice, Result as SerenityResult,
};

mod commands;
mod handler;
mod voice_manager;
use voice_manager::prelude::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let mut client = Client::new(&token, handler::Handler)?;

    let owners = load_app_owners()?;

    voice_manager::register_in_data(&mut client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.on_mention(true).owners(owners))
            .cmd("join", join)
            .cmd("leave", leave)
            .cmd("play", play_raw)
            .cmd("ping", ping)
            .command("quit", |c| c.cmd(commands::system::quit).owners_only(true))
            .group("Experimental", |g| {
                g.owners_only(true)
                    .command("exp1", |c| c.cmd(commands::experimental::exp1))
            })
            .before(|_, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );
                true
            })
            .unrecognised_command(|_, msg, unknown_command_name| {
                let _ = msg.channel_id.say(format!(
                    "Could not find command named '{}'",
                    unknown_command_name,
                ));
            })
            .message_without_command(|_, message| {
                println!("Message is not a command '{}'", message.content);
            })
            .on_dispatch_error(|_ctx, msg, error| {
                if let serenity::framework::standard::DispatchError::OnlyForOwners = error {
                    let _ = msg
                        .channel_id
                        .say("This command can only be invoked by owners");
                }
            }),
    );

    client.start()?;
    Ok(())
}

fn load_app_owners() -> Result<HashSet<serenity::model::id::UserId>, Box<std::error::Error>> {
    let owner_id = http::get_current_application_info()?.owner.id;
    let mut set = HashSet::new();
    set.insert(owner_id);
    Ok(set)
}

command!(join(ctx, msg) {
    let guild = match msg.guild() {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));

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
            check_msg(msg.reply("Not in a voice channel"));

            return Ok(());
        }
    };

    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect_to).is_some() {
        check_msg(msg.channel_id.say(&format!("Joined {}", connect_to.mention())));
    } else {
        check_msg(msg.channel_id.say("Error joining the channel"));
    }
});

command!(leave(ctx, msg) {
    let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        check_msg(msg.channel_id.say("Left voice channel"));
    } else {
        check_msg(msg.reply("Not in a voice channel"));
    }
});

command!(ping(_context, msg) {
    check_msg(msg.channel_id.say("Pong!"));
});

command!(play_raw(ctx, msg, args) {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say("Must provide a URL to a video or audio"));

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say("Must provide a valid URL"));

        return Ok(());
    }

    let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say("Error finding channel info"));

            return Ok(());
        },
    };

    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ytdl(&url) {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say("Error sourcing ffmpeg"));

                return Ok(());
            },
        };

        handler.play(source);

        check_msg(msg.channel_id.say("Playing song"));
    } else {
        check_msg(msg.channel_id.say("Not in a voice channel to play in"));
    }
});

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
