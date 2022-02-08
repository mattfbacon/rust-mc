pub struct Json<T>(pub T);

impl<T: Serialize> Encode for Json<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded = serde_json::to_string(&self.0).map_err(|err| encde::Error::Custom(Box::new(err)))?;
		PrefixedString(encoded).encode(writer)
	}
}

impl<T: Deserialize> Decode for Json<T> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		Ok(Self(serde_json::from_reader(reader).map_err(|err| encde::Error::Custom(Box::new(err)))?))
	}
}
