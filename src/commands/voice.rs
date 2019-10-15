use super::prelude::*;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::RwLock;
use serenity::prelude::ShareMap;
use serenity::voice;
use std::sync::Arc;

group!({
    name: "voice",
    options: {
        only_in: "guilds",
    },
    commands: [join, leave, play]
});

/// Get the guild the message was posted to, or bail if the message has no
/// guild (i.e. it was a DM or group message).
fn get_message_guild(ctx: &mut Context, msg: &Message) -> Result<Arc<RwLock<Guild>>, CommandError> {
    let guild = msg
        .guild(&ctx.cache)
        .ok_or_else(|| "Groups and DMs not supported")?;
    Ok(guild)
}

/// Get a message author's voice channel, or bail if there's no voice channel
/// (i.e. message author is not in a voice channel).
fn get_message_author_voice_channel(
    guild: &Guild,
    msg: &Message,
) -> Result<(GuildId, ChannelId), CommandError> {
    // Get message's author's coive channel ID.
    let voice_channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or_else(|| "Not in a voice channel")?;
    Ok((guild.id, voice_channel_id))
}

fn join_voice_channel<G, C>(data: &ShareMap, guild_id: G, channel_id: C) -> CommandResult
where
    G: Into<GuildId>,
    C: Into<ChannelId>,
{
    let voice_manager_mutex = data.voice_manager();
    let mut voice_manager = voice_manager_mutex.lock();

    let _ = voice_manager
        .join(guild_id, channel_id)
        .ok_or_else(|| "Unable to join voice channel")?;

    Ok(())
}

fn leave_voice_channel<G>(data: &ShareMap, guild_id: G) -> CommandResult
where
    G: Into<GuildId> + Copy,
{
    let voice_manager_mutex = data.voice_manager();
    let mut voice_manager = voice_manager_mutex.lock();

    let _ = voice_manager
        .get(guild_id)
        .ok_or_else(|| "Not in a voice channel")?;

    let _ = voice_manager
        .remove(guild_id)
        .ok_or_else(|| "Unable to leave voice channel")?;

    Ok(())
}

#[command]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = get_message_guild(ctx, msg)?;
    let guild = guild.read();
    let (guild_id, voice_channel_id) = get_message_author_voice_channel(&guild, msg)?;
    join_voice_channel(&ctx.data.read(), guild_id, voice_channel_id)?;
    msg.channel_id
        .say(&ctx.http, &format!("Joined {}", voice_channel_id.mention()))?;
    Ok(())
}

#[command]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = get_message_guild(ctx, msg)?;
    let guild = guild.read();
    leave_voice_channel(&ctx.data.read(), guild.id)?;
    msg.channel_id.say(&ctx.http, "Left voice channel")?;
    Ok(())
}

#[command]
fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args
        .single::<String>()
        .map_err(|_| "You must provide a URL to a YouTube video or audio or a search query")?;

    let guild = get_message_guild(ctx, msg)?;
    let guild = guild.read();

    let (guild_id, voice_channel_id) = get_message_author_voice_channel(&guild, msg)?;
    join_voice_channel(&ctx.data.read(), guild_id, voice_channel_id)?;

    let voice_manager_mutex = ctx.data.read().voice_manager();
    let mut voice_manager = voice_manager_mutex.lock();

    let voice_control_handle = voice_manager
        .get_mut(guild.id)
        .ok_or_else(|| "Not in a voice channel to play in")?;

    let is_url = arg.starts_with("http");
    let source = if is_url {
        voice::ytdl(&arg)
            .map_err(|err| format!("Unable to play video from a YouTube URL: {:?}", err))?
    } else {
        voice::ytdl_search(&arg).map_err(|err| {
            format!(
                "Unable to play video from the YouTube search results: {:?}",
                err
            )
        })?
    };

    voice_control_handle.play_only(source);
    msg.channel_id.say(&ctx.http, "Playing song")?;
    Ok(())
}
