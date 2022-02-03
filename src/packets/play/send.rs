use crate::packets::helpers::*;
use crate::packets::varint::{VarInt, VarLong};
use encde::Encode;

#[derive(Encode)]
struct SpawnEntity {
	entity_id: VarInt,
	object_uuid: Uuid,
	/// TODO more specific type
	entity_type: VarInt,
	position: F64Position,
	rotation: AngleRotation,
	object_data: i32,
	velocity: EntityVelocity,
}

#[derive(Encode)]
struct SpawnExperienceOrb {
	entity_id: VarInt,
	position: F64Position,
	experience_value: i16,
}

#[derive(Encode)]
struct SpawnLivingEntity {
	entity_id: VarInt,
	entity_uuid: Uuid,
	entity_type: VarInt,
	position: F64Position,
	rotation: AngleRotation,
	head_pitch: Angle,
	velocity: EntityVelocity,
}

#[derive(Encode)]
struct SpawnPainting {
	entity_id: VarInt,
	entity_uuid: Uuid,
	name: PaintingName,
	direction: CardinalDirection,
}

/// When a player comes into the visible range of another player
#[derive(Encode)]
struct SpawnPlayer {
	entity_id: VarInt,
	entity_uuid: Uuid,
	position: F64Position,
	rotation: AngleRotation,
}

#[derive(Encode)]
struct SculkVibrationSignal {
	source_position: PackedPosition,
	destination: SculkDestination,
	arrival_ticks: VarInt,
}

#[derive(Encode)]
pub struct EntityAnimation {
	entity_id: VarInt,
	animation: AnimationType,
}

#[derive(Encode)]
pub struct Statistics(PrefixedVec<StatisticsEntry>);

#[derive(Encode)]
pub struct AcknowledgePlayerDigging {
	location: PackedPosition,
	new_block_state: VarInt,
	/// Only digging-related actions are used
	desired_digging_status: GeneralAction,
	successful: bool,
}

#[derive(Encode)]
pub struct BlockBreakAnimation {
	breaker_entity_id: VarInt,
	block_location: PackedPosition,
	destroy_stage: DestroyStage,
}

#[derive(Encode)]
pub struct UpdateBlockEntityData {
	block_location: PackedPosition,
	tag_type: VarInt,
	nbt_data: NbtData,
}

/// AKA "Block Action"
#[derive(Encode)]
pub struct TriggerBlockAction {
	location: PackedPosition,
	action: BlockAction,
	block_state: VarInt,
}

#[derive(Encode)]
pub struct BlockChange {
	location: PackedPosition,
	new_block_state: VarInt,
}

#[derive(Encode)]
pub struct UpdateBossBar {
	bar_uuid: Uuid,
	update_type: BossBarUpdateType,
}

#[derive(Encode)]
pub struct UpdateServerDifficulty {
	difficulty: ServerDifficulty,
	locked: bool,
}

#[derive(Encode)]
pub struct ChatMessage {
	message: Chat,
	position: ChatPosition,
	sender: Uuid,
}

#[derive(Encode)]
pub struct ClearTitles {
	reset: bool,
}

#[derive(Encode)]
pub struct TabCompletions {
	transaction_id: VarInt,
	replace_start: VarInt,
	replace_length: VarInt,
	completions: PrefixedVec<TabCompletion>,
}

// TODO DeclareCommands

#[derive(Encode)]
pub struct CloseWindow {
	window_id: u8,
}

#[derive(Encode)]
pub struct UpdateWindowItems {
	window_id: u8,
	/// The client echoes the most recently received State ID in subsequent window-related packets
	state_id: VarInt,
	slot_data: PrefixedVec<Slot>,
	/// Item that the player is holding with their mouse
	floating_item: Slot,
}

#[derive(Encode)]
pub struct UpdateWindowProperty {
	window_id: u8,
	// TODO more specific type
	property_id: i16,
	new_value: i16,
}

#[derive(Encode)]
pub struct SetWindowSlot {
	window_id: i8, // not u8
	state_id: VarInt,
	slot: IndexedSlot,
}

#[derive(Encode)]
pub struct SetItemCooldown {
	item_id: VarInt,
	/// 0 ticks = no more cooldown
	cooldown_ticks: VarInt,
}

#[derive(Encode)]
pub struct PluginMessage {
	channel: PrefixedString,
	data: UnprefixedBytes,
}

#[derive(Encode)]
pub struct PlayNamedSoundEffect {
	sound_name: PrefixedString,
	sound_category: SoundCategory,
	effect_position: EffectPosition,
	/// 1 = full volume
	volume: f32,
	/// 0.5 to 2.0
	pitch: f32,
}

#[derive(Encode)]
pub struct Disconnect {
	reason: Chat,
}

#[derive(Encode)]
pub struct UpdateEntityStatus {
	entity_id: i32, // should this be VarInt?
	new_status: i8,
}

#[derive(Encode)]
pub struct Explosion {
	position: F32Position,
	strength: f32,
	blocks_destroyed: PrefixedVec<UnpackedPosition<i8>>,
	client_motion: F32Position,
}

#[derive(Encode)]
pub struct UnloadChunk {
	chunk_position: ChunkPosition<i32>,
}

#[derive(Encode)]
pub struct ChangeGameState {
	state_id: u8,
	/// Not always applicable to the state ID, but always included
	value: f32,
}

#[derive(Encode)]
pub struct OpenHorseWindow {
	window_id: u8,
	slot_count: VarInt,
	entity_id: i32,
}

#[derive(Encode)]
pub struct WorldBorderInitialize {
	center_x: f64,
	center_z: f64,
	old_diameter: f64,
	new_diameter: f64,
	transition_time: Milliseconds,
	portal_teleport_boundary: VarInt,
	warning_blocks: VarInt,
	warning_time: Seconds,
}

#[derive(Encode)]
pub struct KeepAlive(i64);

// TODO implement
// #[derive(Encode)]
pub struct UpdateChunkData {
	chunk_position: ChunkPosition<i32>,
	height_maps: HeightMaps,
	chunk_blocks: ChunkBlocks,
	chunk_block_entities: ChunkBlockEntities,
	common: LightUpdateCommon,
}

#[derive(Encode)]
pub struct TriggerEffect {
	effect_id: EffectId,
	location: PackedPosition,
	effect_data: i32,
	disable_relative_volume: bool,
}

#[derive(Encode)]
pub struct ShowParticle {
	particle_id: i32,
	long_distance: bool,
	position: F64Position,
	offset: F32Position,
	particle_data: f32,
	particle_count: i32,
	particle_extra_data: UnprefixedBytes,
}

#[derive(Encode)]
pub struct UpdateLight {
	chunk_position: ChunkPosition<VarInt>,
	common: LightUpdateCommon,
}

#[derive(Encode)]
pub struct JoinGame {
	entity_id: i32,
	is_hardcore: bool,
	new_game_mode: GameMode,
	old_game_mode: OptionalGameMode,
	dimension_names: PrefixedVec<PrefixedString>,
	dimension_codec: NbtData,
	dimension_data: NbtData,
	current_dimension: PrefixedString,
	hashed_seed: i64,
	max_players: VarInt,
	view_distance: VarInt,
	simulation_distance: VarInt,
	reduced_debug_info: bool,
	enable_respawn_screen: bool,
	is_debug: bool,
	is_flat: bool,
}

#[derive(Encode)]
pub struct MapData {
	map_id: VarInt,
	/// 0 to 4; higher is more zoomed-out
	map_scale: i8,
	locked: bool,
	is_tracking_position: bool,
	icons: PrefixedVec<MapIcon>,
	update_info: MapUpdate,
}

#[derive(Encode)]
pub struct TradeList {
	window_id: VarInt, // not byte
	trades: PrefixedVec<VillagerTrade, i8>,
	villager_level: VarInt,
	experience: VarInt,
	/// false for Wandering Trader
	is_regular: bool,
	/// false for Wandering Trader
	can_restock: bool,
}

#[derive(Encode)]
pub struct UpdateEntityNearPosition {
	entity_id: VarInt,
	/// ((current * 32) - (previous * 32)) * 128
	delta: UnpackedPosition<i16>,
	on_ground: bool,
}

#[derive(Encode)]
pub struct UpdateEntityNearPositionRotation {
	entity_id: VarInt,
	/// ((current * 32) - (previous * 32)) * 128
	position_delta: UnpackedPosition<i16>,
	new_rotation: AngleRotation,
	on_ground: bool,
}

#[derive(Encode)]
pub struct UpdateEntityRotation {
	entity_id: VarInt,
	new_rotation: AngleRotation,
	on_ground: bool,
}

#[derive(Encode)]
pub struct VehicleMove {
	position: F64Position,
	rotation: F32Rotation,
}

#[derive(Encode)]
pub struct OpenBook {
	hand: PlayerHand,
}

#[derive(Encode)]
pub struct OpenWindow {
	/// This ID is used as a handle for other window-related packets
	window_id: VarInt,
	window_type: VarInt,
	window_title: Chat,
}

#[derive(Encode)]
pub struct OpenSignEditor {
	sign_location: PackedPosition,
}

#[derive(Encode)]
pub struct Ping(i32);

#[derive(Encode)]
pub struct AcceptCraftRecipeSelection {
	window_id: i8,
	recipe: PrefixedString,
}

#[derive(Encode)]
pub struct UpdatePlayerAbilities {
	/// TODO more specific type
	/// Bit flags:
	/// 1 = invulnerable
	/// 2 = is actually flying right now
	/// 4 = is able to fly
	/// 8 = creative mode - instantly break blocks
	ability_flags: u8,
	/// Default = 0.05
	flying_speed: f32,
	/// Usually matches the movement speed
	field_of_view_modifier: f32,
}

// skipping {Enter,End}CombatEvent

/// AKA "Death Combat Event"
#[derive(Encode)]
pub struct PlayerDeath {
	/// matches client's ID
	player_id: VarInt,
	/// -1 = none
	killer_id: i32,
	death_message: Chat,
}

#[derive(Encode)]
#[repr(u8)]
pub enum UpdatePlayerList {
	#[encde(wire_tag = 0)]
	AddPlayers(PrefixedVec<PlayerListAddPlayer>),
	#[encde(wire_tag = 1)]
	UpdateGamemode(PrefixedVec<PlayerListUpdateGamemode>),
	#[encde(wire_tag = 2)]
	UpdateLatency(PrefixedVec<PlayerListUpdateLatency>),
	#[encde(wire_tag = 3)]
	UpdateDisplayName(PrefixedVec<PlayerListUpdateDisplayName>),
	#[encde(wire_tag = 4)]
	RemovePlayers(PrefixedVec<PlayerListRemovePlayer>),
}

#[derive(Encode)]
pub struct PlayerLookTargetUpdate {
	/// To determine the rotation, the client draws a line and uses trigonometry.
	/// This field determines the origin of that line. false is feet, true is eyes.
	use_eyes: bool,
	target: F64Position,
	target_entity: PrefixedOption<PlayerRotationTargetEntity>,
}

#[derive(Encode)]
pub struct PlayerPositionRotationUpdate {
	/// May be absolute or relative at the axis level
	new_position: F64Position,
	/// Ditto
	new_rotation: F32Rotation,
	/// TODO more specific type
	/// Bit flags:
	/// 1 = new_position.x is relative
	/// 2 = new_position.y is relative
	/// 4 = new_position.z is relative
	/// 8 = new_rotation.yaw is relative
	/// 16 = new_rotation.pitch is relative
	are_fields_relative: u8,
	/// Echoed by client in Teleport Confirm
	teleport_id: VarInt,
	dismount_vehicle: bool,
}

pub struct UnlockRecipes {
	action: UnlockRecipesAction,
	crafting_book: RecipeBookState,
	smelting_book: RecipeBookState,
	blast_furnace_book: RecipeBookState,
	smoker_book: RecipeBookState,
}

impl Encode for UnlockRecipes {
	fn encode(&self, writer: &mut dyn std::io::Write) -> encde::Result<()> {
		let discriminant = match &self.action {
			UnlockRecipesAction::Init { .. } => 0,
			UnlockRecipesAction::Add(_) => 1,
			UnlockRecipesAction::Remove(_) => 2,
		};
		VarInt(discriminant).encode(writer)?;
		self.crafting_book.encode(writer)?;
		self.smelting_book.encode(writer)?;
		self.blast_furnace_book.encode(writer)?;
		self.smoker_book.encode(writer)?;
		match &self.action {
			UnlockRecipesAction::Init { already_shown, new } => {
				already_shown.encode(writer)?;
				new.encode(writer)?;
			}
			UnlockRecipesAction::Add(recipes) => recipes.encode(writer)?,
			UnlockRecipesAction::Remove(recipes) => recipes.encode(writer)?,
		};
		Ok(())
	}
}

#[derive(Encode)]
pub struct RemoveEntities(PrefixedVec<VarInt>);

#[derive(Encode)]
pub struct RemoveEntityEffect {
	entity_id: VarInt,
	effect_id: i8,
}

// TODO Resource Pack Send

#[derive(Encode)]
pub struct RespawnPlayer {
	dimension_data: NbtData,
	dimension_name: PrefixedString,
	hashed_seed: i64,
	new_gamemode: GameMode,
	previous_gamemode: OptionalGameMode,
	is_debug: bool,
	is_flat: bool,
	/// If set, the player's metadata will be retained.
	/// For a respawn where the player loses their items, this should be false.
	is_dimension_change: bool,
}

#[derive(Encode)]
pub struct UpdateEntityHeadRotation {
	entity_id: VarInt,
	new_yaw: Angle,
}

#[derive(Encode)]
pub struct MultiBlockChange {
	/// Positions of blocks to update are relative to this position
	origin_position: ChunkSectionPosition,
	/// The opposite of the trust_edges field in LightUpdateCommon
	no_trust_edges: bool,
	changes: PrefixedVec<MultiBlockChangeEntry>,
}

#[derive(Encode)]
pub struct SelectAdvancementTab(PrefixedOption<PrefixedString>);

#[derive(Encode)]
pub struct ShowActionBar(Chat);

#[derive(Encode)]
pub struct WorldBorderSetCenter {
	x: f64,
	z: f64,
}

#[derive(Encode)]
pub struct WorldBorderTransitionDiameter {
	old_diameter: f64,
	new_diameter: f64,
	speed: VarLong,
}

#[derive(Encode)]
pub struct WorldBorderSetDiameter(f64);

#[derive(Encode)]
pub struct WorldBorderSetWarningTime(Milliseconds);

#[derive(Encode)]
pub struct WorldBorderSetWarningBlocks(VarInt);

#[derive(Encode)]
pub struct SpectateAsEntity {
	/// Use the player's ID to stop spectating
	entity_id: VarInt,
}

#[derive(Encode)]
pub struct ChangeActiveSlot(i8);

// AKA "Update View Position"
#[derive(Encode)]
pub struct UpdateActiveChunk(ChunkPosition<VarInt>);

#[derive(Encode)]
pub struct UpdateRenderDistance(VarInt);

/// Also updates where compasses point
#[derive(Encode)]
pub struct UpdateSpawnPosition {
	location: PackedPosition,
	/// FIXME angle of what?
	angle: f32,
}

#[derive(Encode)]
pub struct DisplayScoreboard {
	position: ScoreboardPosition,
	name: PrefixedString,
}

// #[derive(Encode)]
pub struct UpdateEntityMetadata {
	entity_id: VarInt,
	metadata: EntityMetadata,
}

#[derive(Encode)]
pub struct AttachEntity {
	attached_entity_id: VarInt,
	/// -1 = detach
	holding_entity_id: VarInt,
}

#[derive(Encode)]
pub struct UpdateEntityVelocity {
	entity_id: VarInt,
	velocity: EntityVelocity,
}

#[derive(Encode)]
pub struct UpdateEntityEquipment {
	entity_id: VarInt,
	equipment: PrefixedVec<EquipmentEntry>,
}

#[derive(Encode)]
pub struct UpdatePlayerExperience {
	/// 0 to 1
	bar_progress: f32,
	level: VarInt,
	total_experience: VarInt,
}

#[derive(Encode)]
pub struct UpdatePlayerHealth {
	/// 0 to 20; integer
	health: f32,
	food: VarInt,
	/// 0 to 5; integer
	saturation: f32,
}

#[derive(Encode)]
pub struct UpdateScoreboardObjective {
	objective_name: PrefixedString,
	update: ScoreboardObjectiveUpdate,
}

#[derive(Encode)]
pub struct SetVehiclePassengers {
	vehicle_id: VarInt,
	passengers: PrefixedVec<VarInt>,
}

// TODO finish
/* #[derive(Encode)]
pub struct UpdateTeam {
	team_name: PrefixedString,
	update: TeamUpdate,
}
*/

#[derive(Encode)]
pub struct UpdateScore {
	/// Username for players; UUID for entities
	entity_name: PrefixedString,
	action: ScoreUpdate,
}

#[derive(Encode)]
pub struct UpdateSimulationDistance(VarInt);

#[derive(Encode)]
pub struct SetTitleSubtitle(Chat);

#[derive(Encode)]
pub struct UpdateTime {
	world_age: Ticks64,
	time_of_day: Ticks64,
}

#[derive(Encode)]
pub struct SetTitleTitle(Chat);

#[derive(Encode)]
pub struct SetTitleTimes {
	fade_in: Ticks32,
	stay: Ticks32,
	fade_out: Ticks32,
}

#[derive(Encode)]
pub struct PlayEntitySoundEffect {
	sound_id: VarInt,
	sound_category: SoundCategory,
	entity_id: VarInt,
	volume: f32,
	pitch: f32,
}

#[derive(Encode)]
pub struct PlaySoundEffect {
	sound_id: VarInt,
	sound_category: SoundCategory,
	position: EffectPosition,
	volume: f32,
	pitch: f32,
}

#[derive(Encode)]
#[repr(u8)]
pub enum StopSound {
	#[encde(wire_tag = 0)]
	AllSounds,
	#[encde(wire_tag = 1)]
	ByCategory(SoundCategory),
	#[encde(wire_tag = 2)]
	BySoundName(PrefixedString),
	#[encde(wire_tag = 3)]
	FullyQualified(SoundCategory, PrefixedString),
}

#[derive(Encode)]
pub struct UpdatePlayerListDecoration {
	header: Chat,
	footer: Chat,
}

#[derive(Encode)]
pub struct NbtQueryResponse {
	transaction_id: VarInt,
	data: NbtData,
}

#[derive(Encode)]
pub struct CollectItem {
	collected_entity_id: VarInt,
	collector_entity_id: VarInt,
	/// 1 if N/A (e.g., experience orbs)
	count: VarInt,
}

#[derive(Encode)]
pub struct TeleportEntity {
	entity_id: VarInt,
	position: F64Position,
	rotation: AngleRotation,
	on_ground: bool,
}

// TODO "Advancements"

#[derive(Encode)]
pub struct UpdateEntityProperties {
	entity_id: VarInt,
	properties: PrefixedVec<EntityProperty>,
}

#[derive(Encode)]
pub struct UpdateEntityEffect {
	entity_id: VarInt,
	effect_id: i8,
	amplifier: i8,
	duration: TicksVarInt,
}

#[derive(Encode)]
pub struct DeclareRecipes(PrefixedVec<Recipe>);

#[derive(Encode)]
pub struct DeclareTaggedGroups(PrefixedVec<TagGroup>);

#[derive(Encode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0x00)]
	SpawnEntity(SpawnEntity),
	#[encde(wire_tag = 0x01)]
	SpawnExperienceOrb(SpawnExperienceOrb),
	#[encde(wire_tag = 0x02)]
	SpawnLivingEntity(SpawnLivingEntity),
	#[encde(wire_tag = 0x03)]
	SpawnPainting(SpawnPainting),
	#[encde(wire_tag = 0x04)]
	SpawnPlayer(SpawnPlayer),
	#[encde(wire_tag = 0x05)]
	SculkVibrationSignal(SculkVibrationSignal),
	#[encde(wire_tag = 0x06)]
	EntityAnimation(EntityAnimation),
	#[encde(wire_tag = 0x07)]
	Statistics(Statistics),
	#[encde(wire_tag = 0x08)]
	AcknowledgePlayerDigging(AcknowledgePlayerDigging),
	#[encde(wire_tag = 0x09)]
	BlockBreakAnimation(BlockBreakAnimation),
	#[encde(wire_tag = 0x0a)]
	UpdateBlockEntityData(UpdateBlockEntityData),
	#[encde(wire_tag = 0x0b)]
	TriggerBlockAction(TriggerBlockAction),
	#[encde(wire_tag = 0x0c)]
	BlockChange(BlockChange),
	#[encde(wire_tag = 0x0d)]
	UpdateBossBar(UpdateBossBar),
	#[encde(wire_tag = 0x0e)]
	UpdateServerDifficulty(UpdateServerDifficulty),
	#[encde(wire_tag = 0x0f)]
	ChatMessage(ChatMessage),
	#[encde(wire_tag = 0x10)]
	ClearTitles(ClearTitles),
	#[encde(wire_tag = 0x11)]
	TabCompletions(TabCompletions),
	// #[encde(wire_tag = 0x12)]
	// DeclareCommands(DeclareCommands),
	#[encde(wire_tag = 0x13)]
	CloseWindow(CloseWindow),
	#[encde(wire_tag = 0x14)]
	UpdateWindowItems(UpdateWindowItems),
	#[encde(wire_tag = 0x15)]
	UpdateWindowProperty(UpdateWindowProperty),
	#[encde(wire_tag = 0x16)]
	SetWindowSlot(SetWindowSlot),
	#[encde(wire_tag = 0x17)]
	SetItemCooldown(SetItemCooldown),
	#[encde(wire_tag = 0x18)]
	PluginMessage(PluginMessage),
	#[encde(wire_tag = 0x19)]
	PlayNamedSoundEffect(PlayNamedSoundEffect),
	#[encde(wire_tag = 0x1a)]
	Disconnect(Disconnect),
	#[encde(wire_tag = 0x1b)]
	UpdateEntityStatus(UpdateEntityStatus),
	#[encde(wire_tag = 0x1c)]
	Explosion(Explosion),
	#[encde(wire_tag = 0x1d)]
	UnloadChunk(UnloadChunk),
	#[encde(wire_tag = 0x1e)]
	ChangeGameState(ChangeGameState),
	#[encde(wire_tag = 0x1f)]
	OpenHorseWindow(OpenHorseWindow),
	#[encde(wire_tag = 0x20)]
	WorldBorderInitialize(WorldBorderInitialize),
	#[encde(wire_tag = 0x21)]
	KeepAlive(KeepAlive),
	// #[encde(wire_tag = 0x22)]
	// UpdateChunkData(UpdateChunkData),
	#[encde(wire_tag = 0x23)]
	TriggerEffect(TriggerEffect),
	#[encde(wire_tag = 0x24)]
	ShowParticle(ShowParticle),
	#[encde(wire_tag = 0x25)]
	UpdateLight(UpdateLight),
	#[encde(wire_tag = 0x26)]
	JoinGame(JoinGame),
	#[encde(wire_tag = 0x27)]
	MapData(MapData),
	#[encde(wire_tag = 0x28)]
	TradeList(TradeList),
	#[encde(wire_tag = 0x29)]
	UpdateEntityNearPosition(UpdateEntityNearPosition),
	#[encde(wire_tag = 0x2a)]
	UpdateEntityNearPositionRotation(UpdateEntityNearPositionRotation),
	#[encde(wire_tag = 0x2b)]
	UpdateEntityRotation(UpdateEntityRotation),
	#[encde(wire_tag = 0x2c)]
	VehicleMove(VehicleMove),
	#[encde(wire_tag = 0x2d)]
	OpenBook(OpenBook),
	#[encde(wire_tag = 0x2e)]
	OpenWindow(OpenWindow),
	#[encde(wire_tag = 0x2f)]
	OpenSignEditor(OpenSignEditor),
	#[encde(wire_tag = 0x30)]
	Ping(Ping),
	#[encde(wire_tag = 0x31)]
	AcceptCraftRecipeSelection(AcceptCraftRecipeSelection),
	#[encde(wire_tag = 0x32)]
	UpdatePlayerAbilities(UpdatePlayerAbilities),
	// #[encde(wire_tag = 0x33)]
	// EndCombatEvent(EndCombatEvent)
	// #[encde(wire_tag = 0x34)]
	// EnterCombatEvent(EnterCombatEvent)
	#[encde(wire_tag = 0x35)]
	PlayerDeath(PlayerDeath),
	#[encde(wire_tag = 0x36)]
	UpdatePlayerList(UpdatePlayerList),
	#[encde(wire_tag = 0x37)]
	PlayerLookTargetUpdate(PlayerLookTargetUpdate),
	#[encde(wire_tag = 0x38)]
	PlayerPositionRotationUpdate(PlayerPositionRotationUpdate),
	#[encde(wire_tag = 0x39)]
	UnlockRecipes(UnlockRecipes),
	#[encde(wire_tag = 0x3a)]
	RemoveEntities(RemoveEntities),
	#[encde(wire_tag = 0x3b)]
	RemoveEntityEffect(RemoveEntityEffect),
	// #[encde(wire_tag = 0x3c)]
	// SendResourcePack(SendResourcePack),
	#[encde(wire_tag = 0x3d)]
	RespawnPlayer(RespawnPlayer),
	#[encde(wire_tag = 0x3e)]
	UpdateEntityHeadRotation(UpdateEntityHeadRotation),
	#[encde(wire_tag = 0x3f)]
	MultiBlockChange(MultiBlockChange),
	#[encde(wire_tag = 0x40)]
	SelectAdvancementTab(SelectAdvancementTab),
	#[encde(wire_tag = 0x41)]
	ShowActionBar(ShowActionBar),
	#[encde(wire_tag = 0x42)]
	WorldBorderSetCenter(WorldBorderSetCenter),
	#[encde(wire_tag = 0x43)]
	WorldBorderTransitionDiameter(WorldBorderTransitionDiameter),
	#[encde(wire_tag = 0x44)]
	WorldBorderSetDiameter(WorldBorderSetDiameter),
	#[encde(wire_tag = 0x45)]
	WorldBorderSetWarningTime(WorldBorderSetWarningTime),
	#[encde(wire_tag = 0x46)]
	WorldBorderSetWarningBlocks(WorldBorderSetWarningBlocks),
	#[encde(wire_tag = 0x47)]
	SpectateAsEntity(SpectateAsEntity),
	#[encde(wire_tag = 0x48)]
	ChangeActiveSlot(ChangeActiveSlot),
	#[encde(wire_tag = 0x49)]
	UpdateActiveChunk(UpdateActiveChunk),
	#[encde(wire_tag = 0x4a)]
	UpdateRenderDistance(UpdateRenderDistance),
	#[encde(wire_tag = 0x4b)]
	UpdateSpawnPosition(UpdateSpawnPosition),
	#[encde(wire_tag = 0x4c)]
	DisplayScoreboard(DisplayScoreboard),
	// #[encde(wire_tag = 0x4d)]
	// UpdateEntityMetadata(UpdateEntityMetadata),
	#[encde(wire_tag = 0x4e)]
	AttachEntity(AttachEntity),
	#[encde(wire_tag = 0x4f)]
	UpdateEntityVelocity(UpdateEntityVelocity),
	#[encde(wire_tag = 0x50)]
	UpdateEntityEquipment(UpdateEntityEquipment),
	#[encde(wire_tag = 0x51)]
	UpdatePlayerExperience(UpdatePlayerExperience),
	#[encde(wire_tag = 0x52)]
	UpdatePlayerHealth(UpdatePlayerHealth),
	#[encde(wire_tag = 0x53)]
	UpdateScoreboardObjective(UpdateScoreboardObjective),
	#[encde(wire_tag = 0x54)]
	SetVehiclePassengers(SetVehiclePassengers),
	// #[encde(wire_tag = 0x55)]
	// UpdateTeams(UpdateTeams)
	#[encde(wire_tag = 0x56)]
	UpdateScore(UpdateScore),
	#[encde(wire_tag = 0x57)]
	UpdateSimulationDistance(UpdateSimulationDistance),
	#[encde(wire_tag = 0x58)]
	SetTitleSubtitle(SetTitleSubtitle),
	#[encde(wire_tag = 0x59)]
	UpdateTime(UpdateTime),
	#[encde(wire_tag = 0x5a)]
	SetTitleTitle(SetTitleTitle),
	#[encde(wire_tag = 0x5b)]
	SetTitleTimes(SetTitleTimes),
	#[encde(wire_tag = 0x5c)]
	PlayEntitySoundEffect(PlayEntitySoundEffect),
	#[encde(wire_tag = 0x5d)]
	PlaySoundEffect(PlaySoundEffect),
	#[encde(wire_tag = 0x5e)]
	StopSound(StopSound),
	#[encde(wire_tag = 0x5f)]
	UpdatePlayerListDecoration(UpdatePlayerListDecoration),
	#[encde(wire_tag = 0x60)]
	NbtQueryResponse(NbtQueryResponse),
	#[encde(wire_tag = 0x61)]
	CollectItem(CollectItem),
	#[encde(wire_tag = 0x62)]
	TeleportEntity(TeleportEntity),
	// #[encde(wire_tag = 0x63)]
	// Advancements(Advancements)
	#[encde(wire_tag = 0x64)]
	UpdateEntityProperties(UpdateEntityProperties),
	#[encde(wire_tag = 0x65)]
	UpdateEntityEffect(UpdateEntityEffect),
	#[encde(wire_tag = 0x66)]
	DeclareRecipes(DeclareRecipes),
	#[encde(wire_tag = 0x67)]
	DeclareTaggedGroups(DeclareTaggedGroups),
}
