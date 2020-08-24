use serenity::async_trait;
use serenity::client::Context;

use serenity::{
    client::EventHandler,
    model::{event::ResumedEvent, gateway::Ready},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}
