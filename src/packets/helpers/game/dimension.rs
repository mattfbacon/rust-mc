#[derive(Serialize, Deserialize)]
pub struct DimensionCodec {
	#[serde(rename = "minecraft:dimension_type")]
	dimension_types: DimensionTypeRegistry,
	#[serde(rename = "minecraft:worldgen/biome")]
	biomes: BiomeRegistry,
}

pub struct DimensionTypeRegistry(Vec<DimensionTypeEntry>);

#[derive(Serialize, Deserialize)]
struct DimensionTypeRegistryWire<'a> {
	#[serde(rename = "type")]
	ty: &'a str,
	value: Vec<DimensionTypeEntry>,
}
impl Serialize for DimensionTypeRegistry {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		DimensionTypeRegistryWire { ty: "minecraft:dimension_type", value: self.0 }.serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for DimensionTypeRegistry {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		Ok(Self(DimensionTypeRegistryWire::deserialize(deserializer)?.value))
	}
}

#[derive(Serialize, Deserialize)]
pub struct DimensionTypeEntry {
	name: String,
	id: i32,
	element: DimensionType,
}

#[derive(Serialize, Deserialize)]
pub struct DimensionType {
	piglin_safe: bool,
	natural: bool,
	ambient_light: f32,
	fixed_time: Option<i64>,
	#[serde(rename = "infiniburn")]
	infinite_burn: String,
	respawn_anchor_works: bool,
	has_skylight: bool,
	bed_works: bool,
	#[serde(rename = "effects")]
	dimension_name: String,
	has_raids: bool,
	min_y: i32,
	height: i32,
	logical_height: i32,
	coordinate_scale: f32,
	#[serde(rename = "ultrawarm")]
	ultra_warm: bool,
	has_ceiling: bool,
}
