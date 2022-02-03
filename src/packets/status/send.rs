use crate::packets::helpers::PrefixedString;
use encde::Encode;

#[derive(Encode)]
pub struct Response {
	/// TODO more structured data type (this should be JSON)
	data: PrefixedString,
}

#[derive(Encode)]
pub struct Pong {
	echo: i64,
}

#[derive(Encode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	Response(Response),
	#[encde(wire_tag = 1)]
	Pong(Pong),
}
