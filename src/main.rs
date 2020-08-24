#![warn(rust_2018_idioms)]

use serenity::{client::Client, framework::standard::StandardFramework, http::Http};
use std::env;
mod standard_framerork_config;
use standard_framerork_config::StandardFrameworkConfig;

mod commands;
mod data;
mod handler;
mod help;
mod hook;

use handler::Handler;

use data::InitialData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token =
        env::var("DISCORD_TOKEN").map_err(|_| "Expected a DISCORD_TOKEN in the environment")?;

    let http = Http::new_with_token(&token);
    let configurator = StandardFrameworkConfig::new(&http).await?;

    let framework = StandardFramework::new()
        .configure(|c| configurator.configure(c))
        .group(&commands::voice::VOICE_GROUP)
        .group(&commands::system::SYSTEM_GROUP)
        .group(&commands::experimental::EXPERIMENTAL_GROUP)
        .help(&help::HELP)
        .before(hook::before)
        .unrecognised_command(hook::unrecognised_command)
        .normal_message(hook::normal_message)
        .on_dispatch_error(hook::dispatch_error)
        .after(hook::after);

    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    {
        let initial_data = InitialData::from(&client);
        let mut data = client.data.write().await;
        initial_data.insert(&mut *data);
    }

    client.start().await?;
    Ok(())
}
