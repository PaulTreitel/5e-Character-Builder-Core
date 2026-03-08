use crate::basic;

use crate::feats;

use basic::{Skill, Stat};

use feats::{feat_effect::FeatEffect, Feat};

pub fn make_feats() -> () {
        let alert_desc = "Always on the lookout for danger, you gain the following benefits:\n• You gain a +5 bonus to initiative.\n• You can’t be surprised while you are conscious.\n• Other creatures don’t gain advantage on attack rolls against you as a result of being hidden from you.";
        let alert_effects = vec![FeatEffect::InitiativeBonus(5)];
        let _alert = Feat::new(
            "Alert".to_string(), 
            alert_desc.to_string(), 
            None, 
            alert_effects
        );

        // should athlete have an effect for standing up with 5ft?
        let athlete_desc = "You have undergone extensive physical training to gain the following benefits:\n• Increase your Strength or D exterity score by 1, to a maximum of 20.\n• When you are prone, standing up uses only 5 feet of your movement.• Climbing doesn ’t halve your speed.\n• You can make a running long jump or a running high jump after moving only 5 feet on foot, rather than 10 feet.";
        let athlete_effects = vec![
            FeatEffect::IncreaseStat { options: vec![Stat::Strength, Stat::Dexterity], by: 1, max: 20 }];
        let _athlete = Feat::new(
            "Athlete".to_string(), 
            athlete_desc.to_string(), 
            None, 
            athlete_effects
        );
        
        let actor_desc = "Skilled at mimicry and dramatics, you gain the following benefits:\n• Increase your Charisma score by 1, to a maximum of 20.\n• You have advantage on Charisma (Deception) and Charisma (Performance) checks when trying to pass yourself off as a different person.\n• You can mimic the speech of another person or the sounds made by other creatures. You must have heard the person speaking, or heard the creature make the sound, for at least 1 minute. A successful Wisdom (Insight) check contested by your Charisma (Deception) check allows a listener to determine that the effect is faked.";
        let actor_effects = vec![
            FeatEffect::IncreaseStat { options: vec![Stat::Charisma], by: 1, max: 20 },
            FeatEffect::SkillAdv { 
                s: Skill::Deception, 
                context: Some("when trying to pass yourself off as a different person".to_string()) 
            },
            FeatEffect::SkillAdv { 
                s: Skill::Performance, 
                context: Some("when trying to pass yourself off as a different person".to_string()) 
            },
        ];
        let _actor = Feat::new(
            "Actor".to_string(), 
            actor_desc.to_string(), 
            None, 
            actor_effects
        );

        let charger_desc = "When you use your action to Dash, you can use a bonus action to make one melee weapon attack or to shove a creature.\n\tIf you move at least 10 feet in a straight line immediately before taking this bonus action, you either gain a +5 bonus to the attack’s damage roll (if you chose to make a melee attack and hit) or push the target up to 10 feet away from you (if you chose to shove and you succeed).";
        let charger_effects = vec![
            FeatEffect::GrantAction { 
                time: basic::ActivationTime::BonusAction, 
                action_desc: charger_desc.to_string() }
        ];
        let _charger = Feat::new(
            "Charger".to_string(), 
            charger_desc.to_string(), 
            None, 
            charger_effects
        );

        let cbe_desc = "Thanks to extensive practice with the crossbow, you gain the following benefits:\n• You ignore the loading quality of crossbows with which you are proficient.\n• Being within 5 feet of a hostile creature doesn’t impose disadvantage on your ranged attack rolls.\n• When you use the Attack action and attack with a one-handed weapon , you can use a bonus action to attack with a loaded hand crossbow you are holding.";
        let cbe_effects = vec![
            FeatEffect::GrantAction { 
                time: basic::ActivationTime::BonusAction, 
                action_desc: "When you use the Attack action and attack with a one-handed weapon , you can use a bonus action to attack with a loaded hand crossbow you are holding"
                    .to_string()
            }
        ];
        let _crossbow_expert = Feat::new(
            "Crossbow Expert".to_string(), 
            cbe_desc.to_string(), 
            None, 
            cbe_effects
        );

        let defensive_duelist_desc = "When you are wielding a finesse weapon with which you are proficient and another creature hits you with a melee attack, you can use your reaction to add your proficiency bonus to your AC for that attack, potentially causing the attack to miss you.";
        let defensive_duelist_effects = vec![
            FeatEffect::GrantAction { 
                time: basic::ActivationTime::Reaction, 
                action_desc: defensive_duelist_desc.to_string() }
        ];
        let _defensive_duelist = Feat::new(
            "Defensive Duelist".to_string(),
            defensive_duelist_desc.to_string(),
            Some(feats::FeatPrereq::MinStat { s: vec![(Stat::Dexterity, 13)], logical_or: false }),
            defensive_duelist_effects
        );

        // TODO Dual Wielder

        let dungeon_delver_desc = "Alert to the hidden traps and secret doors found in many dungeons, you gain the follow in g benefits:\n• You have advantage on Wisdom (Perception) and Intelligence (Investigation) checks made to detect the presence of secret doors.\n• You have advantage on saving throws made to avoid or resist traps.\n• You have resistance to the damage dealt by traps.\n• You can search for traps while traveling at a normal pace, instead of only at a slow pace.";
        let dungeon_delver_effects = vec![
            FeatEffect::SkillAdv { 
                s: Skill::Perception, 
                context: Some("to detect the presence of secret doors".to_string()) 
            },
            FeatEffect::SkillAdv { 
                s: Skill::Investigation, 
                context: Some("to detect the presence of secret doors".to_string())
            },
            FeatEffect::SaveAdvAgainst { context: "saving throws made to avoid or resist traps".to_string() },
            FeatEffect::ResistanceAgainst { context: "damage dealt by traps".to_string() }
        ];
        let _dungeon_delver = Feat::new(
            "Dungeon Delver".to_string(),
            dungeon_delver_desc.to_string(),
            None,
            dungeon_delver_effects
        );

        let durable_desc = "Hardy and resilient, you gain the follow in g benefits:\n• Increase your Constitution score by 1, to a maximum of 20.\n• When you roll a Hit Die to regain hit points, the minimum number of hit points you regain from the roll equals twice your Constitution modifier (minimum of 2).";
        let durable_effects = vec![
            FeatEffect::IncreaseStat { options: vec![Stat::Constitution], by: 1, max: 20 }
        ];
        let _durable = Feat::new(
            "Durable".to_string(),
            durable_desc.to_string(),
            None,
            durable_effects
        );

        let elemental_adept_desc = "When you gain this feat, choose one of the following damage types: acid, cold, fire, lightning, or thunder. Spells you cast ignore resistance to damage of the chosen type. In addition, when you roll damage for a spell you cast that deals dam age of that type, you can treat any 1 on a damage die as a 2. You can select this feat multiple times. Each time you do so, you must choose a different damage type.";
        let elemental_adept_effects = vec![
            FeatEffect::Choice(vec![
                "acid".to_string(), 
                "cold".to_string(), 
                "fire".to_string(), 
                "lightning".to_string(), 
                "thunder".to_string()
            ])
        ];
        let _elemental_adept = Feat::new(
            "Elemental Adept".to_string(), 
            elemental_adept_desc.to_string(),
            Some(feats::FeatPrereq::CastASpell),
            elemental_adept_effects
        );

        let grappler_desc = "You’ve developed the skills necessary to hold your own in close-quarters grappling. You gain the following benefits:\n• You have advantage on attack rolls against a creature you are grappling.\n• You can use your action to try to pin a creature grappled by you. To do so, make another grapple check. If you succeed , you and the creature are both restrained until the grapple ends.\n• Creatures that are one size larger than you don’t automatically succeed on checks to escape your grapple.";
        let grappler_effects = vec![
            FeatEffect::GrantAction { 
                time: basic::ActivationTime::Action, 
                action_desc: "You can use your action to try to pin a creature grappled by you. To do so, make another grapple check. If you succeed , you and the creature are both restrained until the grapple ends."
                    .to_string()
                }
        ];
        let _grappler = Feat::new(
            "Grappler".to_string(), 
            grappler_desc.to_string(), 
            Some(feats::FeatPrereq::MinStat { s: vec![(Stat::Strength, 13)], logical_or: false }), 
            grappler_effects
        );

        let gwm_desc = "You’ve learned to put the weight of a weapon to your advantage, letting its momentumem power your strikes. You gain the following benefits:
    • On your turn, when you score a critical hit with a melee weapon or reduce a creature to 0 hit points with one, you can make one melee weapon attack as a bonus action.
    • Before you make a melee attack with a heavy weapon that you are proficient with, you can choose to take a -5 penalty to the attack roll. If the attack hits, you add +10 to the attack’s damage.";
        let gwm_effects = vec![
            FeatEffect::GrantAction { 
                time: basic::ActivationTime::BonusAction, 
                action_desc: "On your turn, when you score a critical hit with a melee weapon or reduce a creature to 0 hit points with one, you can make one melee weapon attack as a bonus action"
                    .to_string()
            }
        ];
        let _gwm = Feat::new(
            "Great Weapon Master".to_string(), 
            gwm_desc.to_string(), 
            None, 
            gwm_effects
        );
}