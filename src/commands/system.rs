use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

group!({
    name: "system",
    options: {
        owners_only: true,
    },
    commands: [quit, ping]
});

#[command]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    ctx.quit();
    let _ = msg.reply(&ctx, "Shutting down!");
    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(&ctx, "Pong!");
    Ok(())
}
