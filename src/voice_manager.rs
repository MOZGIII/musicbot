use std::sync::Arc;

use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::Mutex;

// This imports `typemap`'s `Key` as `TypeMapKey`.
use serenity::prelude::*;

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub fn register_in_data(client: &mut Client) {
    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    let mut data = client.data.write();
    data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
}

// Export some useful stuff as prelude.
pub mod prelude {
    pub use super::VoiceManager;
}
