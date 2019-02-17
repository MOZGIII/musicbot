#[macro_use]
extern crate serenity;

use std::env;

use serenity::{
    client::{Client, CACHE},
    framework::StandardFramework,
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
    voice_manager::register_in_data(&mut client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.on_mention(true))
            .cmd("join", join)
            .cmd("leave", leave)
            .cmd("play", play_raw)
            .cmd("ping", ping)
            .command("quit", |c| c.cmd(commands::system::quit).owners_only(true)),
    );

    client.start()?;
    Ok(())
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
