mod config;
mod logging;
mod packets;
mod server;

fn main() -> anyhow::Result<()> {
	let config: &'static _ = Box::leak(Box::new(config::load()?));
	let _ = logging::init(&config.logging);
	server::Server::new(config)?.listen()
}
