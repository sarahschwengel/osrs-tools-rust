use crate::{
    bonus::{EquipmentInfo, EquipmentMap, GearLike, Weapon},
    boost::{Boost, BoostMap},
    character::Slayer,
    combat::{effective_level, multiply_then_trunc},
    data::{CombatAspect, MeleeDamageType},
    levels::{combat_skill, Levels},
    modifiers::{player::void::VoidModifiers, CmbMod},
    prayer::{Prayer, Prayers},
    spell::{Spell, Spellbook},
    stance::Stance,
    style::Style,
    Result, Skill, DT, STANCE_MAP,
};

/** The Player struct describes a player in OSRS.
 */
pub struct Player {
    pub equipment_info: EquipmentInfo,
    pub style: &'static Style,
    pub spell: Option<&'static Spell>,
    pub levels: Levels,
    prayers: Option<Prayers>,
    boosts: Boost,
    // #[allow(dead_code)]
    // effects: Vec<Box<dyn EffectLike>>,
    pub kandarin_hard: bool,
    pub slayer_task: Option<Slayer>,
    // Should be hands
    default_weapon: &'static Weapon,
    pub uid: u32,
    pub resources: PlayerResources,
}

pub struct PlayerResources {
    #[allow(dead_code)]
    special_energy: u32,
    #[allow(dead_code)]
    run_energy: u32,
    pub hitpoints: u32,
    pub prayer: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Player {
    /// Return a shorthand reference to equipped gear container
    pub fn eqpd(&self) -> &EquipmentMap {
        &self.equipment_info.equipment.equipment
    }

    /// Return a reference to the Weapon slot item in equipment.
    pub fn weapon(&self) -> &Weapon {
        match &self.equipment_info.equipment.weapon {
            Some(wpn) => wpn,
            None => self.default_weapon,
        }
    }

    /// Return the actual attack speed, accounting for style and spell modifiers.
    pub fn attack_speed(&self) -> u8 {
        let mut atk_spd: u8 = self.weapon().weapon_info.base_attack_speed;
        let stance_modifier: i8 = self.style.attack_speed_mod.unwrap_or(0);
        atk_spd = ((atk_spd as i8) + stance_modifier) as u8;

        if let Some(spl) = self.spell {
            if spl.spellbook == Spellbook::Standard {
                todo!();
                // if self.weapon() == HarmonisedNightmareStaff {
                //     atk_spd -= 1;
                // }
            }
        }

        atk_spd
    }

    /** Add a prayer to the collection, panicing if this results in a bad prayer collection
     *  This happens with overlapping stat modifiers, or modifiers across damage types that violate
     *  the rules.
     */
    pub fn pray(&mut self, prayer: &'static Prayer) -> Result<()> {
        self.prayers = match &self.prayers {
            Some(prev_pryrs) => {
                let mut prayers_vec = vec![prayer];
                for pry in prev_pryrs.prayers.iter() {
                    prayers_vec.push(pry);
                }

                let pryrs: Prayers = Prayers::new(prayers_vec)?;
                Some(pryrs)
            }
            None => {
                let pryrs: Prayers = Prayers::new(vec![prayer])?;
                Some(pryrs)
            }
        };

        Ok(())
    }

    pub fn get_prayers(&self) -> &Option<Prayers> {
        &self.prayers
    }

    /// The level displayed on the stats page, the sum of a player's level and boosts.
    pub fn get_visible_level(&self, skill: Skill) -> u32 {
        let minimum_visible_level: u32 = 1;
        let maximum_visible_level: u32 = 125;

        let visible_level: i32 =
            self.levels.get(&skill).unwrap() + self.boosts.get(&skill).unwrap_or(&0);

        // sanity check before type cast
        assert!((minimum_visible_level..=maximum_visible_level).contains(&(visible_level as u32)));

        let visible_level: u32 = (visible_level as u32)
            .max(minimum_visible_level)
            .min(maximum_visible_level);

        visible_level
    }

    /// Return the invisible level, which is the visible level modified by the prayer modifier.
    pub fn get_invisible_level(&self, dt: DT, aspect: CombatAspect) -> u32 {
        let skill: Skill = combat_skill(dt, aspect);
        let mut invisible_level: u32 = self.get_visible_level(skill);

        if let Some(prys) = &self.prayers {
            if let Some(prayer_stats) = &prys.prayer_stats {
                if let Some(pray_mod) = prayer_stats.get(&(dt, aspect)) {
                    let pray_mod_f64 = (100 + pray_mod) as f64 / 100.0;
                    invisible_level = multiply_then_trunc(invisible_level, pray_mod_f64);
                }
            }
        }
        invisible_level
    }

    /** Return the effective level, which is the level used in further accuracy / damage
     *  calculations
     */
    pub fn get_effective_level(&self, dt: DT, aspect: CombatAspect) -> u32 {
        let invis_lvl: u32 = self.get_invisible_level(dt, aspect);
        let stance_mod: Option<i32>;
        let stance: Stance = self.style.stance;

        if let Some(Some(stance_stats)) = STANCE_MAP.get(&stance) {
            if let Some(st_mod) = stance_stats.get(&(dt, aspect)) {
                stance_mod = Some(*st_mod)
            } else {
                stance_mod = None
            }
        } else {
            stance_mod = None
        }

        // For magic defence, employ recursion. OSRS has some weird formulas.
        if dt == DT::Magic && aspect == CombatAspect::Defence {
            let mut adj_def: u32 = self
                .get_effective_level(DT::Melee(MeleeDamageType::Default), CombatAspect::Defence);
            adj_def = multiply_then_trunc(adj_def, 0.30);

            let mut adj_mag: u32 = self.get_effective_level(DT::Magic, CombatAspect::Attack);
            adj_mag = multiply_then_trunc(adj_mag, 0.70);

            let eff_mag_def: u32 = adj_def + adj_mag;
            return eff_mag_def;
        }

        let void_modifiers = VoidModifiers { player: self };
        let void_mod = if let Some(void_mods) = void_modifiers.combat_mod() {
            // // interesting rust corner
            // // I wrote this...
            //
            // if let Some(lvl_mod) = void_mods.get(&(dt, aspect)) {
            //     Some(*lvl_mod)
            // } else {
            //     None
            // }
            //
            // // This was suggested by the compiler as a first replacement...
            //
            // void_mods.get(&(dt, aspect)).map(|lvl_mod| *lvl_mod)
            //
            // // Which then suggested this...
            //
            void_mods.get(&(dt, aspect)).copied()
            //
            // // In short, cool language!
        } else {
            None
        };

        effective_level(invis_lvl, stance_mod, void_mod)
    }

    /// Apply a boost map, taking previous buffs & debuffs into account.
    pub fn apply_boost(&mut self, boost: &BoostMap) {
        for (skill, mod_func) in boost.iter() {
            if let Some(prev_boost) = self.boosts.get(skill) {
                let active_level: i32 = if *prev_boost < 0 {
                    self.levels.get(skill).unwrap() + prev_boost
                } else {
                    *self.levels.get(skill).unwrap()
                };

                let diff_boost: i32 = mod_func(&active_level);
                let new_boost: i32 = prev_boost + diff_boost;
                self.boosts.insert(*skill, new_boost);
            } else {
                let active_level: &i32 = self.levels.get(skill).unwrap();
                let diff_boost: i32 = mod_func(active_level);
                self.boosts.insert(*skill, diff_boost);
            }
        }
    }

    /// Calculate a Player's combat level.
    pub fn combat_level(&self) -> i32 {
        let atk: &i32 = self.levels.get(&Skill::Attack).unwrap();
        let stn: &i32 = self.levels.get(&Skill::Strength).unwrap();
        let def: &i32 = self.levels.get(&Skill::Defence).unwrap();
        let rng: &i32 = self.levels.get(&Skill::Ranged).unwrap();
        let mag: &i32 = self.levels.get(&Skill::Magic).unwrap();
        let pry: &i32 = self.levels.get(&Skill::Prayer).unwrap();
        let htp: &i32 = self.levels.get(&Skill::Hitpoints).unwrap();

        // Base level
        let base_lvl: i32 = def + htp + (pry / 2);
        let base_lvl: f64 = (1.0 / 4.0) * (base_lvl as f64);

        // Specialized level
        let melee_lvl: f64 = (13.0 / 40.0) * (atk + stn) as f64;
        let magic_lvl: f64 = (13.0 / 40.0) * (mag + mag / 2) as f64;
        let ranged_lvl: f64 = (13.0 / 40.0) * (rng + rng / 2) as f64;
        let type_lvl: f64 = melee_lvl.max(magic_lvl).max(ranged_lvl);

        let combat_lvl: i32 = (base_lvl + type_lvl).trunc() as i32;
        combat_lvl
    }

    /// Caculate a Player's prayer drain resistance.
    pub fn prayer_drain_resistance(&self) -> u32 {
        let pdr: u32 = (2 * self.equipment_info.equipment.get_pry()) + 60;
        pdr
    }

    /// Calculate a the ticks it takes to lose one prayer point
    pub fn ticks_per_pp_lost(&self) -> u32 {
        if let Some(prys) = &self.prayers {
            let ticks: u32 = self.prayer_drain_resistance() / prys.drain_effect;
            ticks
        } else {
            0
        }
    }

    /// Calculate a player's maximum attack range
    pub fn attack_range(&self) -> u8 {
        let min_range: u8 = 0;
        let max_range: u8 = 10;
        let weapon: &Weapon = self.weapon();

        let mut atk_rng: u8 = weapon.weapon_info.base_attack_range;

        // Apply stance modifier
        if let Some(stance_mod) = self.style.attack_speed_mod {
            atk_rng = ((atk_rng as i8) + stance_mod) as u8;
        }

        // Clamp to boundaries
        atk_rng = atk_rng.min(max_range).max(min_range);
        atk_rng
    }
}
