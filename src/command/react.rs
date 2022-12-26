use crate::prelude::*;

pub const NAME: &str = "react";
pub const OPTION_EMOJIS: &str = "emojis";

pub fn new() -> CreateCommand {
    CreateCommand::new(NAME)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("Monkey see, monkey do!")
        .dm_permission(false)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                OPTION_EMOJIS,
                "Up to 20 different emojis, space separated",
            )
            .required(true),
        )
}

pub async fn run_command(http: &Http, command: &CommandInteraction) -> Result<()> {
    command.defer_ephemeral(http).await?;

    let mut emojis = get_str(&command.data.options(), OPTION_EMOJIS)?
        .split(' ')
        .map_while(|slice| ReactionType::try_from(slice).ok())
        .take(20)
        .collect::<Vec<_>>();

    emojis.dedup();

    if emojis.is_empty() {
        return Err(anyhow!("No valid reactions provided!"));
    }

    let message = command
        .channel_id
        .messages(http, GetMessages::new().limit(1))
        .await?
        .pop()
        .ok_or_else(|| anyhow!("No valid message!"))?;

    for emoji in emojis {
        message.react(http, emoji).await?;
    }

    let follow_up = CreateInteractionResponseFollowup::new().content("Live monkey reaction!!! ^");
    command.create_followup(http, follow_up).await?;
    Ok(())
}
