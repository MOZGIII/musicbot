use serenity::client::bridge::{gateway::ShardManager, voice::ClientVoiceManager};
use serenity::prelude::*;
use std::sync::Arc;

mod data_container_key;
pub use data_container_key::*;

pub type VoiceManagerKey = DataContainerKey<Arc<Mutex<ClientVoiceManager>>>;
pub type ShardManagerKey = DataContainerKey<Arc<Mutex<ShardManager>>>;

pub struct InitialData {
    voice_manager: Arc<Mutex<ClientVoiceManager>>,
    shard_manager: Arc<Mutex<ShardManager>>,
}

impl From<&Client> for InitialData {
    fn from(client: &Client) -> Self {
        Self {
            voice_manager: Arc::clone(&client.voice_manager),
            shard_manager: Arc::clone(&client.shard_manager),
        }
    }
}

impl InitialData {
    pub fn insert(self, data: &mut ShareMap) {
        VoiceManagerKey::insert(data, self.voice_manager);
        ShardManagerKey::insert(data, self.shard_manager);
    }
}

pub trait DataExt {
    fn voice_manager(&self) -> Arc<Mutex<ClientVoiceManager>>;
    fn shard_manager(&self) -> Arc<Mutex<ShardManager>>;
}

impl DataExt for ShareMap {
    fn voice_manager(&self) -> Arc<Mutex<ClientVoiceManager>> {
        Arc::clone(VoiceManagerKey::get(self).unwrap())
    }

    fn shard_manager(&self) -> Arc<Mutex<ShardManager>> {
        Arc::clone(ShardManagerKey::get(self).unwrap())
    }
}
