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
        macros::help,
    },
    model::{channel::Message, id::UserId},
};

mod standard_framerork_config;
use standard_framerork_config::StandardFrameworkConfig;
mod commands;
mod handler;
mod voice_manager;

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
