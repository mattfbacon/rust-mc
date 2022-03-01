use crate::config::Config;
use crate::packets::handshake::receive::Packet as HandshakeReceive;
use crate::packets::helpers::varint::VarInt;
use crate::packets::ProtocolState;
use crate::server::{GlobalState, PROTOCOL_VERSION};
use encde::{Decode, DecodeSized, Encode};
use log::trace;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, RwLock};

mod login;
mod play;
mod status;

trait Backing: Read + Write + Send {}
impl<T: Read + Write + Send> Backing for T {}

struct Socket {
	backing: Box<dyn Backing>,
}

impl Socket {
	pub fn wrap<W: Backing + 'static>(&mut self, make_wrapper: impl FnOnce(Box<dyn Backing>) -> W) {
		// SAFETY: `std::ptr::read` can create two instances that point to the same memory (resulting in double Drops), but `std::ptr::write` does not drop the old value—instead the old value is replaced before it could be erroneously dropped.
		unsafe { std::ptr::write(&mut self.backing, Box::new(make_wrapper(std::ptr::read(&mut self.backing)))) }
	}
}

pub struct Client {
	socket: Socket,
	config: Arc<RwLock<Config>>,
	global_state: Arc<RwLock<GlobalState>>,
}

impl Socket {
	fn receive_packet<P: DecodeSized>(&mut self) -> encde::Result<Option<P>> {
		let packet_len = match VarInt::decode(&mut self.backing) {
			Ok(VarInt(packet_len)) => packet_len.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?,
			Err(encde::Error::Io(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
			Err(err) => return Err(err),
		};
		let mut packet_data = vec![0u8; packet_len];
		self.backing.read_exact(&mut packet_data)?;
		encde::util::decode_from_entire_slice(&packet_data).map(Some)
	}
	fn send_packet<P: Encode>(&mut self, data: &P) -> encde::Result<()> {
		let packet_data = encde::util::encode_to_vec(data)?;
		VarInt(packet_data.len().try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(&mut self.backing)?;
		self.backing.write_all(&packet_data)?;
		Ok(())
	}
}

impl Client {
	pub fn new(socket: TcpStream, address: SocketAddr, config: Arc<RwLock<Config>>, global_state: Arc<RwLock<GlobalState>>) -> Self {
		trace!("New connection from {}", &address);
		Self {
			socket: Socket { backing: Box::new(socket) },
			config,
			global_state,
		}
	}
	pub fn handle(mut self) -> anyhow::Result<()> {
		let HandshakeReceive::Handshake(handshake) = self.socket.receive_packet()?.ok_or_else(|| anyhow::anyhow!("Client closed connection").context("Handshake"))?;
		trace!("Client handshake: {:?}", handshake);
		match handshake.next_state {
			ProtocolState::Login => {
				anyhow::ensure!(
					handshake.protocol_version.0 == PROTOCOL_VERSION,
					"Client protocol version ({}) does not match ours ({}); disconnecting",
					handshake.protocol_version.0,
					PROTOCOL_VERSION
				);
				self.handle_login().map_err(|err| err.context("Login"))
			}
			ProtocolState::Status => self.handle_status().map_err(|err| err.context("Status")),
			unacceptable => {
				anyhow::bail!("Client requested an unacceptable next state ({:?}) in handshake", unacceptable);
			}
		}
	}
}
