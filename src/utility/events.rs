use serenity::{all::OnlineStatus, prelude::EventHandler};

use crate::prelude::*;

#[derive(Debug)]
pub struct Events {
    pub logger: Logger,
}

impl Events {
    pub const fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub fn info(&self, content: impl Into<String>) {
        self.logger.info(content).ok();
    }
    pub fn warn(&self, content: impl Into<String>) {
        self.logger.warn(content).ok();
    }
    pub fn error(&self, content: impl Into<String>) {
        self.logger.error(content).ok();
    }

    pub async fn patch_commands(&self, http: &Http) -> Result<()> {
        let guild_id = dev_guild()?;
        let cmds = vec![react::new(), speak::new()];

        let global = if DEV_BUILD {
            http.get_global_application_commands().await?.len()
        } else {
            http.create_global_application_commands(&cmds).await?.len()
        };

        self.info(format!("Patched {global} global commands"));

        let guild = guild_id.set_application_commands(http, cmds).await?.len();

        self.logger.info(format!("Patched {guild} guild commands"))
    }
    pub async fn search_react(&self, http: &Http, message: &Message) -> Result<()> {
        if message.author.bot {
            return Ok(());
        }

        let triggers = Stored::<Vec<String>>::read("react", "triggers", Kind::Ron)?;

        if let Some(word) = message.search(triggers.to_vec(), true, true) {
            let responses = Stored::<WeightVec<char>>::read("react", "responses", Kind::Ron)?;
            let response = *responses.get().ok_or_else(|| anyhow!("no responses"))?;

            self.info(format!("Reacting to \"{word}\" with '{response}'"));

            message.react(http, response).await?;
        }

        Ok(())
    }
    pub async fn search_reply(&self, http: &Http, message: &Message) -> Result<()> {
        if message.author.bot {
            return Ok(());
        }

        let triggers = Stored::<Vec<String>>::read("reply", "triggers", Kind::Ron)?;

        if let Some(word) = message.search(triggers.to_vec(), true, true) {
            let responses =
                Stored::<WeightVec<(String, Option<char>)>>::read("reply", "responses", Kind::Ron)?;
            let (text, emoji) = responses.get().ok_or_else(|| anyhow!("no responses"))?;
            let emoji = emoji.map(|c| c.to_string()).unwrap_or_default();
            let response = format!("{emoji} {text} {emoji}").trim().to_string();

            self.info(format!("Replying to \"{word}\" with \"{response}\""));

            let reply = CreateMessage::new()
                .reference_message(message)
                .content(response);

            message.channel_id.send_message(http, reply).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, ctx: Context, ready: Ready) {
        self.info(format!("Connected as \"{}\"", ready.user.tag()));

        if let Some(count) = ready.shard.map(|s| s.total) {
            self.info(format!("Using {count} shards"));
        }

        ctx.set_presence(
            Some(ActivityData::watching("for apes 🙈")),
            OnlineStatus::Idle,
        );

        if let Err(error) = self.patch_commands(&ctx.http).await {
            self.warn(error.to_string());
        }
    }
    async fn message(&self, ctx: Context, message: Message) {
        if let Err(error) = self.search_react(&ctx.http, &message).await {
            self.warn(format!("Error reacting: {error}"));
        }
        if let Err(error) = self.search_reply(&ctx.http, &message).await {
            self.warn(format!("Error replying: {error}"));
        }
    }
    async fn interaction_create(&self, ctx: Context, mut interaction: Interaction) {
        let id = match &interaction {
            Interaction::Autocomplete(i) => format!("{}<a:{}>", i.data.name, i.id),
            Interaction::Command(i) => format!("{}<c:{}>", i.data.name, i.id),
            Interaction::Component(i) => format!("{}<b:{}>", i.data.custom_id, i.id),
            Interaction::Modal(i) => format!("{}<m:{}>", i.data.custom_id, i.id),
            Interaction::Ping(i) => format!("{}<p:{}>", i.token, i.id),
        };

        let http = &ctx.http;

        let result = match &mut interaction {
            Interaction::Command(command) => match command.data.name.as_str() {
                react::NAME => react::run_command(http, command).await,
                speak::NAME => speak::run_command(http, command).await,
                _ => Err(anyhow!("unknown interaction: {id}")),
            },
            _ => Err(anyhow!("unknown interaction: {id}")),
        };

        if let Err(error) = result {
            self.error(format!("Interaction failed: {id} - {error}"));

            let embed = CreateEmbed::new()
                .color(Color::GOLD)
                .description(format!("> {error}"))
                .title("Encountered an error!");
            let r = CreateInteractionResponseFollowup::new()
                .embed(embed)
                .ephemeral(true);

            let result = match &interaction {
                Interaction::Command(i) => i.create_followup(http, r).await.map_err(Into::into),
                Interaction::Component(i) => i.create_followup(http, r).await.map_err(Into::into),
                Interaction::Modal(i) => i.create_followup(http, r).await.map_err(Into::into),
                _ => Err(anyhow!("invalid interaction type")),
            };

            if let Err(error) = result {
                self.warn(format!("Error could not be displayed: {error}"));
            }
        } else {
            self.info(format!("Interaction succeeded: {id}"));
        }
    }
}
