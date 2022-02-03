use figment::{
	providers::{Env, Format, Toml},
	Error, Figment,
};
use serde::Deserialize;

fn default_port() -> u16 {
	25565
}

#[derive(Deserialize)]
pub struct Config {
	pub address: std::net::IpAddr,
	#[serde(default = "default_port")]
	pub port: u16,
}

pub fn load() -> Result<Config, Error> {
	let raw = Figment::new().merge(Toml::file("server.toml")).merge(Env::prefixed("RUSTMC_"));
	raw.extract()
}
