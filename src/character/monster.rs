use crate::{bonus::MonsterBonus, levels::Levels};
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq)]
pub struct Monster {
    pub name: String,
    pub monster_bonus: MonsterBonus,
    pub location: MonsterLocation,
    pub slayer: Option<Slayer>,
    pub attributes: Option<Vec<MonsterAttribute>>,
    pub levels: Levels,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MonsterAttribute {
    Demon,
    Draconic,
    Fiery,
    Golem,
    Kalphite,
    Leafy,
    Penance,
    Shade,
    Spectral,
    Undead,
    Vampyre(VampyreTier),
    Xerician,
    Wilderness,
    Toa,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum VampyreTier {
    One,
    Two,
    Three,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MonsterLocation {
    Wilderness,
    Cox,
    Tob,
    Toa,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Slayer {
    AberrantSpectres,
    AbyssalDemons,
    AdamantDragons,
    Ankous,
    Aviansie,
    Bandits,
    Banshees,
    Basilisks,
    Bats,
    Bears,
    Birds,
    BlackDemons,
    BlackDragons,
    BlackKnights,
    Bloodvelds,
    BlueDragons,
    BrineRats,
    BronzeDragons,
    Catablepon,
    CaveBugs,
    CaveCrawlers,
    CaveHorrors,
    CaveSlimes,
    CaveKraken,
    ChaosDruids,
    Cockatrices,
    Cows,
    Dagannoths,
    DustDevils,
    FossilIslandWyverns,
    Goblins,
    GreaterDemons,
    GreenDragons,
    Hellhounds,
    Hydras,
    Jellies,
    Kalphites,
    Kurasks,
    LavaDragons,
    Lizardmen,
    MithrilDragons,
    Nechryael,
    RedDragons,
    Revenants,
    RuneDragons,
    Scorpions,
    Shades,
    SkeletalWyverns,
    Skeletons,
    SmokeDevils,
    Suqahs,
    Trolls,
    Vampyres,
    Wyrms,
}