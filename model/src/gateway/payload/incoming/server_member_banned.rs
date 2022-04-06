//! The TeamMemberBanned event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::server::member::ban::ServerMemberBan;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberBanned {
    server_member_ban: ServerMemberBan,
    server_id: Id<ServerMarker>,
}

impl ServerMemberBanned {
    pub fn server_member_ban(&self) -> ServerMemberBan {
        self.server_member_ban.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }
}