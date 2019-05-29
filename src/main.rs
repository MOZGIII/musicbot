#![warn(rust_2018_idioms)]

use std::env;

use serenity::{client::Client, framework::standard::StandardFramework};

mod standard_framerork_config;
use standard_framerork_config::StandardFrameworkConfig;
mod commands;
mod handler;
mod voice_manager;
mod helpers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let mut client = Client::new(&token, handler::Handler)?;

    let configurator = StandardFrameworkConfig::new(&client.cache_and_http.http)?;

    voice_manager::register_in_data(&mut client);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| configurator.configure(c))
            .group(&commands::voice::VOICE_GROUP)
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
                let _ = msg.channel_id.say(
                    &ctx.http,
                    format!("Could not find command named '{}'", unknown_command_name,),
                );
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
