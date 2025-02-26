mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGuildChatInputCommand, message::CreateGuildMessageCommand,
    user::CreateGuildUserCommand,
};

use crate::Client;
use twilight_model::id::{
    marker::{ApplicationMarker, GuildMarker},
    Id,
};
use twilight_validate::command::CommandValidationError;

/// Create a new command in a guild.
#[must_use = "the command must have a type"]
pub struct CreateGuildCommand<'a> {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> CreateGuildCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            application_id,
            guild_id,
            http,
        }
    }

    /// Create a chat input command in a guild.
    ///
    /// The command name must only contain alphanumeric characters and lowercase
    /// variants must be used where possible. Special characters `-` and `_` are
    /// allowed. The description must be between 1 and 100 characters in length.
    ///
    /// Creating a guild command with the same name as an already-existing guild
    /// command in the same guild will overwrite the old command. See [the
    /// discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] or [`NameCharacterInvalid`]
    /// if the command name is invalid.
    ///
    /// Returns an error of type [`DescriptionInvalid`] error type if the
    /// command description is not between 1 and 100 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [`NameCharacterInvalid`]: twilight_validate::command::CommandValidationErrorType::NameCharacterInvalid
    /// [`DescriptionInvalid`]: twilight_validate::command::CommandValidationErrorType::DescriptionInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub fn chat_input(
        self,
        name: &'a str,
        description: &'a str,
    ) -> Result<CreateGuildChatInputCommand<'a>, CommandValidationError> {
        CreateGuildChatInputCommand::new(
            self.http,
            self.application_id,
            self.guild_id,
            name,
            description,
        )
    }

    /// Create a message command in a guild.
    ///
    /// Creating a guild command with the same name as an already-existing guild
    /// command in the same guild will overwrite the old command. See [the
    /// discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the command name is
    /// not between 1 and 32 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub fn message(
        self,
        name: &'a str,
    ) -> Result<CreateGuildMessageCommand<'a>, CommandValidationError> {
        CreateGuildMessageCommand::new(self.http, self.application_id, self.guild_id, name)
    }

    /// Create a user command in a guild.
    ///
    /// Creating a guild command with the same name as an already-existing guild
    /// command in the same guild will overwrite the old command. See [the
    /// discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the command name is
    /// not between 1 and 32 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub fn user(self, name: &'a str) -> Result<CreateGuildUserCommand<'a>, CommandValidationError> {
        CreateGuildUserCommand::new(self.http, self.application_id, self.guild_id, name)
    }
}
