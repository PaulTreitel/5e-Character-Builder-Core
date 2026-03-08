use serde::{Deserialize, Serialize};

use crate::{basic::DieSize, class::Classes};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct CharHealth {
    current_max_hp: i32,
    base_max_hp: i32,
    current_hp: i32,
    temp_hp: u32,
    hit_dice: Vec<HitDice>,
    death_save_successes: u8,
    death_save_fails: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HitDice {
    size: DieSize,
    max: u8,
    used: u8,
    class_src: Classes,
}

pub enum HealthError {
    ExpendMoreHitDiceThanRemain,
    ExpendNonExistentHitDie,
    DieSizeMismatch,
    RemoveNonExistentDie,
}

impl CharHealth {
    pub fn new(
        current_hp: i32, 
        max_hp: i32, 
        base_max_hp: i32, 
        temp_hp: u32, 
        hit_dice: Vec<HitDice>, 
        death_save_successes: u8,
        death_save_fails: u8,
    ) -> Self {
        Self { 
            current_max_hp: max_hp, 
            base_max_hp, 
            current_hp, 
            temp_hp, 
            hit_dice,
            death_save_successes, 
            death_save_fails
        }
    }

    pub fn new_simple(
        current_hp: i32,
        max_hp: i32,
    ) -> Self {
        Self { 
            current_max_hp: max_hp, 
            base_max_hp: max_hp, 
            current_hp, 
            temp_hp: 0, 
            hit_dice: Vec::new(), 
            death_save_successes: 0, 
            death_save_fails: 0 
        }
    }
    
    pub fn max_hp(&self) -> i32 {
        self.current_max_hp
    }

    pub fn current_hp(&self) -> i32 {
        self.current_hp
    }

    pub fn temp_hp(&self) -> u32 {
        self.temp_hp
    }

    pub fn set_max_hp(& mut self, max: i32) -> () {
        self.current_max_hp = max;
    }

    pub fn set_current_hp(&mut self, hp: i32) -> () {
        self.current_hp = hp;
    }

    pub fn set_temp_hp(&mut self, tmp_hp: u32) -> () {
        self.temp_hp = tmp_hp;
    }

    pub fn reset_max_hp(&mut self) -> () {
        self.current_max_hp = self.base_max_hp;
    }

    pub fn reset_hp_to_max(&mut self) -> () {
        self.current_hp = self.current_max_hp;
    }

    pub fn reset_temp_hp(&mut self) -> () {
        self.temp_hp = 0;
    }

    pub fn change_max_hp(&mut self, amt: i32) -> () {
        if amt < 0 {
            self.current_hp += amt;
            self.current_max_hp += amt;   
        } else {
            self.current_max_hp += amt;
            self.current_hp += amt;
        }
    }

    pub fn take_damage(&mut self, dmg: u32) -> () {
        let mut total_taken = 0;
        if self.temp_hp > 0 {
            if self.temp_hp >= dmg {
                self.temp_hp -= dmg;
            } else {
                total_taken += self.temp_hp;
                self.temp_hp = 0;
            }
        }
        self.current_hp -= dmg as i32 - total_taken as i32;
    }

    pub fn heal(&mut self, heal:  u32) -> () {
        if self.current_hp <= 0 {
            self.current_hp = heal as i32;
        } else if self.current_hp + heal as i32 <= self.current_max_hp {
            self.current_hp += heal as i32;
        } else {
            self.current_hp = self.current_max_hp;
        }
    }

    pub fn gain_temp_hp(&mut self, tmp_hp: u32) -> () {
        if self.temp_hp < tmp_hp {
            self.temp_hp = tmp_hp;
        }
    }

    pub fn lose_temp_hp(&mut self, amt: u32) -> () {
        if amt >= self.temp_hp {
            self.temp_hp = 0;
        } else {
            self.temp_hp -= amt;
        }
    }

    pub fn total_num_hit_dice(&self) -> u32 {
        self.hit_dice.clone()
            .iter()
            .map(|x| x.max)
            .reduce(|acc, e| acc + e)
            .unwrap() as u32
    }

    pub fn hit_dice_remaining(&self) -> Vec<(Classes, DieSize, u8)> {
        self.hit_dice.clone()
            .iter()
            .filter(|x| x.max > x.used)
            .map(|x| (x.class_src.clone(), x.size.clone(), x.max - x.used))
            .collect()
    }

    pub fn long_rest_restore_hit_dice(&mut self) -> () {
        let mut count = (self.total_num_hit_dice() / 2) as i32;
        for i in 0..self.hit_dice.len() {
            let d = self.hit_dice.get_mut(i).unwrap();
            if count == 0 {
                return;
            } else if count >= d.used as i32 {
                count -= d.used as i32;
                d.used = 0;
            } else {
                d.used -= count as u8;
                return;
            }
        }
    }

    pub fn expend_hit_dice(&mut self, dice: &[(Classes, DieSize, u8)]) -> Result<(), HealthError> {
        let changes = self.find_hit_die_changes(dice)?;
        for (idx, num_used) in changes {
            self.hit_dice.get_mut(idx).unwrap().used += num_used;
        }
        Ok(())
    }

    fn find_hit_die_changes(
        &self, 
        dice: &[(Classes, DieSize, u8)]
    ) -> Result<Vec<(usize, u8)>, HealthError> {
        let mut changes = Vec::new();
        for i in 0..dice.len() {
            let spent = dice.get(i).unwrap();
            let mut match_found = false;
            for j in 0..self.hit_dice.len() {
                let current = self.hit_dice.get(j).unwrap();
                if spent.0 != current.class_src {
                    continue;
                }
                if spent.1 != current.size {
                    return Err(HealthError::DieSizeMismatch);
                }
                if spent.2 > current.max - current.used {
                    return Err(HealthError::ExpendMoreHitDiceThanRemain);
                }
                changes.push((i, spent.2));
                match_found = true;
            }
            if !match_found {
                return Err(HealthError::ExpendNonExistentHitDie);
            }
        }
        Ok(changes)
    }

    pub fn add_hit_die(&mut self, src: Classes, size: DieSize) -> Result<(), HealthError> {
        let p = self.hit_dice.binary_search_by(|x| x.class_src.cmp(&src));
        match p {
            Ok(idx) => {
                let d = self.hit_dice.get_mut(idx).unwrap();
                if d.size != size {
                    Err(HealthError::DieSizeMismatch)
                } else {
                    d.max += 1;
                    Ok(())
                }
            },
            Err(idx) => {
                let new = HitDice { size, max: 1, used: 0, class_src: src };
                self.hit_dice.insert(idx, new);
                Ok(())
            },
        }
    }

    pub fn remove_hit_die(&mut self, src: Classes) -> Result<(), HealthError> {
        let p = self.hit_dice.binary_search_by(|x| x.class_src.cmp(&src));
        match p {
            Ok(idx) => {
                let d = self.hit_dice.get_mut(idx).unwrap();
                if d.max > 1 {
                    d.max -= 1;
                } else {
                    self.hit_dice.remove(idx);
                }
                Ok(())
            },
            Err(_) => Err(HealthError::RemoveNonExistentDie),
        }
    }
}


mod test {
    use super::CharHealth;
    use crate::{basic::DieSize, class::Classes};

    #[test]
    fn test_hit_dice_sorting() {
        let mut h = CharHealth::default();
        let _ = h.add_hit_die(Classes::Cleric, DieSize::D8);
        let _ = h.add_hit_die(Classes::Sorcerer, DieSize::D6);
        let _ = h.add_hit_die(Classes::Barbarian, DieSize::D12);
        
        assert_eq!(h.hit_dice.get(0).unwrap().size, DieSize::D12);
        assert_eq!(h.hit_dice.get(1).unwrap().size, DieSize::D8);
        assert_eq!(h.hit_dice.get(2).unwrap().size, DieSize::D6);
    }
}