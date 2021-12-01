use crate::{
    application::{command::CommandOptionChoice, component::Component},
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
};
use serde::{Deserialize, Serialize};

use super::{InteractionResponse, ResponseType};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum CallbackDataEnvelope {
    Autocomplete(Autocomplete),
    Messages(CallbackData),
    Modal(ModalData),
}

#[derive(Debug, Deserialize)]
pub(super) struct CallbackDataHolder {
    allowed_mentions: Option<AllowedMentions>,
    components: Option<Vec<Component>>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    flags: Option<MessageFlags>,
    tts: Option<bool>,
    choices: Option<Vec<CommandOptionChoice>>,
    custom_id: Option<String>,
    title: Option<String>,
}

impl CallbackDataHolder {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_response(self, kind: ResponseType) -> Result<InteractionResponse, String> {
        match kind {
            ResponseType::Pong | ResponseType::DeferredUpdateMessage => Err(self.to_error(kind)),
            ResponseType::ChannelMessageWithSource => Ok(
                InteractionResponse::ChannelMessageWithSource(self.to_callback_data()),
            ),
            ResponseType::DeferredChannelMessageWithSource => Ok(
                InteractionResponse::DeferredChannelMessageWithSource(self.to_callback_data()),
            ),
            ResponseType::UpdateMessage => {
                Ok(InteractionResponse::UpdateMessage(self.to_callback_data()))
            }
            ResponseType::ApplicationCommandAutocompleteResult => {
                Ok(InteractionResponse::Autocomplete(self.to_auto_complete()?))
            }
            ResponseType::Modal => Ok(InteractionResponse::Modal(self.to_modal()?)),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_callback_data(self) -> CallbackData {
        CallbackData {
            allowed_mentions: self.allowed_mentions,
            components: self.components,
            content: self.content,
            embeds: self.embeds.unwrap_or_default(),
            flags: self.flags,
            tts: self.tts,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_auto_complete(self) -> Result<Autocomplete, String> {
        if self.choices.is_none() {
            return Err(self.to_error(ResponseType::ApplicationCommandAutocompleteResult));
        }

        Ok(Autocomplete {
            choices: self.choices.unwrap(),
        })
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_modal(self) -> Result<ModalData, String> {
        // let custom_id = self.custom_id.ok_or_else(|| self.to_error(ResponseType::Modal))?;
        // let title = self.title.ok_or_else(|| self.to_error(ResponseType::Modal))?;
        // let components = self.components.ok_or_else(|| self.to_error(ResponseType::Modal))?;

        if self.custom_id.is_none() | self.title.is_none() | self.components.is_none() {
            return Err(self.to_error(ResponseType::Modal));
        }

        Ok(ModalData {
            custom_id: self.custom_id.unwrap(),
            title: self.title.unwrap(),
            components: self.components.unwrap(),
        })
    }

    fn to_error(&self, kind: ResponseType) -> String {
        format!(
            "unknown type/data combination: type={:?} data={:?}",
            kind, self
        )
    }
}

/// Optional extra data sent when responding to an [`Interaction`] of type
/// [`ApplicationCommand`].
///
/// This is used when intending to send a message in the response.
///
/// This struct has an [associated builder] in the [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/builder/struct.CallbackDataBuilder.html
///
/// [`Interaction`]: crate::application::interaction::Interaction
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of components to include in the callback response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

/// Response to an autocomplete [`Interaction`].
///
/// [`Interaction`]: crate::application::interaction::Interaction
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Autocomplete {
    /// List of autocomplete alternatives.
    pub choices: Vec<CommandOptionChoice>,
}

/// Modal response to an [`Interaction::ApplicationCommand`] or [`Interaction::MessageComponent`].
///
/// This is used when intending to prompt the user for a modal.
///
/// [`Interaction::ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
/// [`Interaction::MessageComponent`]: crate::application::interaction::Interaction::MessageComponent
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ModalData {
    /// User defined identifier for the modal.
    pub custom_id: String,
    /// The title of the modal.
    pub title: String,
    /// List of components to include in the modal.
    ///
    /// This field only allows following [`Component`]s:
    ///
    /// - [`Component::InputText`]
    pub components: Vec<Component>,
}

#[cfg(test)]
mod tests {
    use super::{Autocomplete, CallbackData};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        CallbackData: allowed_mentions,
        components,
        content,
        embeds,
        flags,
        tts
    );
    assert_impl_all!(
        CallbackData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    assert_fields!(Autocomplete: choices);

    assert_impl_all!(
        Autocomplete: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
}
