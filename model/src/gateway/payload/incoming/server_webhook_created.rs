//! The TeamWebhookCreated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::webhook::Webhook;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerWebhookCreated {
    pub server_id: Id<ServerMarker>,
    pub webhook: Webhook,
}
