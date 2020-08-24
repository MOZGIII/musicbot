use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::channel::Message,
};

#[hook]
pub async fn unrecognised_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let _ = msg.channel_id.say(
        &ctx.http,
        format!("Could not find command named '{}'", unknown_command_name,),
    );
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(duration) => {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", duration),
                )
                .await;
        }
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "This command can only be invoked by owners")
                .await;
        }
        _ => (),
    }
}

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );
    true
}

#[hook]
pub async fn after(
    ctx: &Context,
    msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(err) = command_result {
        println!(
            "Error while processing {:?} command: {:?}",
            command_name, err
        );
        if let Err(why) = msg.channel_id.say(&ctx.http, err).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
