use crate::packets::helpers::{PrefixedBytes, PrefixedString};
use encde::Decode;

#[derive(Decode)]
struct LoginStart {
	username: PrefixedString,
}

#[derive(Decode)]
struct Encryption {
	shared_secret: PrefixedBytes,
	verify_token: PrefixedBytes,
}

// TODO LoginPluginResponse

#[derive(Decode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	LoginStart(LoginStart),
	#[encde(wire_tag = 1)]
	Encryption(Encryption),
	// #[encde(wire_tag = 2)]
	// LoginPluginResponse(LoginPluginResponse),
}
