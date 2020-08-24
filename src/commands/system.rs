use super::prelude::*;

#[group]
#[owners_only]
#[commands(quit, ping)]
struct System;

#[command]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    data.shard_manager().lock().await.shutdown_all().await;
    let _ = msg.reply(&ctx, "Shutting down!");
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(&ctx, "Pong!");
    Ok(())
}
