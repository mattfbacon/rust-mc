use super::super::wrappers::{std::*, uuid::Uuid};
use super::slot::Slot;
use bitvec::vec::BitVec;
use encde::{Decode, Encode, Result as EResult};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

// TODO implement
// #[derive(Encode)]
pub struct EntityMetadata(());

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
