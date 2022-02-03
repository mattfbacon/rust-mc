use encde::Decode;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

mod config;
mod packets;

fn main() -> Result<(), Box<dyn Error>> {
	let config = config::load().map_err(|err| Box::new(err))?;
	let listener = TcpListener::bind((config.address, config.port))?;
	loop {
		let (socket, client_address) = listener.accept()?;
		std::thread::spawn(move || accept_client(socket, client_address));
	}
}

fn accept_client(mut socket: TcpStream, _address: SocketAddr) {
	loop {
		let packet_len = packets::varint::VarInt::decode(&mut socket).unwrap().0 as usize;
		let mut packet_data = vec![0u8; packet_len];
		socket.read_exact(&mut packet_data).unwrap();
		eprintln!("raw data: {:?}", packet_data);
		let packet: packets::handshake::receive::Packet = encde::util::decode_from_entire_slice(&packet_data).unwrap();
		eprintln!("packet {:?}", packet);
	}
}
