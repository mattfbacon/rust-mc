#![allow(dead_code)]

use super::varint::VarInt;
use bitvec::vec::BitVec;
use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct PrefixedArray<T, const N: usize>(pub [T; N]);
#[derive(Debug)]
pub struct PrefixedBytes(pub Vec<u8>);
#[derive(Debug)]
pub struct PrefixedBorrowedBytes<'a>(pub &'a [u8]);
#[derive(Debug)]
pub struct PrefixedString(pub String);
#[derive(Debug)]
pub struct UnprefixedBytes(pub Vec<u8>);
#[derive(Debug)]
pub struct PrefixedVec<T, SizeType = VarInt>(pub Vec<T>, std::marker::PhantomData<SizeType>)
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error;
#[derive(Debug)]
pub struct PrefixedOption<T>(pub Option<T>);
#[derive(Debug)]
pub struct PrefixedBitVec<T: bitvec::store::BitStore = u64>(pub BitVec<T>);

impl<T, SizeType> PrefixedVec<T, SizeType>
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error,
{
	pub fn new(underlying: Vec<T>) -> Self {
		Self(underlying, std::marker::PhantomData)
	}
}

fn encode_usize_as_varint(writer: &mut dyn Write, val: usize) -> EResult<()> {
	VarInt(val.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(writer)
}
fn encode_encode_slice<T: Encode>(writer: &mut dyn Write, slice: &[T]) -> EResult<()> {
	encode_usize_as_varint(writer, slice.len())?;
	for item in slice.iter() {
		item.encode(writer)?;
	}
	Ok(())
}
fn encode_u8_slice(writer: &mut dyn Write, slice: &[u8]) -> EResult<()> {
	encode_usize_as_varint(writer, slice.len())?;
	writer.write_all(slice)?;
	Ok(())
}

impl<T: Encode, const N: usize> Encode for PrefixedArray<T, N> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_usize_as_varint(writer, N)?;
		self.0.encode(writer)?;
		Ok(())
	}
}

impl<T: Decode, const N: usize> Decode for PrefixedArray<T, N> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = VarInt::decode(reader)?.0.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		if len != N {
			return Err(encde::Error::UnexpectedLength { expected: N, actual: len });
		}
		Ok(Self(<[T; N]>::decode(reader)?))
	}
}

impl Encode for PrefixedString {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_u8_slice(writer, self.0.as_bytes())
	}
}

impl Decode for PrefixedString {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = VarInt::decode(reader)?;
		let len: usize = len.0.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		let mut buffer = vec![0u8; len];
		reader.read_exact(&mut buffer)?;
		Ok(Self(String::from_utf8(buffer).map_err(|err| encde::Error::Custom(Box::new(err)))?))
	}
}

impl Encode for PrefixedBorrowedBytes<'_> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_u8_slice(writer, self.0)
	}
}

impl Encode for PrefixedBytes {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_u8_slice(writer, self.0.as_slice())
	}
}

impl Decode for PrefixedBytes {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = VarInt::decode(reader)?;
		let len: usize = len.0.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		let mut buffer = vec![0u8; len];
		reader.read_exact(&mut buffer)?;
		Ok(Self(buffer))
	}
}

impl Encode for UnprefixedBytes {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		writer.write_all(self.0.as_slice())?;
		Ok(())
	}
}
impl encde::DecodeSized for UnprefixedBytes {
	fn decode_sized(reader: &mut dyn Read, size: usize) -> EResult<Self> {
		let mut ret = vec![0u8; size];
		reader.read_exact(ret.as_mut_slice())?;
		Ok(Self(ret))
	}
}

impl<T: Encode, SizeType> Encode for PrefixedVec<T, SizeType>
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error,
{
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_encode_slice(writer, self.0.as_slice())
	}
}

impl<T: Decode, SizeType> Decode for PrefixedVec<T, SizeType>
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error + Send + Sync,
{
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = SizeType::decode(reader)?;
		let len: usize = len.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		let mut ret = Vec::with_capacity(len);
		for _ in 0..len {
			ret.push(T::decode(reader)?);
		}
		Ok(Self::new(ret))
	}
}

impl<T: Encode> Encode for PrefixedOption<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		self.0.is_some().encode(writer)?;
		match &self.0 {
			Some(inner) => inner.encode(writer)?,
			None => (),
		}
		Ok(())
	}
}

impl<T: Decode> Decode for PrefixedOption<T> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let is_some = bool::decode(reader)?;
		Ok(Self(if is_some { Some(T::decode(reader)?) } else { None }))
	}
}

impl<T: Encode + bitvec::store::BitStore> Encode for PrefixedBitVec<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_encode_slice(writer, self.0.as_raw_slice())
	}
}

impl<T: Decode + bitvec::store::BitStore> Decode for PrefixedBitVec<T> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let raw = PrefixedVec::<_, VarInt>::decode(reader)?.0;
		Ok(Self(BitVec::from_vec(raw)))
	}
}

pub mod chat;
pub use chat::Chat;

pub struct Uuid(pub uuid::Uuid);

impl Encode for Uuid {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		writer.write_all(&self.0.as_u128().to_be_bytes())?;
		Ok(())
	}
}

impl Decode for Uuid {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let mut buf: uuid::Bytes = [0u8; 16];
		reader.read_exact(&mut buf)?;
		// PANICS: from_slice only panics if the buffer is the wrong length and we used the type from the `uuid` crate directly to ensure the correct size.
		Ok(Self(uuid::Uuid::from_slice(&buf).unwrap()))
	}
}

impl std::fmt::Display for Uuid {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.to_hyphenated().fmt(formatter)
	}
}

impl serde::Serialize for Uuid {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}

#[derive(Encode, Decode)]
pub struct UnpackedPosition<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}
pub type F32Position = UnpackedPosition<f32>;
pub type F64Position = UnpackedPosition<f64>;
pub type EntityVelocity = UnpackedPosition<i16>;

// XXX is it better to store the position as packed or unpacked?
pub struct PackedPosition {
	x: i32,
	y: i16,
	z: i32,
}
impl PackedPosition {
	pub fn new(x: i32, y: i16, z: i32) -> Self {
		Self { x, y, z }
	}
}
impl Encode for PackedPosition {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		// 26 MSBs = x
		// 26 middle bits = z
		// 12 LSBs = y
		let out = (((self.x as u32 & 0x3ffffff) as u64) << 38) | (((self.z as u32 & 0x3fffff) as u64) << 12) | ((self.y as u16 & 0xfff) as u64);
		out.encode(writer)
	}
}
impl Decode for PackedPosition {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let raw = u64::decode(reader)?;
		Ok(Self {
			x: (raw >> 38) as i32,
			y: (raw & 0xfff) as i16,
			z: ((raw >> 12) & 0x3ffffff) as i32,
		})
	}
}

/// coordinates are in eighths
/// TODO write constructor once use case is known
#[derive(Encode, Decode)]
pub struct EffectPosition(UnpackedPosition<i32>);

/// The angle is encoded in 256th-turns
#[derive(Encode, Decode)]
pub struct Angle(u8);

impl Angle {
	const UNITS_PER_TURN: f32 = 256.0;
	const DEGREES_PER_TURN: f32 = 360.0;

	pub fn as_degrees(&self) -> f32 {
		f32::from(self.0) * (Self::DEGREES_PER_TURN / Self::UNITS_PER_TURN)
	}
	pub fn from_degrees(degrees: f32) -> Self {
		assert!(degrees.is_finite());
		let ratio = Self::UNITS_PER_TURN / Self::DEGREES_PER_TURN;
		let converted = degrees * ratio;
		let normalized = converted.rem_euclid(Self::UNITS_PER_TURN);
		debug_assert!(normalized.is_finite() && normalized > 0.0 && normalized < Self::UNITS_PER_TURN);
		Self(unsafe { normalized.to_int_unchecked() })
	}
}

#[derive(Encode, Decode)]
pub struct Rotation<T: Encode + Decode> {
	pub pitch: T,
	pub yaw: T,
}

pub type AngleRotation = Rotation<Angle>;
pub type F32Rotation = Rotation<f32>;

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum CardinalDirection {
	North = 2,
	South = 0,
	West = 1,
	East = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum PaintingName {
	Kebab = 0,
	Aztec = 1,
	Alban = 2,
	Aztec2 = 3,
	Bomb = 4,
	Plant = 5,
	Wasteland = 6,
	Pool = 7,
	Courbet = 8,
	Sea = 9,
	Sunset = 10,
	CreeBet = 11,
	Wanderer = 12,
	Graham = 13,
	Match = 14,
	Bust = 15,
	Stage = 16,
	Void = 17,
	SkullAndRoses = 18,
	Wither = 19,
	Fighters = 20,
	Pointer = 21,
	PigScene = 22,
	BurningSkull = 23,
	Skeleton = 24,
	DonkeyKong = 25,
}

pub enum SculkDestination {
	Block(PackedPosition),
	Entity(VarInt),
}

impl Encode for SculkDestination {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		match self {
			Self::Block(position) => {
				encode_u8_slice(writer, "block".as_bytes())?;
				position.encode(writer)
			}
			Self::Entity(entity_id) => {
				encode_u8_slice(writer, "entity".as_bytes())?;
				entity_id.encode(writer)
			}
		}
	}
}

impl Decode for SculkDestination {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let discriminant = PrefixedString::decode(reader)?.0;
		match discriminant.as_str() {
			"block" => Ok(Self::Block(PackedPosition::decode(reader)?)),
			"entity" => Ok(Self::Entity(VarInt::decode(reader)?)),
			_ => Err(encde::Error::CustomStr("Invalid sculk destination; expected \"entity\" or \"block\"")),
		}
	}
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum AnimationType {
	SwingMainArm = 0,
	TakeDamage = 1,
	LeaveBed = 2,
	SwingOffhand = 3,
	CriticalEffect = 4,
	MagicCriticalEffect = 5,
}

/// Specific fields are TODO
#[derive(Encode, Decode)]
pub struct StatisticsEntry {
	category_id: VarInt,
	statistic_id: VarInt,
	value: VarInt,
}

pub enum DestroyStage {
	/// 0 to 9
	Breaking(u8),
	NotBreaking,
}

impl Encode for DestroyStage {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		match self {
			Self::Breaking(amount) => std::cmp::Ord::clamp(*amount, 0, 9).encode(writer),
			// any value outside of the 0..=9 range is acceptable
			Self::NotBreaking => u8::MAX.encode(writer),
		}
	}
}

impl Decode for DestroyStage {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let raw = u8::decode(reader)?;
		Ok(match raw {
			amount @ 0..=9 => Self::Breaking(amount),
			_ => Self::NotBreaking,
		})
	}
}

pub struct NbtData(pub nbt::Blob);

impl Encode for NbtData {
	fn encode(&self, mut writer: &mut dyn Write) -> EResult<()> {
		self.0.to_writer(&mut writer).map_err(|err| encde::Error::Custom(Box::new(err)))
	}
}

impl Decode for NbtData {
	fn decode(mut reader: &mut dyn Read) -> EResult<Self> {
		Ok(Self(nbt::Blob::from_reader(&mut reader).map_err(|err| encde::Error::Custom(Box::new(err)))?))
	}
}

#[derive(Encode, Decode)]
pub struct BlockAction {
	id: u8,
	param: u8,
}

#[derive(Encode)]
#[repr(u8)]
pub enum BossBarUpdateType {
	#[encde(wire_tag = 0)]
	Add {
		title: Chat,
		/// from 0 to 1
		health: f32,
		color: BossBarColor,
		notches: BossBarNotches,
		/// bit mask (TODO custom type)
		/// 1 = darken sky
		/// 2 = dragon bar
		/// 4 = create fog
		flags: u8,
	},
	#[encde(wire_tag = 1)]
	Remove,
	#[encde(wire_tag = 2)]
	UpdateHealth(f32),
	#[encde(wire_tag = 3)]
	UpdateTitle(Chat),
	#[encde(wire_tag = 4)]
	UpdateStyle { color: BossBarColor, notches: BossBarNotches },
	/// TODO custom type
	#[encde(wire_tag = 5)]
	UpdateFlags(u8),
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum BossBarColor {
	Pink = 0,
	Blue = 1,
	Red = 2,
	Green = 3,
	Yellow = 4,
	Purple = 5,
	White = 6,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum BossBarNotches {
	None = 0,
	Six = 1,
	Ten = 2,
	Twelve = 3,
	Twenty = 4,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum ServerDifficulty {
	Peaceful = 0,
	Easy = 1,
	Normal = 2,
	Hard = 3,
}

#[derive(Encode, Decode)]
#[repr(i8)]
pub enum ChatPosition {
	Chat = 0,
	System = 1,
	GameInfo = 2,
}

#[derive(Encode, Decode)]
#[repr(i8)]
pub enum ClientChatMode {
	Enabled = 0,
	CommandsOnly = 1,
	Hidden = 2,
}

#[derive(Encode)]
pub struct TabCompletion {
	text: PrefixedString,
	tooltip: PrefixedOption<Chat>,
}

pub type Slot = PrefixedOption<PresentSlot>;

#[derive(Encode, Decode)]
pub struct IndexedSlot {
	slot_index: i16,
	slot_data: Slot,
}

#[derive(Encode, Decode)]
pub struct PresentSlot {
	item_id: VarInt,
	count: i8,
	nbt_data: NbtData,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum SoundCategory {
	Root = 0,
	Music = 1,
	Records = 2,
	Weather = 3,
	Blocks = 4,
	Hostile = 5,
	Neutral = 6,
	Players = 7,
	Ambient = 8,
	Voice = 9,
}

pub struct Milliseconds(std::time::Duration);

impl Encode for Milliseconds {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		super::varint::VarLong(self.0.as_millis().try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(writer)
	}
}

impl Decode for Milliseconds {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let value = super::varint::VarLong::decode(reader)?.0;
		Ok(Self(std::time::Duration::from_millis(value.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?)))
	}
}

pub struct Seconds(std::time::Duration);

impl Encode for Seconds {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		super::varint::VarLong(self.0.as_secs().try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(writer)
	}
}

impl Decode for Seconds {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let value = super::varint::VarLong::decode(reader)?.0;
		Ok(Self(std::time::Duration::from_secs(value.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?)))
	}
}

#[derive(Encode, Decode)]
pub struct LightUpdateCommon {
	trust_edges: bool,
	sky_light_mask: PrefixedBitVec,
	block_light_mask: PrefixedBitVec,
	empty_sky_light_mask: PrefixedBitVec,
	empty_block_light_mask: PrefixedBitVec,
	sky_light_array: PrefixedVec<SkyLightData>,
	block_light_array: PrefixedVec<BlockLightData>,
}

// TODO implement
// #[derive(Encode, Decode)]
pub struct HeightMaps(());

// TODO implement
// #[derive(Encode, Decode)]
pub struct ChunkBlocks(());

// TODO implement
// #[derive(Encode, Decode)]
pub struct ChunkBlockEntities(());

/// 2048 u8 backing items, 4096 4-bit entries
#[derive(Encode, Decode)]
pub struct SkyLightData(PrefixedBitVec<u8>);

/// 2048 u8 backing items, 4096 4-bit entries
#[derive(Encode, Decode)]
pub struct BlockLightData(PrefixedBitVec<u8>);

#[derive(Encode, Decode)]
#[repr(i32)]
pub enum EffectId {
	DispenserDispenses = 1000,
	DispenserFailsToDispense = 1001,
	DispenserShoots = 1002,
	EnderEyeLaunched = 1003,
	FireworkShot = 1004,
	IronDoorOpened = 1005,
	WoodenDoorOpened = 1006,
	WoodenTrapdoorOpened = 1007,
	FenceGateOpened = 1008,
	FireExtinguished = 1009,
	PlayRecord = 1010,
	IronDoorClosed = 1011,
	WoodenDoorClosed = 1012,
	WoodenTrapdoorClosed = 1013,
	FenceGateClosed = 1014,
	GhastWarns = 1015,
	GhastShoots = 1016,
	EnderDragonShoots = 1017,
	BlazeShoots = 1018,
	ZombieAttacksWoodDoor = 1019,
	ZombieAttacksIronDoor = 1020,
	ZombieBreaksWoodDoor = 1021,
	WitherBreaksBlock = 1022,
	WitherSpawned = 1023,
	WitherShoots = 1024,
	BatTakesOff = 1025,
	ZombieInfects = 1026,
	ZombieVillagerConverted = 1027,
	EnderDragonDeath = 1028,
	AnvilDestroyed = 1029,
	AnvilUsed = 1030,
	AnvilLanded = 1031,
	PortalTravel = 1032,
	ChorusFlowerGrown = 1033,
	ChorusFlowerDied = 1034,
	BrewingStandBrewed = 1035,
	IronTrapdoorOpened = 1036,
	IronTrapdoorClosed = 1037,
	EndPortalCreatedInOverworld = 1038,
	PhantomBites = 1039,
	ZombieConvertsToDrowned = 1040,
	HuskConvertsToZombieByDrowning = 1041,
	GrindstoneUsed = 1042,
	BookPageTurned = 1043,
	ComposterComposts = 1500,
	LavaConvertsBlock = 1501,
	RedstoneTorchBurnsOut = 1502,
	EnderEyePlaced = 1503,
	Spawn10SmokeParticles = 2000,
	BlockBreak = 2001,
	SplashPotion = 2002,
	EyeOfEnderBreak = 2003,
	MobSpawnParticleEffect = 2004,
	BonemealParticles = 2005,
	DragonBreath = 2006,
	InstantSplashPotion = 2007,
	EnderDragonDestroysBlock = 2008,
	WetSpongeVaporizesInNether = 2009,
	EndGatewaySpawn = 3000,
	EnderDragonGrowl = 3001,
	ElectricSpark = 3002,
	CopperApplyWax = 3003,
	CopperRemoveWax = 3004,
	CopperScrapeOxidation = 3005,
}

#[derive(Encode, Decode)]
pub struct ChunkPosition<T: Encode + Decode> {
	pub x: T,
	pub z: T,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum GameMode {
	Survival = 0,
	Creative = 1,
	Adventure = 2,
	Spectator = 3,
}

#[derive(Encode, Decode)]
#[repr(i8)]
pub enum OptionalGameMode {
	None = -1,
	Survival = 0,
	Creative = 1,
	Adventure = 2,
	Spectator = 3,
}

#[derive(Encode)]
pub struct MapIcon {
	icon_type: MapIconType,
	position: ChunkPosition<i8>,
	direction: i8,
	display_name: PrefixedOption<Chat>,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum MapIconType {
	WhiteArrow = 0,
	GreenArrow = 1,
	RedArrow = 2,
	BlueArrow = 3,
	WhiteCross = 4,
	RedPointer = 5,
	WhiteCircle = 6,
	SmallWhiteCircle = 7,
	Mansion = 8,
	Temple = 9,
	WhiteBanner = 10,
	OrangeBanner = 11,
	MagentaBanner = 12,
	LightBlueBanner = 13,
	YellowBanner = 14,
	LimeBanner = 15,
	PinkBanner = 16,
	GrayBanner = 17,
	LightGrayBanner = 18,
	CyanBanner = 19,
	PurpleBanner = 20,
	BlueBanner = 21,
	BrownBanner = 22,
	GreenBanner = 23,
	RedBanner = 24,
	BlackBanner = 25,
	TreasureMarker = 26,
}

pub struct MapUpdate {
	columns: u8,
	rows: u8,
	top_left: ChunkPosition<i8>,
	data: PrefixedVec<u8>,
}

impl Encode for MapUpdate {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		self.columns.encode(writer)?;
		if self.columns > 0 {
			self.rows.encode(writer)?;
			self.top_left.encode(writer)?;
			self.data.encode(writer)?;
		}
		Ok(())
	}
}

#[derive(Encode)]
pub struct VillagerTrade {
	first_input_item: Slot,
	output_item: Slot,
	second_input_item: PrefixedOption<Slot>,
	trade_disabled: bool,
	num_uses: i32,
	max_uses: i32,
	xp_reward: i32,
	special_price: i32,
	price_fluctuation_multiplier: f32,
	demand: i32,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum PlayerHand {
	Main = 0,
	Off = 1,
}

#[derive(Encode)]
pub struct PlayerListAddPlayer {
	player_uuid: Uuid,
	/// Usually one item named "textures" with the profile, skin, and cape data from the Mojang API as Base-64 JSON
	properties: PrefixedVec<AddPlayerProperty>,
	gamemode: GameMode,
	ping: VarInt,
	display_name: PrefixedOption<Chat>,
}

#[derive(Encode)]
pub struct AddPlayerProperty {
	name: PrefixedString,
	value: PrefixedString,
	signature: PrefixedOption<PrefixedBytes>,
}

#[derive(Encode)]
pub struct PlayerListUpdateGamemode {
	player_uuid: Uuid,
	new_gamemode: GameMode,
}

#[derive(Encode)]
pub struct PlayerListUpdateLatency {
	player_uuid: Uuid,
	ping: VarInt,
}

#[derive(Encode)]
pub struct PlayerListUpdateDisplayName {
	player_uuid: Uuid,
	display_name: PrefixedOption<Chat>,
}

#[derive(Encode)]
pub struct PlayerListRemovePlayer {
	player_uuid: Uuid,
}

#[derive(Encode)]
pub struct PlayerRotationTargetEntity {
	entity_id: VarInt,
	/// see documentation in UpdatePlayerRotation
	use_eyes: bool,
}

pub enum UnlockRecipesAction {
	Init { already_shown: PrefixedVec<PrefixedString>, new: PrefixedVec<PrefixedString> },
	Add(PrefixedVec<PrefixedString>),
	Remove(PrefixedVec<PrefixedString>),
}

#[derive(Encode)]
pub struct RecipeBookState {
	open: bool,
	filter_active: bool,
}

pub struct ChunkSectionPosition(UnpackedPosition<i32>);

impl Encode for ChunkSectionPosition {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded: u64 = ((((self.0.x as u32) & 0x3fffff) as u64) << 42) | (((self.0.y as u32) & 0xfffff) as u64) | ((((self.0.z as u32) & 0x3fffff) as u64) << 20);
		encoded.encode(writer)
	}
}

pub struct MultiBlockChangeEntry {
	relative_position: UnpackedPosition<u8>,
	new_block_state: i32,
}

impl Encode for MultiBlockChangeEntry {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded: u64 = ((self.new_block_state as u64) << 12) | ((self.relative_position.x as u64) << 8) | ((self.relative_position.z as u64) << 4) | (self.relative_position.y as u64);
		super::varint::VarLong(encoded as i64).encode(writer)
	}
}

pub enum ScoreboardPosition {
	List,
	Sidebar,
	BelowName,
	TeamSidebar(u8),
}

impl Encode for ScoreboardPosition {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded: u8 = match self {
			Self::List => 0,
			Self::Sidebar => 1,
			Self::BelowName => 2,
			Self::TeamSidebar(team) => 3 + team,
		};
		encoded.encode(writer)
	}
}

// TODO implement
// #[derive(Encode)]
pub struct EntityMetadata(());

#[derive(Encode)]
pub struct EquipmentEntry {
	slot: EquipmentSlot,
	item: Slot,
}

#[derive(Encode)]
#[repr(u8)]
pub enum EquipmentSlot {
	MainHand = 0,
	OffHand = 1,
	Boots = 2,
	Leggings = 3,
	Chestplate = 4,
	Helmet = 5,
}

#[derive(Encode)]
#[repr(u8)]
pub enum ScoreboardObjectiveUpdate {
	#[encde(wire_tag = 0)]
	Create(ScoreboardObjectiveData),
	#[encde(wire_tag = 1)]
	Remove,
	#[encde(wire_tag = 2)]
	Update(ScoreboardObjectiveData),
}

#[derive(Encode)]
pub struct ScoreboardObjectiveData {
	value: Chat,
	ty: ScoreboardObjectiveType,
}

#[derive(Encode)]
#[repr(u8)]
pub enum ScoreboardObjectiveType {
	Integer = 0,
	Hearts = 1,
}

// TODO finish
/*
#[derive(Encode)]
#[repr(u8)]
pub enum TeamUpdate {
	#[encde(wire_tag = 0)]
	CreateTeam {
		display_name: Chat,
		/// Bit flags (TODO custom type)
		/// 1 = allow friendly fire
		/// 2 = can see invisible players on the same team
		friendly_flags: u8,
		name_tag_visibility: NameTagVisibility,
		collision_rule: CollisionRule,
	},
	#[encde(wire_tag = 1)]
	RemoveTeam,
	#[encde(wire_tag = 2)]
	UpdateTeam,
	#[encde(wire_tag = 3)]
	AddMembers,
	#[encde(wire_tag = 4)]
	RemoveMembers,
}
*/

pub enum NameTagVisibility {
	Always,
	HideForOtherTeams,
	HideForOwnTeam,
	Never,
}

impl Encode for NameTagVisibility {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let s = match self {
			Self::Always => "always",
			Self::HideForOtherTeams => "hideForOtherTeams",
			Self::HideForOwnTeam => "hideForOwnTeam",
			Self::Never => "never",
		};
		encode_u8_slice(writer, s.as_bytes())
	}
}

pub enum CollisionRule {
	Always,
	PushOtherTeams,
	PushOwnTeam,
	Never,
}

impl Encode for CollisionRule {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let s = match self {
			Self::Always => "always",
			Self::PushOtherTeams => "pushOtherTeams",
			Self::PushOwnTeam => "pushOwnTeam",
			Self::Never => "never",
		};
		encode_u8_slice(writer, s.as_bytes())
	}
}

// TODO custom types with Duration-based constructors
pub type Ticks64 = u64;
pub type Ticks32 = u32;
pub type TicksVarInt = VarInt;

#[derive(Encode)]
pub struct EntityProperty {
	key: PrefixedString,
	value: f64,
	/// These must be applied in order of their type: Add, then AddPercent, then Multiply
	modifiers: PrefixedVec<EntityPropertyModifier>,
}

#[derive(Encode)]
pub struct EntityPropertyModifier {
	uuid: Uuid,
	amount: f64,
	operation: EntityPropertyModifierType,
}

#[derive(Encode)]
#[repr(u8)]
pub enum EntityPropertyModifierType {
	/// value += amount
	Add = 0,
	/// value += value * (amount / 100)
	AddPercent = 1,
	/// value *= amount
	Multiply = 2,
}

pub struct Recipe {
	id: PrefixedString,
	data: RecipeType,
}

pub enum RecipeType {
	Shapeless {
		group: PrefixedString,
		ingredients: PrefixedVec<RecipeIngredient>,
		result: Slot,
	},
	Shaped {
		width: VarInt,
		height: VarInt,
		group: PrefixedString,
		/// width * height; row-major
		ingredients: Vec<RecipeIngredient>,
		result: Slot,
	},
	ArmorDye,
	BookCloning,
	MapCloning,
	MapExtending,
	FireworkRocket,
	FireworkStar,
	FireworkStarFade,
	RepairItem,
	TippedArrow,
	BannerDuplicate,
	BannerAddPattern,
	ShieldDecoration,
	ShulkerBoxColoring,
	SuspiciousStew,
	Smelting(SmeltingRecipe),
	Blasting(SmeltingRecipe),
	Smoking(SmeltingRecipe),
	CampfireCooking(SmeltingRecipe),
	Stonecutting {
		group: PrefixedString,
		ingredient: RecipeIngredient,
		result: Slot,
	},
	Smithing {
		base: RecipeIngredient,
		addition: RecipeIngredient,
		result: Slot,
	},
}

#[derive(Encode)]
pub struct SmeltingRecipe {
	group: PrefixedString,
	ingredient: RecipeIngredient,
	result: Slot,
	experience: f32,
	cooking_time: VarInt,
}

pub type RecipeIngredient = PrefixedVec<Slot>;

impl Encode for Recipe {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let identifier = match &self.data {
			RecipeType::Shapeless { .. } => "crafting_shapeless",
			RecipeType::Shaped { .. } => "crafting_shaped",
			RecipeType::ArmorDye => "crafting_special_armordye",
			RecipeType::BookCloning => "crafting_special_bookcloning",
			RecipeType::MapCloning => "crafting_special_mapcloning",
			RecipeType::MapExtending => "crafting_special_mapextending",
			RecipeType::FireworkRocket => "crafting_special_firework_rocket",
			RecipeType::FireworkStar => "crafting_special_firework_star",
			RecipeType::FireworkStarFade => "crafting_special_firework_star_fade",
			RecipeType::RepairItem => "crafting_special_repairitem",
			RecipeType::TippedArrow => "crafting_special_tippedarrow",
			RecipeType::BannerDuplicate => "crafting_special_bannerduplicate",
			RecipeType::BannerAddPattern => "crafting_special_banneraddpattern",
			RecipeType::ShieldDecoration => "crafting_special_shielddecoration",
			RecipeType::ShulkerBoxColoring => "crafting_special_shulkerboxcoloring",
			RecipeType::SuspiciousStew => "crafting_special_suspiciousstew",
			RecipeType::Smelting(_) => "smelting",
			RecipeType::Blasting(_) => "blasting",
			RecipeType::Smoking(_) => "smoking",
			RecipeType::CampfireCooking(_) => "campfire_cooking",
			RecipeType::Stonecutting { .. } => "stonecutting",
			RecipeType::Smithing { .. } => "smithing",
		};
		encode_u8_slice(writer, identifier.as_bytes())?;
		self.id.encode(writer)?;
		match &self.data {
			RecipeType::Shapeless { group, ingredients, result } => {
				group.encode(writer)?;
				ingredients.encode(writer)?;
				result.encode(writer)?;
			}
			RecipeType::Shaped { width, height, group, ingredients, result } => {
				width.encode(writer)?;
				height.encode(writer)?;
				group.encode(writer)?;
				// PANICS: this state is invalid and should not have occurred in the first place
				assert_eq!(ingredients.len(), usize::try_from(width.0).unwrap() * usize::try_from(height.0).unwrap());
				for item in ingredients.iter() {
					item.encode(writer)?;
				}
				result.encode(writer)?;
			}
			RecipeType::ArmorDye => {}
			RecipeType::BookCloning => {}
			RecipeType::MapCloning => {}
			RecipeType::MapExtending => {}
			RecipeType::FireworkRocket => {}
			RecipeType::FireworkStar => {}
			RecipeType::FireworkStarFade => {}
			RecipeType::RepairItem => {}
			RecipeType::TippedArrow => {}
			RecipeType::BannerDuplicate => {}
			RecipeType::BannerAddPattern => {}
			RecipeType::ShieldDecoration => {}
			RecipeType::ShulkerBoxColoring => {}
			RecipeType::SuspiciousStew => {}
			RecipeType::Smelting(inner) => inner.encode(writer)?,
			RecipeType::Blasting(inner) => inner.encode(writer)?,
			RecipeType::Smoking(inner) => inner.encode(writer)?,
			RecipeType::CampfireCooking(inner) => inner.encode(writer)?,
			RecipeType::Stonecutting { group, ingredient, result } => {
				group.encode(writer)?;
				ingredient.encode(writer)?;
				result.encode(writer)?;
			}
			RecipeType::Smithing { base, addition, result } => {
				base.encode(writer)?;
				addition.encode(writer)?;
				result.encode(writer)?;
			}
		};
		Ok(())
	}
}

#[derive(Encode)]
pub struct TagGroup {
	group_name: PrefixedString,
	tags: PrefixedVec<TaggedIds>,
}

#[derive(Encode)]
pub struct TaggedIds {
	tag_name: PrefixedString,
	ids: PrefixedVec<VarInt>,
}

#[derive(Encode)]
#[repr(u8)]
pub enum ScoreUpdate {
	#[encde(wire_tag = 0)]
	CreateUpdate { objective_name: PrefixedString, value: VarInt },
	#[encde(wire_tag = 1)]
	Remove { objective_name: PrefixedString },
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum InteractionType {
	#[encde(wire_tag = 0)]
	Interact { hand: PlayerHand },
	#[encde(wire_tag = 1)]
	Attack,
	#[encde(wire_tag = 2)]
	InteractAt { position: F32Position, hand: PlayerHand },
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum BlockFace {
	/// -Y
	Bottom = 0,
	/// +Y
	Top = 1,
	/// -Z
	North = 2,
	/// +Z
	South = 3,
	/// -X
	West = 4,
	/// +X
	East = 5,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum GeneralAction {
	StartDigging = 0,
	CancelDigging = 1,
	FinishDigging = 2,
	DropItemStack = 3,
	DropItem = 4,
	/// Eat food, shoot arrow, use bucket, etc
	UseHeldItem = 5,
	SwapHands = 6,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum EntityAction {
	StartSneaking = 0,
	StopSneaking = 1,
	LeaveBed = 2,
	StartSprinting = 3,
	StopSprinting = 4,
	StartHorseJump = 5,
	StopHorseJump = 6,
	OpenHorseInventory = 7,
	StartElytraFlight = 8,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum RecipeBookType {
	Crafting = 0,
	Furnace = 1,
	BlastFurnace = 2,
	Smoker = 3,
}

/// TODO custom type
pub type PotionId = VarInt;

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum CommandBlockMode {
	Sequence = 0,
	Auto = 1,
	Redstone = 2,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockAction {
	NoAction = 0,
	SaveStructure = 1,
	LoadStructure = 2,
	DetectSize = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockUpdateType {
	Save = 0,
	Load = 1,
	Corner = 2,
	Data = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockMirroring {
	None = 0,
	LeftRight = 1,
	FrontBack = 2,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockRotation {
	None = 0,
	Clockwise90 = 1,
	Clockwise180 = 2,
	Counterclockwise90 = 3,
}

pub struct Json<T>(pub T);

impl<T: serde::Serialize> Encode for Json<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded = serde_json::to_string(&self.0).map_err(|err| encde::Error::Custom(Box::new(err)))?;
		PrefixedString(encoded).encode(writer)
	}
}
