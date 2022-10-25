use super::{Agg, BonusLike, Def};

/** Gear struct which represents a single Gear item. Gear implements BonusLike. Gear derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq, Default)]
pub struct MonsterBonus {
    agg: Agg,
    def: Def,
}

/// Implementing BonusLike for Gear basically provides copies / clones of private fields
impl BonusLike for MonsterBonus {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
}

impl MonsterBonus {
    pub fn new(agg: Agg, def: Def) -> Self {
        Self { agg, def }
    }
}
