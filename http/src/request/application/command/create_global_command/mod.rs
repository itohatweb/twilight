mod chat_input;
mod message;
mod user;

pub use self::{
    chat_input::CreateGlobalChatInputCommand, message::CreateGlobalMessageCommand,
    user::CreateGlobalUserCommand,
};

use crate::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};
use twilight_validate::command::CommandValidationError;

/// Create a new global command.
#[must_use = "the command must have a type"]
pub struct CreateGlobalCommand<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
}

impl<'a> CreateGlobalCommand<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            http,
        }
    }

    /// Create a new chat input global command.
    ///
    /// The command name must only contain alphanumeric characters and lowercase
    /// variants must be used where possible. Special characters `-` and `_` are
    /// allowed. The description must be between 1 and 100 characters in length.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the discord docs] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] or [`NameCharacterInvalid`]
    /// if the command name is invalid.
    ///
    /// Returns an error of type [`DescriptionInvalid`] if the
    /// command description is not between 1 and 100 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [`NameCharacterInvalid`]: twilight_validate::command::CommandValidationErrorType::NameCharacterInvalid
    /// [`DescriptionInvalid`]: twilight_validate::command::CommandValidationErrorType::DescriptionInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn chat_input(
        self,
        name: &'a str,
        description: &'a str,
    ) -> Result<CreateGlobalChatInputCommand<'a>, CommandValidationError> {
        CreateGlobalChatInputCommand::new(self.http, self.application_id, name, description)
    }

    /// Create a new message global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the discord docs] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the command name is
    /// not between 1 and 32 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn message(
        self,
        name: &'a str,
    ) -> Result<CreateGlobalMessageCommand<'a>, CommandValidationError> {
        CreateGlobalMessageCommand::new(self.http, self.application_id, name)
    }

    /// Create a new user global command.
    ///
    /// Creating a command with the same name as an already-existing global
    /// command will overwrite the old command. See [the discord docs] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameLengthInvalid`] if the command name is
    /// not between 1 and 32 characters.
    ///
    /// [`NameLengthInvalid`]: twilight_validate::command::CommandValidationErrorType::NameLengthInvalid
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn user(
        self,
        name: &'a str,
    ) -> Result<CreateGlobalUserCommand<'a>, CommandValidationError> {
        CreateGlobalUserCommand::new(self.http, self.application_id, name)
    }
}
