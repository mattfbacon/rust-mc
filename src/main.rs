use std::sync::{Arc, RwLock};

mod client;
mod config;
mod logging;
mod packets;
mod server;
mod world;

fn main() -> anyhow::Result<()> {
	let config = config::load()?;
	let _ = logging::init(&config.logging);
	let config = Arc::new(RwLock::new(config));
	server::Server::new(config)?.listen()
}
