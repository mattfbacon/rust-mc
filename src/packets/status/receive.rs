use encde::Decode;

#[derive(Decode)]
pub struct Request {}

#[derive(Decode)]
pub struct Ping {
	echo: i64,
}

#[derive(Decode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	Request(Request),
	#[encde(wire_tag = 1)]
	Ping(Ping),
}
