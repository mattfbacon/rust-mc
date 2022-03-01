use crate::client::Client;
use crate::config::Config;
use anyhow::Result;
use log::{debug, info};
use openssl::rsa::Rsa;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};

pub const PROTOCOL_VERSION: i32 = 757;
pub const SERVER_VERSION: &str = "1.18.1";

pub struct Server {
	config: Arc<RwLock<Config>>,
	global_state: Arc<RwLock<GlobalState>>,
}

impl Server {
	pub fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
		Ok(Self {
			config,
			global_state: Arc::new(RwLock::new(GlobalState::new()?)),
		})
	}
	pub fn listen(self) -> Result<()> {
		let Config { address, port, .. } = *self.config.read().unwrap();
		let listener = TcpListener::bind((address, port))?;
		info!("Listening on {}:{}", address, port);
		loop {
			let (socket, client_address) = listener.accept()?;
			let client = Client::new(socket, client_address, self.config.clone(), self.global_state.clone());
			std::thread::Builder::new().name(client_address.to_string()).spawn(move || {
				if let Err(err) = client.handle() {
					log::error!("{:#}", err);
				}
			})?;
		}
	}
}

pub struct GlobalState {
	pub rsa_key: Rsa<openssl::pkey::Private>,
	pub rsa_public_der: Vec<u8>,
}

impl GlobalState {
	fn new() -> Result<Self> {
		debug!("Generating RSA key");
		let rsa_key = Rsa::generate(1024)?;
		debug!("Finished generating RSA key");
		let rsa_public_der = rsa_key.public_key_to_der()?;
		Ok(Self { rsa_public_der, rsa_key })
	}
}
