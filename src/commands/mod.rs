pub mod experimental;
pub mod system;
pub mod voice;

pub mod prelude {
    pub use crate::data::DataExt;
    pub use serenity::{
        client::Context,
        framework::standard::{
            macros::{command, group},
            Args, CommandError, CommandResult,
        },
        model::{channel::Message, misc::Mentionable},
    };
}
