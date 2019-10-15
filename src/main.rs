#![warn(rust_2018_idioms)]

use serenity::{client::Client, framework::standard::StandardFramework};
use std::env;
mod standard_framerork_config;
use standard_framerork_config::StandardFrameworkConfig;

mod commands;
mod data;
mod handler;

use data::InitialData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let mut client = Client::new(&token, handler::Handler)?;

    let configurator = StandardFrameworkConfig::new(&client.cache_and_http.http)?;

    let initial_data = InitialData::from(&client);
    initial_data.insert(&mut client.data.write());

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
            })
            .after(|ctx, msg, command_name, res| {
                if let Err(err) = res {
                    println!(
                        "Error while processing {:?} command: {:?}",
                        command_name, err
                    );
                    if let Err(why) = msg.channel_id.say(&ctx.http, err.0) {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }),
    );

    client.start()?;
    Ok(())
}
