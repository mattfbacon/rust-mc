use super::Client;
use crate::packets::helpers::{chat, Json, PrefixedString, Uuid as UuidWrapper};
use crate::packets::play::{receive::Packet as Receive, send::Packet as Send};
use log::debug;

impl Client {
	pub(super) fn handle_play(mut self) -> anyhow::Result<()> {
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
