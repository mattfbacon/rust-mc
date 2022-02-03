use crate::packets::helpers::{Chat, PrefixedBytes, PrefixedString, Uuid};
use crate::packets::varint::VarInt;
use encde::Encode;

#[derive(Encode)]
pub struct Disconnect {
	reason: Chat,
}

#[derive(Encode)]
pub struct Encryption {
	server_id: PrefixedString,
	public_key: PrefixedBytes,
	verify_token: PrefixedBytes,
}

#[derive(Encode)]
pub struct LoginSuccess {
	uuid: Uuid,
	username: PrefixedString,
}

/// This packet is optional and not sending it means to not compress.
/// Thus, we don't need to send it until we've implemented compression.
#[derive(Encode)]
pub struct SetCompression {
	/// Maximum packet size before it's compressed. If <= 0, disable compression.
	threshold: VarInt,
}

// No LoginPluginRequest for now

#[derive(Encode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	Disconnect(Disconnect),
	#[encde(wire_tag = 1)]
	Encryption(Encryption),
	#[encde(wire_tag = 2)]
	LoginSuccess(LoginSuccess),
	#[encde(wire_tag = 3)]
	SetCompression(SetCompression),
	// #[encde(wire_tag = 4)]
	// LoginPluginRequest(LoginPluginRequest),
}
