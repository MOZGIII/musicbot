use super::prelude::*;

group!({
    name: "system",
    options: {
        owners_only: true,
    },
    commands: [quit, ping]
});

#[command]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    data.shard_manager().lock().shutdown_all();
    let _ = msg.reply(&ctx, "Shutting down!");
    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(&ctx, "Pong!");
    Ok(())
}
