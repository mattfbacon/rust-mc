use super::EncryptedClient;
use crate::packets::helpers::{chat, Json, PrefixedString, Uuid as UuidWrapper};
use crate::packets::play::{receive::Packet as Receive, send::Packet as Send};
use log::debug;

impl EncryptedClient {
	pub(super) fn enter_play(mut self, session: super::login::SessionResponse) -> anyhow::Result<()> {
		debug!("Sending login success packet");
		let packet = crate::packets::login::send::Packet::LoginSuccess {
			uuid: UuidWrapper(session.uuid),
			username: PrefixedString(session.username),
		};
		self.send_packet(&packet)?;
		self.handle_play()
	}
}

impl EncryptedClient {
	fn handle_play(mut self) -> anyhow::Result<()> {
		debug!("Entering play state");
		self.send_packet(&Send::Disconnect {
			reason: Json(chat::Content {
				text: "hello".to_owned(),
				..Default::default()
			}),
		})?;
		Ok(())
	}
}
