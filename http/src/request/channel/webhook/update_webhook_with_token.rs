use crate::{
    client::Client,
    error::Error,
    request::{NullableField, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::Webhook,
    id::{marker::WebhookMarker, Id},
};

#[derive(Serialize)]
struct UpdateWebhookWithTokenFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<NullableField<&'a str>>,
}

/// Update a webhook, with a token, by ID.
#[must_use = "requests must be configured and executed"]
pub struct UpdateWebhookWithToken<'a> {
    fields: UpdateWebhookWithTokenFields<'a>,
    http: &'a Client,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
}

impl<'a> UpdateWebhookWithToken<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
    ) -> Self {
        Self {
            fields: UpdateWebhookWithTokenFields {
                avatar: None,
                name: None,
            },
            http,
            token,
            webhook_id,
        }
    }

    /// Set the avatar of the webhook.
    ///
    /// See [Discord Docs/Image Data] for more information. This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type and `{data}` is the
    /// base64-encoded image.
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub const fn avatar(mut self, avatar: Option<&'a str>) -> Self {
        self.fields.avatar = Some(NullableField(avatar));

        self
    }

    /// Change the name of the webhook.
    pub const fn name(mut self, name: Option<&'a str>) -> Self {
        self.fields.name = Some(NullableField(name));

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Webhook> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateWebhookWithToken<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateWebhook {
            token: Some(self.token),
            webhook_id: self.webhook_id.get(),
        })
        .use_authorization_token(false);

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
