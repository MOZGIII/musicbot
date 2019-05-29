pub mod experimental;
pub mod system;
pub mod voice;

pub mod prelude {
    pub use serenity::{
        client::Context,
        framework::standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        model::{channel::Message, misc::Mentionable},
    };
}
