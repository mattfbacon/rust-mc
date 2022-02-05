use super::{Client, EncryptedClient};
use crate::packets::cipher::{Cipher, CipherWrapper};
use crate::packets::helpers::{PrefixedArray, PrefixedBorrowedBytes, PrefixedBytes, PrefixedString};
use crate::packets::login::receive::{self, Packet as Receive};
use crate::packets::login::send::Packet as Send;
use log::{debug, trace};
use serde::{Deserialize, Deserializer};
use sha::utils::{Digest, DigestExt};

fn deserialize_skin_texture<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
	use serde::de::{self, Error};
	use std::fmt;
	#[derive(Deserialize)]
	struct Property {
		name: String,
		value: String,
		signature: String,
	}

	struct Visitor;
	impl<'de> de::Visitor<'de> for Visitor {
		type Value = Vec<u8>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("A sequence of session properties including the `textures` property")
		}
		fn visit_seq<S: de::SeqAccess<'de>>(self, mut access: S) -> Result<Self::Value, S::Error>
		where
			S::Error: Error,
		{
			while let Some(property) = access.next_element::<Property>()? {
				if property.name == "textures" {
					// TODO check signature; I can't find the Yggdrasil public key anywhere
					return base64::decode(property.value).map_err(|err| S::Error::custom(err.to_string()));
				}
			}
			Err(S::Error::custom("No `textures` property present"))
		}
	}
	deserializer.deserialize_seq(Visitor)
}

#[derive(Deserialize)]
pub struct SessionResponse {
	#[serde(rename = "id")]
	pub uuid: uuid::Uuid,
	#[serde(rename = "name")]
	pub username: String,
	#[serde(deserialize_with = "deserialize_skin_texture")]
	#[serde(rename = "properties")]
	pub skin_texture: Vec<u8>,
}

fn format_minecraft_sha1(mut data: &mut [u8]) -> String {
	use std::fmt::Write;
	let negative = (data[0] & 0x80) == 0x80;
	let mut ret = String::with_capacity((data.len() * 2) + 1);
	if negative {
		ret += "-";
		// two's complement
		let mut carry = true;
		for byte in data.iter_mut().rev() {
			*byte = !*byte;
			if carry {
				// destructuring assignments are unstable
				let result = byte.overflowing_add(1);
				*byte = result.0;
				carry = result.1;
			}
		}
	}
	let first_nonzero = data.iter().position(|&x| x != 0).unwrap_or(data.len());
	data = &mut data[first_nonzero..];
	write!(ret, "{:x}", data[0]).unwrap();
	for byte in data.iter().skip(1) {
		write!(ret, "{:02x}", byte).unwrap();
	}
	ret
}

#[cfg(test)]
mod test {
	use sha::utils::{Digest, DigestExt};
	pub fn format_sha(plain: &str) -> String {
		super::format_minecraft_sha1(&mut sha::sha1::Sha1::default().digest(plain.as_bytes()).to_bytes())
	}
	#[test]
	pub fn sha1_formatting() {
		assert_eq!("4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48", format_sha("Notch"));
		assert_eq!("-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1", format_sha("jeb_"));
		assert_eq!("88e16a1019277b15d58faf0541e11910eb756f6", format_sha("simon"));
	}
}

impl Client {
	fn receive_login_start(&mut self) -> anyhow::Result<String> {
		match self.receive_packet()? {
			None => anyhow::bail!("Client closed connection"),
			Some(Receive::LoginStart { username }) => Ok(username.0),
			Some(other) => anyhow::bail!("Expected Login Start packet but received {:?}", other),
		}
	}
	fn receive_encryption_response(&mut self) -> anyhow::Result<receive::Encryption> {
		match self.receive_packet()? {
			None => anyhow::bail!("Client closed connection"),
			Some(Receive::Encryption(info)) => Ok(info),
			Some(other) => anyhow::bail!("Expected Encryption Response packet but received {:?}", other),
		}
	}
	fn request_encryption(&mut self) -> anyhow::Result<[u8; 4]> {
		debug!("Requesting encryption");
		let verify_token = rand::random();
		self.send_packet(&Send::Encryption {
			server_id: PrefixedString(String::new()),
			public_key: PrefixedBorrowedBytes(&self.global_state.rsa_public_der),
			verify_token: PrefixedArray(verify_token),
		})?;
		Ok(verify_token)
	}
	fn receive_shared_secret(&mut self, verify_token: [u8; 4]) -> anyhow::Result<Vec<u8>> {
		debug!("Receiving encryption response");
		let receive::Encryption {
			shared_secret: PrefixedBytes(encrypted_shared_secret),
			verify_token: PrefixedBytes(encrypted_verify_token),
		} = self.receive_encryption_response()?;
		debug!("Received encryption response");
		let decrypted_verify_token = self.global_state.rsa_key.decrypt(rsa::PaddingScheme::PKCS1v15Encrypt, &encrypted_verify_token)?;
		anyhow::ensure!(verify_token == decrypted_verify_token.as_slice(), "Verify token does not match");
		Ok(self.global_state.rsa_key.decrypt(rsa::PaddingScheme::PKCS1v15Encrypt, &encrypted_shared_secret)?)
	}
	fn make_cipher(shared_secret: &[u8]) -> anyhow::Result<Cipher> {
		use cfb8::cipher::NewCipher;
		Ok(Cipher::new_from_slices(shared_secret, shared_secret)?)
	}
	fn get_session(&mut self, username: String, shared_secret: &[u8]) -> anyhow::Result<SessionResponse> {
		let mut auth_hash = sha::sha1::Sha1::default().digest(shared_secret).digest(&self.global_state.rsa_public_der).to_bytes();
		let auth_hash = format_minecraft_sha1(&mut auth_hash);
		debug!("Making request to session server");
		Ok(reqwest::blocking::get(reqwest::Url::parse_with_params(
			"https://sessionserver.mojang.com/session/minecraft/hasJoined",
			&[("username", username), ("serverId", auth_hash), ("ip", self.address.to_string())],
		)?)?
		.json()?)
	}

	pub(super) fn handle_login(mut self) -> anyhow::Result<()> {
		trace!("Entering login state");
		let username = self.receive_login_start()?;
		trace!("Connection username: {}", username);
		let verify_token = self.request_encryption()?;
		let shared_secret = self.receive_shared_secret(verify_token)?;
		let session_response = self.get_session(username, &shared_secret)?;
		let cipher = Self::make_cipher(&shared_secret)?;
		let wrapper = CipherWrapper::new(self.socket, cipher);
		EncryptedClient {
			socket: wrapper,
			address: self.address,
			config: self.config,
			global_state: self.global_state,
		}
		.enter_play(session_response)
	}
}
