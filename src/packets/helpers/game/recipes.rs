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

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum RecipeBookType {
	Crafting = 0,
	Furnace = 1,
	BlastFurnace = 2,
	Smoker = 3,
}
