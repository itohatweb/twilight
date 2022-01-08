//! Used when responding to interactions.

mod callback_data;
mod response_type;

use crate::application::callback::callback_data::CallbackDataHolder;

pub use self::{
    callback_data::Autocomplete, callback_data::CallbackData, callback_data::ModalData,
    response_type::ResponseType,
};

use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

/// Payload used for responding to an interaction.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InteractionResponse {
    /// Used when responding to an interaction of type Ping.
    Pong,
    /// Responds to an interaction with a message.
    ChannelMessageWithSource(CallbackData),
    /// Acknowledges an interaction, showing a loading state.
    DeferredChannelMessageWithSource(CallbackData),
    /// Acknowledge an interaction and edit the original message later.
    ///
    /// This is only valid for components.
    DeferredUpdateMessage,
    /// Edit the message a component is attached to.
    UpdateMessage(CallbackData),
    /// Autocomplete results.
    Autocomplete(Autocomplete),
    /// Respond with a modal for the user to fill out.
    Modal(ModalData),
}

impl InteractionResponse {
    /// Type of response this is.
    ///
    /// # Examples
    ///
    /// Check the types of the [`DeferredUpdateMessage`] and [`Pong`]
    /// interaction response variants.
    ///
    /// ```
    /// use twilight_model::application::callback::{
    ///     InteractionResponse,
    ///     ResponseType,
    /// };
    ///
    /// assert_eq!(
    ///     ResponseType::DeferredUpdateMessage,
    ///     InteractionResponse::DeferredUpdateMessage.kind(),
    /// );
    /// assert_eq!(ResponseType::Pong, InteractionResponse::Pong.kind());
    /// ```
    ///
    /// [`DeferredUpdateMessage`]: Self::DeferredUpdateMessage
    /// [`Pong`]: Self::Pong
    pub const fn kind(&self) -> ResponseType {
        match self {
            Self::Autocomplete(_) => ResponseType::ApplicationCommandAutocompleteResult,
            Self::Pong => ResponseType::Pong,
            Self::ChannelMessageWithSource(_) => ResponseType::ChannelMessageWithSource,
            Self::DeferredChannelMessageWithSource(_) => {
                ResponseType::DeferredChannelMessageWithSource
            }
            Self::DeferredUpdateMessage => ResponseType::DeferredUpdateMessage,
            Self::UpdateMessage(_) => ResponseType::UpdateMessage,
            Self::Modal(_) => ResponseType::Modal,
        }
    }
}

impl<'de> Deserialize<'de> for InteractionResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ResponseVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ResponseField {
    Data,
    Type,
}

struct ResponseVisitor;

impl<'de> Visitor<'de> for ResponseVisitor {
    type Value = InteractionResponse;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct InteractionResponse")
    }

    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut data: Option<CallbackDataHolder> = None;
        let mut kind: Option<ResponseType> = None;

        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("deserializing interaction response");
        #[cfg(feature = "tracing")]
        let _span_enter = span.enter();

        loop {
            #[cfg(feature = "tracing")]
            let span_child = tracing::trace_span!("iterating over interaction response");
            #[cfg(feature = "tracing")]
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                #[cfg(feature = "tracing")]
                Err(why) => {
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
                #[cfg(not(feature = "tracing"))]
                Err(_) => {
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
            };

            match key {
                ResponseField::Data => {
                    if data.is_some() {
                        return Err(DeError::duplicate_field("data"));
                    }

                    data = Some(map.next_value()?);
                }
                ResponseField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    kind = Some(map.next_value()?);
                }
            }
        }

        let kind = kind.ok_or_else(|| DeError::missing_field("type"))?;
        let data = data
            // .ok_or_else(|| DeError::missing_field("data"))?
            .map(|holder| holder.to_response(kind));
        // .to_response(kind)
        // .map_err(|err| DeError::custom(err))?;

        Ok(match (kind, data) {
            (ResponseType::Pong, _) => Self::Value::Pong,
            (
                ResponseType::ChannelMessageWithSource,
                Some(Ok(Self::Value::ChannelMessageWithSource(data))),
            ) => Self::Value::ChannelMessageWithSource(data),
            (
                ResponseType::DeferredChannelMessageWithSource,
                Some(Ok(Self::Value::DeferredChannelMessageWithSource(data))),
            ) => Self::Value::DeferredChannelMessageWithSource(data),
            (ResponseType::DeferredUpdateMessage, _) => Self::Value::DeferredUpdateMessage,
            (ResponseType::UpdateMessage, Some(Ok(Self::Value::UpdateMessage(data)))) => {
                Self::Value::UpdateMessage(data)
            }
            (
                ResponseType::ApplicationCommandAutocompleteResult,
                Some(Ok(Self::Value::Autocomplete(data))),
            ) => Self::Value::Autocomplete(data),
            (ResponseType::Modal, Some(Ok(Self::Value::Modal(data)))) => Self::Value::Modal(data),
            (t, d) => {
                return Err(DeError::custom(format!(
                    "unknown type/data combination: type={:?} data={:?}",
                    t, d
                )))
            }
        })
    }
}

impl Serialize for InteractionResponse {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Autocomplete(data) => {
                let mut state = serializer.serialize_struct("InteractionResponse", 2)?;

                state.serialize_field("type", &self.kind())?;
                state.serialize_field("data", &data)?;

                state.end()
            }
            Self::Pong | Self::DeferredUpdateMessage => {
                let mut state = serializer.serialize_struct("InteractionResponse", 1)?;

                state.serialize_field("type", &self.kind())?;

                state.end()
            }
            Self::ChannelMessageWithSource(data)
            | Self::DeferredChannelMessageWithSource(data)
            | Self::UpdateMessage(data) => {
                let mut state = serializer.serialize_struct("InteractionResponse", 2)?;

                state.serialize_field("type", &self.kind())?;
                state.serialize_field("data", &data)?;

                state.end()
            }
            Self::Modal(data) => {
                let mut state = serializer.serialize_struct("InteractionResponse", 2)?;

                state.serialize_field("type", &self.kind())?;
                state.serialize_field("data", &data)?;

                state.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Autocomplete, CallbackData, InteractionResponse, ModalData};
    use crate::{
        application::{
            callback::ResponseType,
            command::CommandOptionChoice,
            component::{
                input_text::InputTextStyle, ActionRow, Component, ComponentType, InputText,
            },
        },
        channel::message::MessageFlags,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        InteractionResponse: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_kind() {
        let callback = CallbackData {
            allowed_mentions: None,
            content: None,
            components: None,
            embeds: Vec::new(),
            flags: None,
            tts: None,
        };

        let autocomplete = Autocomplete {
            choices: Vec::new(),
        };

        let modal = ModalData {
            custom_id: "modal-1".to_owned(),
            title: "test".to_owned(),
            components: Vec::new(),
        };

        assert_eq!(
            InteractionResponse::Autocomplete(autocomplete).kind(),
            ResponseType::ApplicationCommandAutocompleteResult
        );
        assert_eq!(InteractionResponse::Pong.kind(), ResponseType::Pong);
        assert_eq!(
            InteractionResponse::ChannelMessageWithSource(callback.clone()).kind(),
            ResponseType::ChannelMessageWithSource
        );
        assert_eq!(
            InteractionResponse::DeferredChannelMessageWithSource(callback.clone()).kind(),
            ResponseType::DeferredChannelMessageWithSource
        );
        assert_eq!(
            InteractionResponse::DeferredUpdateMessage.kind(),
            ResponseType::DeferredUpdateMessage
        );
        assert_eq!(
            InteractionResponse::UpdateMessage(callback).kind(),
            ResponseType::UpdateMessage
        );
        assert_eq!(
            InteractionResponse::Modal(modal).kind(),
            ResponseType::Modal
        );
    }

    #[test]
    fn test_autocomplete() {
        let value = InteractionResponse::Autocomplete(Autocomplete {
            choices: Vec::from([CommandOptionChoice::String {
                name: "Twilight".to_owned(),
                value: "twilight".to_owned(),
            }]),
        });

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::String("type"),
                Token::U8(ResponseType::ApplicationCommandAutocompleteResult as u8),
                Token::String("data"),
                Token::Struct {
                    name: "Autocomplete",
                    len: 1,
                },
                Token::String("choices"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoice",
                    len: 2,
                },
                Token::String("name"),
                Token::String("Twilight"),
                Token::String("value"),
                Token::String("twilight"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::String("type"),
                Token::U8(ResponseType::ApplicationCommandAutocompleteResult as u8),
                Token::String("data"),
                Token::Struct {
                    name: "CallbackDataHolder",
                    len: 1,
                },
                Token::String("choices"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoice",
                    len: 2,
                },
                Token::String("name"),
                Token::String("Twilight"),
                Token::String("value"),
                Token::String("twilight"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_pong() {
        let value = InteractionResponse::Pong;

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 1,
                },
                Token::String("type"),
                Token::U8(ResponseType::Pong as u8),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_deferred_update_message() {
        let value = InteractionResponse::DeferredUpdateMessage;

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 1,
                },
                Token::String("type"),
                Token::U8(ResponseType::DeferredUpdateMessage as u8),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_channel_message_with_source() {
        let value = InteractionResponse::ChannelMessageWithSource(CallbackData {
            allowed_mentions: None,
            content: Some("test".into()),
            components: None,
            embeds: None,
            flags: Some(MessageFlags::EPHEMERAL),
            tts: None,
        });

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(ResponseType::ChannelMessageWithSource as u8),
                Token::Str("data"),
                Token::Struct {
                    name: "CallbackData",
                    len: 2,
                },
                Token::Str("content"),
                Token::Some,
                Token::Str("test"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(MessageFlags::EPHEMERAL.bits()),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(ResponseType::ChannelMessageWithSource as u8),
                Token::Str("data"),
                Token::Struct {
                    name: "CallbackDataHolder",
                    len: 2,
                },
                Token::Str("content"),
                Token::Some,
                Token::Str("test"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(MessageFlags::EPHEMERAL.bits()),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_modal() {
        let value = InteractionResponse::Modal(ModalData {
            custom_id: "modal-1".to_owned(),
            title: "Test".to_owned(),
            components: Vec::from([Component::ActionRow(ActionRow {
                components: Vec::from([Component::InputText(InputText {
                    custom_id: "input-1".to_owned(),
                    label: "Test".to_owned(),
                    style: InputTextStyle::Short,
                    placeholder: None,
                    max_length: None,
                    min_length: None,
                    required: None,
                    value: None,
                })]),
            })]),
        });

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(ResponseType::Modal as u8),
                Token::String("data"),
                Token::Struct {
                    name: "ModalData",
                    len: 3,
                },
                Token::String("custom_id"),
                Token::String("modal-1"),
                Token::String("title"),
                Token::String("Test"),
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ActionRow",
                    len: 2,
                },
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "InputText",
                    len: 4,
                },
                Token::String("type"),
                Token::U8(ComponentType::InputText as u8),
                Token::String("custom_id"),
                Token::String("input-1"),
                Token::String("label"),
                Token::String("Test"),
                Token::String("style"),
                Token::U8(InputTextStyle::Short as u8),
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("type"),
                Token::U8(ComponentType::ActionRow as u8),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(ResponseType::Modal as u8),
                Token::String("data"),
                Token::Struct {
                    name: "CallbackDataHolder",
                    len: 3,
                },
                Token::String("custom_id"),
                Token::Some,
                Token::String("modal-1"),
                Token::String("title"),
                Token::Some,
                Token::String("Test"),
                Token::String("components"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ActionRow",
                    len: 2,
                },
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "InputText",
                    len: 4,
                },
                Token::String("type"),
                Token::U8(ComponentType::InputText as u8),
                Token::String("custom_id"),
                Token::String("input-1"),
                Token::String("label"),
                Token::String("Test"),
                Token::String("style"),
                Token::U8(InputTextStyle::Short as u8),
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("type"),
                Token::U8(ComponentType::ActionRow as u8),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
