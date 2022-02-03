use crate::packets::helpers::PrefixedString;
use crate::packets::varint::VarInt;
use encde::Decode;

#[derive(Decode, Debug)]
pub struct Handshake {
	protocol_version: VarInt,
	server_addr: PrefixedString,
	server_port: u16,
	/// status or login
	next_state: super::super::ProtocolState,
}

#[derive(Decode, Debug)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	Handshake(Handshake),
}
