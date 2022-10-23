use crate::data::{StyleName, DT};
use crate::stance_mod::stance::Stance;
use crate::stance_mod::stance_stats::{get_all_stances, StanceMap, StanceStats};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Style {
    pub name: StyleName,
    pub dt: DT,
    pub stance: Stance,
    pub stance_stats: Option<StanceStats>,
    pub attack_speed_mod: Option<i8>,
    pub attack_range_mod: Option<i8>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Styles {
    pub styles: Vec<Style>,
    pub default: usize,
}

impl Style {
    pub fn new(
        name: StyleName,
        dt: DT,
        stance: Stance,
        attack_speed_mod: Option<i8>,
        attack_range_mod: Option<i8>,
    ) -> Self {
        let stance_map: StanceMap = get_all_stances();
        let stance_stats = stance_map.get(&stance).unwrap().clone();

        Self {
            name,
            dt,
            stance,
            stance_stats,
            attack_speed_mod,
            attack_range_mod,
        }
    }
}

impl Styles {
    pub fn new(styles: Vec<Style>, default: usize) -> Self {
        match styles.len() > default {
            true => Self { styles, default },
            false => panic!("Default index must point to a valid style in styles"),
        }
    }

    pub fn default_style(&self) -> &Style {
        self.styles.get(self.default).unwrap()
    }
}
