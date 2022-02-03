//! Utility functions related to encoding and decoding

use crate::{Decode, Encode, Error};
use std::{cmp::min, io};

const EMPTY_SLICE: [u8; 1024] = [0u8; 1024];
/// Write `amount` zero bytes to `writer`
pub fn write_padding(writer: &mut dyn io::Write, mut amount: usize) -> io::Result<()> {
	while amount > EMPTY_SLICE.len() {
		writer.write_all(&EMPTY_SLICE)?;
		amount -= EMPTY_SLICE.len();
	}
	writer.write_all(&EMPTY_SLICE[0..amount])?;
	Ok(())
}

static mut EMPTY_MUT_SLICE: [u8; 1024] = [0u8; 1024];
/// Read and discard `amount` bytes from `reader`
pub fn read_padding(reader: &mut dyn io::Read, mut amount: usize) -> io::Result<()> {
	while amount > 0 {
		// Mutable statics are unsafe due to race conditions between threads, but we're using it as a bitbucket, so it doesn't matter
		amount -= unsafe { reader.read(&mut EMPTY_MUT_SLICE[0..min(amount, EMPTY_MUT_SLICE.len())])? }
	}
	Ok(())
}

/// A wrapper around a [Vec]<u8> that can be [Write]n to
///
/// [Vec]: std::collections::Vec
/// [Write]: std::io::Write
#[derive(Default, Debug)]
pub struct VecWriter(Vec<u8>);
impl VecWriter {
	/// Create a new instance with an empty vector
	pub fn new() -> Self {
		Self::default()
	}
	/// Create a new instance with an empty vector of the given capacity
	pub fn with_capacity(cap: usize) -> Self {
		Self(Vec::with_capacity(cap))
	}
	/// Consume the VecWriter and return the internal vector
	pub fn into_inner(self) -> Vec<u8> {
		self.0
	}
}
impl std::io::Write for VecWriter {
	fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
		self.0.extend(data);
		Ok(data.len())
	}
	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}
/// Encode data into a Vec<u8> and return it
pub fn encode_to_vec(item: &dyn Encode) -> crate::Result<Vec<u8>> {
	let mut writer = VecWriter::new();
	item.encode(&mut writer)?;
	Ok(writer.0)
}

/// A wrapper around a byte slice that can be [Read] from
///
/// [Read]: std::io::Read
pub struct SliceReader<'a>(&'a [u8]);
impl<'a> SliceReader<'a> {
	/// Create a new instance with the provided slice as the backing data
	pub fn new<'b: 'a>(slice: &'b [u8]) -> Self {
		Self(slice)
	}
	/// Return the number of bytes remaining in the backing data
	pub fn remaining(&self) -> usize {
		self.0.len()
	}
}
impl std::io::Read for SliceReader<'_> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let can_read_amount = std::cmp::min(buf.len(), self.0.len());
		buf[..can_read_amount].clone_from_slice(&self.0[..can_read_amount]);
		self.0 = &self.0[can_read_amount..];
		Ok(can_read_amount)
	}
}
/// Decode data from a &[u8] and return the data, along with the number of bytes remaining
/// If the slice is too short, an Err variant will be returned
pub fn decode_from_slice<T: Decode>(data: &[u8]) -> crate::Result<(T, usize)> {
	let mut reader = SliceReader::new(data);
	let ret = T::decode(&mut reader)?;
	Ok((ret, reader.remaining()))
}

/// Equivalent to `decode_from_slice`, except that an Err variant is returned if the entire slice was not used by `T`'s `decode` implementation.
pub fn decode_from_entire_slice<T: Decode>(data: &[u8]) -> crate::Result<T> {
	let (decoded, amount_left): (T, usize) = decode_from_slice(data)?;
	if amount_left != 0 {
		Err(Error::UnexpectedLength {
			expected: data.len() - amount_left,
			actual: data.len(),
		})
	} else {
		Ok(decoded)
	}
}
