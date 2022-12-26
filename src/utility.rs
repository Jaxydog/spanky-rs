use crate::prelude::*;

pub mod events;
pub mod logger;
pub mod random;
pub mod search;
pub mod stored;

pub const DEV_BUILD: bool = cfg!(debug_assertions);
pub const INTENTS: GatewayIntents = GatewayIntents::DIRECT_MESSAGES
    .union(GatewayIntents::DIRECT_MESSAGE_REACTIONS)
    .union(GatewayIntents::GUILDS)
    .union(GatewayIntents::GUILD_MESSAGES)
    .union(GatewayIntents::GUILD_MESSAGE_REACTIONS)
    .union(GatewayIntents::MESSAGE_CONTENT);

pub fn token() -> Result<String> {
    std::env::var(if DEV_BUILD { "DEV_TOKEN" } else { "TOKEN" }).map_err(Into::into)
}
pub fn dev_guild() -> Result<GuildId> {
    Ok(GuildId::new(std::env::var("DEV_GUILD")?.parse()?))
}
