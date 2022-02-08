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
