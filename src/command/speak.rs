use crate::prelude::*;

pub const NAME: &str = "speak";
pub const OPTION_CONTENT: &str = "content";
pub const OPTION_REPLY: &str = "reply";

pub fn new() -> CreateCommand {
    CreateCommand::new(NAME)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("Speak, monkey!")
        .dm_permission(false)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                OPTION_CONTENT,
                "What should the monkey say?",
            )
            .max_length(2000)
            .clone()
            .required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            OPTION_REPLY,
            "Whether to reply to the last sent message",
        ))
}

pub async fn run_command(http: &Http, command: &CommandInteraction) -> Result<()> {
    command.defer_ephemeral(http).await?;

    let o = &command.data.options();
    let content = get_str(o, OPTION_CONTENT)?;
    let reply = get_bool(o, OPTION_REPLY).unwrap_or_default();
    let mut message = CreateMessage::new().content(content);

    if reply {
        let reference = command
            .channel_id
            .messages(http, GetMessages::new().limit(1))
            .await?
            .pop()
            .ok_or_else(|| anyhow!("No valid message!"))?;

        message = message.reference_message(&reference);
    }

    command.channel_id.send_message(http, message).await?;

    let follow_up = CreateInteractionResponseFollowup::new().content("The monkey has spoken!");
    command.create_followup(http, follow_up).await?;
    Ok(())
}
