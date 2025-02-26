//! Constants, error types, and functions for validating [`Command`]s.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::application::command::{Command, CommandOption, CommandType};

/// Maximum number of choices an option can have.
pub const CHOICES_LIMIT: usize = 25;

/// Maximum length of a command's description.
pub const DESCRIPTION_LENGTH_MAX: usize = 100;

/// Minimum length of a command's description.
pub const DESCRIPTION_LENGTH_MIN: usize = 1;

/// Maximum length of a command's name.
pub const NAME_LENGTH_MAX: usize = 32;

/// Minimum length of a command's name.
pub const NAME_LENGTH_MIN: usize = 1;

/// Maximum amount of options a command may have.
pub const OPTIONS_LIMIT: usize = 25;

/// Maximum length of a command's description.
pub const OPTION_DESCRIPTION_LENGTH_MAX: usize = 100;

/// Minimum length of a command's description.
pub const OPTION_DESCRIPTION_LENGTH_MIN: usize = 1;

/// Maximum length of a command's name.
pub const OPTION_NAME_LENGTH_MAX: usize = 32;

/// Minimum length of a command's name.
pub const OPTION_NAME_LENGTH_MIN: usize = 1;

/// Maximum number of commands an application may have in an individual
/// guild.
pub const GUILD_COMMAND_LIMIT: usize = 100;

/// Maximum number of permission overwrites an application may have in an
/// individual guild command.
pub const GUILD_COMMAND_PERMISSION_LIMIT: usize = 10;

/// Error created when a [`Command`] is invalid.
#[derive(Debug)]
pub struct CommandValidationError {
    /// Type of error that occurred.
    kind: CommandValidationErrorType,
}

impl CommandValidationError {
    /// Constant instance of a [`CommandValidationError`] with type
    /// [`CountInvalid`].
    ///
    /// [`CountInvalid`]: CommandValidationErrorType::CountInvalid
    pub const COMMAND_COUNT_INVALID: CommandValidationError = CommandValidationError {
        kind: CommandValidationErrorType::CountInvalid,
    };

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CommandValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CommandValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }

    /// Create an error of type [`OptionsRequiredFirst`] with a provided index.
    ///
    /// [`OptionsRequiredFirst`]: CommandValidationErrorType::OptionsRequiredFirst
    #[must_use = "creating an error has no effect if left unused"]
    pub const fn option_required_first(index: usize) -> Self {
        Self {
            kind: CommandValidationErrorType::OptionsRequiredFirst { index },
        }
    }
}

impl Display for CommandValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CommandValidationErrorType::CountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&GUILD_COMMAND_LIMIT, f)?;

                f.write_str(" commands were set")
            }
            CommandValidationErrorType::DescriptionInvalid => {
                f.write_str("command description must be between ")?;
                Display::fmt(&DESCRIPTION_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&DESCRIPTION_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::NameLengthInvalid => {
                f.write_str("command name must be between ")?;
                Display::fmt(&NAME_LENGTH_MIN, f)?;
                f.write_str(" and ")?;

                Display::fmt(&NAME_LENGTH_MAX, f)
            }
            CommandValidationErrorType::NameCharacterInvalid { character } => {
                f.write_str(
                    "command name must only contain lowercase alphanumeric characters, found `",
                )?;
                Display::fmt(character, f)?;

                f.write_str("`")
            }
            CommandValidationErrorType::OptionDescriptionInvalid => {
                f.write_str("command option description must be between ")?;
                Display::fmt(&OPTION_DESCRIPTION_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&OPTION_DESCRIPTION_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::OptionNameLengthInvalid => {
                f.write_str("command option name must be between ")?;
                Display::fmt(&OPTION_NAME_LENGTH_MIN, f)?;
                f.write_str(" and ")?;

                Display::fmt(&OPTION_NAME_LENGTH_MAX, f)
            }
            CommandValidationErrorType::OptionNameCharacterInvalid { character } => {
                f.write_str("command option name must only contain lowercase alphanumeric characters, found `")?;
                Display::fmt(character, f)?;

                f.write_str("`")
            }
            CommandValidationErrorType::OptionsCountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&OPTIONS_LIMIT, f)?;

                f.write_str(" options were set")
            }
            CommandValidationErrorType::OptionsRequiredFirst { .. } => {
                f.write_str("optional command options must be added after required")
            }
            CommandValidationErrorType::PermissionsCountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&GUILD_COMMAND_PERMISSION_LIMIT, f)?;

                f.write_str(" permission overwrites were set")
            }
        }
    }
}

impl Error for CommandValidationError {}

/// Type of [`CommandValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CommandValidationErrorType {
    /// Too many commands have been provided.
    ///
    /// The maximum number of commands is defined by
    /// [`GUILD_COMMAND_LIMIT`].
    CountInvalid,
    /// Command description is invalid.
    DescriptionInvalid,
    /// Command name length is invalid.
    NameLengthInvalid,
    /// Command name contain an invalid character.
    NameCharacterInvalid {
        /// Invalid character.
        character: char,
    },
    /// Command option description is invalid.
    OptionDescriptionInvalid,
    /// Command option name length is invalid.
    OptionNameLengthInvalid,
    /// Command option name contain an invalid character.
    OptionNameCharacterInvalid {
        /// Invalid character.
        character: char,
    },
    /// Command options count invalid.
    OptionsCountInvalid,
    /// Required command options have to be passed before optional ones.
    OptionsRequiredFirst {
        /// Index of the option that failed validation.
        index: usize,
    },
    /// More than 10 permission overwrites were set.
    PermissionsCountInvalid,
}

/// Validate a [`Command`].
///
/// # Errors
///
/// Returns an error of type [`DescriptionInvalid`] if the description is
/// invalid.
///
/// Returns an error of type [`NameLengthInvalid`] or [`NameCharacterInvalid`]
/// if the name is invalid.
///
/// [`DescriptionInvalid`]: CommandValidationErrorType::DescriptionInvalid
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
pub fn command(value: &Command) -> Result<(), CommandValidationError> {
    let Command {
        description,
        name,
        kind,
        ..
    } = value;

    self::description(description)?;

    match kind {
        CommandType::ChatInput => self::chat_input_name(name),
        CommandType::User | CommandType::Message => self::name(name),
    }
}

/// Validate the description of a [`Command`].
///
/// The length of the description must be more than [`DESCRIPTION_LENGTH_MIN`]
/// and less than or equal to [`DESCRIPTION_LENGTH_MAX`].
///
/// # Errors
///
/// Returns an error of type [`DescriptionInvalid`] if the description is
/// invalid.
///
/// [`DescriptionInvalid`]: CommandValidationErrorType::DescriptionInvalid
pub fn description(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    if (DESCRIPTION_LENGTH_MIN..=DESCRIPTION_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::DescriptionInvalid,
        })
    }
}

/// Validate the name of a [`User`] or [`Message`] command.
///
/// The length of the name must be more than [`NAME_LENGTH_MIN`] and less than
/// or equal to [`NAME_LENGTH_MAX`].
///
/// Use [`chat_input_name`] to validate name of a [`ChatInput`] command.
///
/// # Errors
///
/// Returns an error of type [`NameLengthInvalid`] if the name is invalid.
///
/// [`User`]: CommandType::User
/// [`Message`]: CommandType::Message
/// [`ChatInput`]: CommandType::ChatInput
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
pub fn name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    if (NAME_LENGTH_MIN..=NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::NameLengthInvalid,
        })
    }
}

/// Validate the name of a [`ChatInput`] command.
///
/// The length of the name must be more than [`NAME_LENGTH_MIN`] and less than
/// or equal to [`NAME_LENGTH_MAX`]. It can only contain alphanumeric characters
/// and lowercase variants must be used where possible. Special characters `-`
/// and `_` are allowed.
///
/// # Errors
///
/// Returns an error of type [`NameLengthInvalid`] if the length is invalid.
///
/// Returns an error of type [`NameCharacterInvalid`] if the name contains a
/// non-alphanumeric character or an uppercase character for which a lowercase
/// variant exists.
///
/// [`ChatInput`]: CommandType::ChatInput
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
pub fn chat_input_name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    self::name(&value)?;

    self::name_characters(value)?;

    Ok(())
}

/// Validate the name of a [`CommandOption`].
///
/// The length of the name must be more than [`NAME_LENGTH_MIN`] and less than
/// or equal to [`NAME_LENGTH_MAX`]. It can only contain alphanumeric characters
/// and lowercase variants must be used where possible. Special characters `-`
/// and `_` are allowed.
///
/// # Errors
///
/// Returns an error of type [`NameLengthInvalid`] if the length is invalid.
///
/// Returns an error of type [`NameCharacterInvalid`] if the name contains a
/// non-alphanumeric character or an uppercase character for which a lowercase
/// variant exists.
///
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
pub fn option_name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    if !(OPTION_NAME_LENGTH_MIN..=OPTION_NAME_LENGTH_MAX).contains(&len) {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::NameLengthInvalid,
        });
    }

    self::name_characters(value)?;

    Ok(())
}

/// Validate the characters of a [`ChatInput`] command name or a
/// [`CommandOption`] name.
///
/// The name can only contain alphanumeric characters and lowercase variants
/// must be used where possible. Special characters `-` and `_` are allowed.
///
/// # Errors
///
/// Returns an error of type [`NameCharacterInvalid`] if the name contains a
/// non-alphanumeric character or an uppercase character for which a lowercase
/// variant exists.
///
/// [`ChatInput`]: CommandType::ChatInput
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
fn name_characters(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let chars = value.as_ref().chars();

    for char in chars {
        if !char.is_alphanumeric() && char != '_' && char != '-' {
            return Err(CommandValidationError {
                kind: CommandValidationErrorType::NameCharacterInvalid { character: char },
            });
        }

        if char.to_lowercase().next() != Some(char) {
            return Err(CommandValidationError {
                kind: CommandValidationErrorType::NameCharacterInvalid { character: char },
            });
        }
    }

    Ok(())
}

/// Validate a single [`CommandOption`].
///
/// # Errors
///
/// Returns an error of type [`OptionDescriptionInvalid`] if the description is
/// invalid.
///
/// Returns an error of type [`OptionNameLengthInvalid`] or [`OptionNameCharacterInvalid`]
/// if the name is invalid.
///
/// [`OptionDescriptionInvalid`]: CommandValidationErrorType::OptionDescriptionInvalid
/// [`OptionNameLengthInvalid`]: CommandValidationErrorType::OptionNameLengthInvalid
/// [`OptionNameCharacterInvalid`]: CommandValidationErrorType::OptionNameCharacterInvalid
pub fn option(option: &CommandOption) -> Result<(), CommandValidationError> {
    let (description, name) = match option {
        CommandOption::SubCommand(_) | CommandOption::SubCommandGroup(_) => return Ok(()),
        CommandOption::String(data) => (&data.description, &data.name),
        CommandOption::Integer(data) | CommandOption::Number(data) => {
            (&data.description, &data.name)
        }
        CommandOption::Channel(data) => (&data.description, &data.name),
        CommandOption::Boolean(data)
        | CommandOption::User(data)
        | CommandOption::Role(data)
        | CommandOption::Mentionable(data) => (&data.description, &data.name),
    };

    let description_len = description.chars().count();
    if description_len > OPTION_DESCRIPTION_LENGTH_MAX
        && description_len < OPTION_DESCRIPTION_LENGTH_MIN
    {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::OptionDescriptionInvalid,
        });
    }

    self::option_name(name)
}

/// Validate a list of command options for count, order, and internal validity.
///
/// # Errors
///
/// Returns an error of type [`OptionsRequiredFirst`] if a required option is
/// listed before an optional option.
///
/// Returns an error of type [`OptionsCountInvalid`] if the list of options or
/// any sub-list of options is too long.
///
/// [`OptionsRequiredFirst`]: CommandValidationErrorType::OptionsRequiredFirst
/// [`OptionsCountInvalid`]: CommandValidationErrorType::OptionsCountInvalid
pub fn options(options: &[CommandOption]) -> Result<(), CommandValidationError> {
    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
    if options.len() > OPTIONS_LIMIT {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::OptionsCountInvalid,
        });
    }

    // Validate that there are no required options listed after optional ones.
    options
        .iter()
        .zip(options.iter().skip(1))
        .enumerate()
        .try_for_each(|(index, (first, second))| {
            if !first.is_required() && second.is_required() {
                Err(CommandValidationError::option_required_first(index))
            } else {
                Ok(())
            }
        })?;

    // Validate that each option is correct.
    options.iter().try_for_each(|option| match option {
        CommandOption::SubCommandGroup(data) | CommandOption::SubCommand(data) => {
            self::options(data.options.as_ref())
        }
        other => self::option(other),
    })?;

    Ok(())
}

/// Validate the number of guild command permission overwrites.
///
/// The maximum number of commands allowed in a guild is defined by
/// [`GUILD_COMMAND_PERMISSION_LIMIT`].
///
/// # Errors
///
/// Returns an error of type [`PermissionsCountInvalid`] if the permissions are
/// invalid.
///
/// [`PermissionsCountInvalid`]: CommandValidationErrorType::PermissionsCountInvalid
pub const fn guild_permissions(count: usize) -> Result<(), CommandValidationError> {
    // https://discord.com/developers/docs/interactions/application-commands#registering-a-command
    if count <= GUILD_COMMAND_PERMISSION_LIMIT {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::PermissionsCountInvalid,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::non_ascii_literal)]

    use super::*;
    use twilight_model::{application::command::CommandType, id::Id};

    // This tests [`description`] and [`name`] by proxy.
    #[test]
    fn test_command() {
        let valid_command = Command {
            application_id: Some(Id::new(1)),
            default_permission: None,
            description: "a".repeat(100),
            guild_id: Some(Id::new(2)),
            id: Some(Id::new(3)),
            kind: CommandType::ChatInput,
            name: "b".repeat(32),
            options: Vec::new(),
            version: Id::new(4),
        };

        assert!(command(&valid_command).is_ok());

        let invalid_command = Command {
            description: "c".repeat(101),
            name: "d".repeat(33),
            ..valid_command
        };

        assert!(command(&invalid_command).is_err());
    }

    #[test]
    fn test_name_characters() {
        assert!(name_characters("hello-command").is_ok()); // Latin language
        assert!(name_characters("Hello").is_err()); // Latin language with uppercase
        assert!(name_characters("hello!").is_err()); // Latin language with non-alphanumeric

        assert!(name_characters("здрасти").is_ok()); // Russian
        assert!(name_characters("Здрасти").is_err()); // Russian with uppercase
        assert!(name_characters("здрасти!").is_err()); // Russian with non-alphanumeric

        assert!(name_characters("你好").is_ok()); // Chinese (no upper and lowercase variants)
        assert!(name_characters("你好。").is_err()); // Chinese with non-alphanumeric
    }

    #[test]
    fn test_guild_permissions() {
        assert!(guild_permissions(0).is_ok());
        assert!(guild_permissions(1).is_ok());
        assert!(guild_permissions(10).is_ok());

        assert!(guild_permissions(11).is_err());
    }
}
