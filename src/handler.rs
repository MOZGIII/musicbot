use serenity::client::Context;

use serenity::{
    client::EventHandler,
    model::{event::ResumedEvent, gateway::Ready},
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}
