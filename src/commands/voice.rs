use super::prelude::*;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::TypeMap;
use serenity::voice;

#[group]
#[commands(join, leave, play)]
struct Voice;

/// Get the guild the message was posted to, or bail if the message has no
/// guild (i.e. it was a DM or group message).
async fn get_message_guild(ctx: &Context, msg: &Message) -> Result<Guild, CommandError> {
    let guild = msg
        .guild(&ctx.cache)
        .await
        .ok_or_else(|| "Groups and DMs not supported")?;
    Ok(guild)
}

/// Get a message author's voice channel, or bail if there's no voice channel
/// (i.e. message author is not in a voice channel).
async fn get_message_author_voice_channel(
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

async fn join_voice_channel<G, C>(data: &TypeMap, guild_id: G, channel_id: C) -> CommandResult
where
    G: Into<GuildId>,
    C: Into<ChannelId>,
{
    let voice_manager_mutex = data.voice_manager();
    let mut voice_manager = voice_manager_mutex.lock().await;

    let _ = voice_manager
        .join(guild_id, channel_id)
        .ok_or_else(|| "Unable to join voice channel")?;

    Ok(())
}

async fn leave_voice_channel<G>(data: &TypeMap, guild_id: G) -> CommandResult
where
    G: Into<GuildId> + Copy,
{
    let voice_manager_mutex = data.voice_manager();
    let mut voice_manager = voice_manager_mutex.lock().await;

    let _ = voice_manager
        .get(guild_id)
        .ok_or_else(|| "Not in a voice channel")?;

    let _ = voice_manager
        .remove(guild_id)
        .ok_or_else(|| "Unable to leave voice channel")?;

    Ok(())
}

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = get_message_guild(ctx, msg).await?;
    let (guild_id, voice_channel_id) = get_message_author_voice_channel(&guild, msg).await?;
    let data = ctx.data.read().await;
    join_voice_channel(&*data, guild_id, voice_channel_id).await?;
    msg.channel_id
        .say(&ctx.http, &format!("Joined {}", voice_channel_id.mention()))
        .await?;
    Ok(())
}

#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = get_message_guild(ctx, msg).await?;
    let data = ctx.data.read().await;
    leave_voice_channel(&*data, guild.id).await?;
    msg.channel_id.say(&ctx.http, "Left voice channel").await?;
    Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args
        .single::<String>()
        .map_err(|_| "You must provide a URL to a YouTube video or audio or a search query")?;

    let guild = get_message_guild(ctx, msg).await?;

    let (guild_id, voice_channel_id) = get_message_author_voice_channel(&guild, msg).await?;
    let data = ctx.data.read().await;
    join_voice_channel(&*data, guild_id, voice_channel_id).await?;

    let voice_manager_mutex = data.voice_manager();
    let mut voice_manager = voice_manager_mutex.lock().await;

    let voice_control_handle = voice_manager
        .get_mut(guild.id)
        .ok_or_else(|| "Not in a voice channel to play in")?;

    let is_url = arg.starts_with("http");
    let source = if is_url {
        voice::ytdl(&arg)
            .await
            .map_err(|err| format!("Unable to play video from a YouTube URL: {:?}", err))?
    } else {
        voice::ytdl_search(&arg).await.map_err(|err| {
            format!(
                "Unable to play video from the YouTube search results: {:?}",
                err
            )
        })?
    };

    voice_control_handle.play_only(source);
    msg.channel_id.say(&ctx.http, "Playing song").await?;
    Ok(())
}
