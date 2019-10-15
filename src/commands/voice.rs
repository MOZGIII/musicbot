use super::prelude::*;
use serenity::voice;

group!({
    name: "voice",
    options: {
        only_in: "guilds",
    },
    commands: [join, leave, play_raw]
});

#[command]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_mutex = msg
        .guild(&ctx.cache)
        .ok_or_else(|| "Groups and DMs not supported")?;

    let guild = guild_mutex.read();

    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = channel_id.ok_or_else(|| "Not in a voice channel")?;

    let voice_manager_mutex = ctx.data.read().voice_manager();
    let mut voice_manager = voice_manager_mutex.lock();

    let _ = voice_manager
        .join(guild_id, connect_to)
        .ok_or_else(|| "Error joining the channel")?;

    msg.channel_id
        .say(&ctx.http, &format!("Joined {}", connect_to.mention()))?;
    Ok(())
}

#[command]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_channel = ctx
        .cache
        .read()
        .guild_channel(msg.channel_id)
        .ok_or_else(|| "Groups and DMs not supported")?;

    let guild_id = guild_channel.read().guild_id;

    let voice_manager_mutex = ctx.data.read().voice_manager();
    let mut voice_manager = voice_manager_mutex.lock();

    let _ = voice_manager
        .get(guild_id)
        .ok_or_else(|| "Not in a voice channel")?;

    let _ = voice_manager
        .remove(guild_id)
        .ok_or_else(|| "Unable to leave voice channel")?;

    msg.channel_id.say(&ctx.http, "Left voice channel")?;

    Ok(())
}

#[command]
fn play_raw(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args
        .single::<String>()
        .map_err(|_| "Must provide a URL to a video or audio")?;

    if !url.starts_with("http") {
        return Err("Must provide a valid URL".into());
    }

    let guild_channel = ctx
        .cache
        .read()
        .guild_channel(msg.channel_id)
        .ok_or_else(|| "Groups and DMs not supported")?;

    let guild_id = guild_channel.read().guild_id;

    let voice_manager_mutex = ctx.data.read().voice_manager();
    let mut voice_manager = voice_manager_mutex.lock();

    let handler = voice_manager
        .get_mut(guild_id)
        .ok_or_else(|| "Not in a voice channel to play in")?;

    let source = match voice::ytdl(&url) {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);
            return Err("Error sourcing ffmpeg".into());
        }
    };

    handler.play(source);

    msg.channel_id.say(&ctx.http, "Playing song")?;

    Ok(())
}
