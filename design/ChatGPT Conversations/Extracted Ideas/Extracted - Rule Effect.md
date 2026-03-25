# Links

### Part 1
1. [[Part 1 Raw#5. Effect System (Critical for Homebrew)|Basic effects and requirements]]
2. [[Part 1 Raw#4. Effects (Rule Engine Layer)|Another]]
3. [[Part 1 Raw#10. How Features Fit Into This|EffectApplier]]
4. [[Part 1 Raw#5. Effects (Rule Engine Core)|Another enum]]

### Part 2
1. [[Part 2 Raw#7. Effect System (Rule Engine Core)|Effect enum]]
2. [[Part 2 Raw#Why Data-Driven Rules Are Better|Effect enum and JSON example]]
3. [[Part 2 Raw#5. Magic Items and Rule Effects|Magic item rule effect YAML]]

### Part 3
1. [[Part 3 Raw#Effect Model|Active Effect]]
2. [[Part 3 Raw#Item Effects → Rule Graph|Item effects]]

### Part 4
1. [[Part 4 Raw#1. Stat Paths (Structured Stat Namespace)|Effect YAML examples]]
2. [[Part 4 Raw#2. Applying Upcasting in the Rule Graph|Spells generating effects]]
3. [[Part 4 Raw#7. Rule Effects (Core Data-Driven Mechanism)|Effect enum and modifiers]]
4. [[Part 4 Raw#Case 1 — Polymorph / Wild Shape|Polymorph, advantage, and carrying capacity]]
5. [[Part 4 Raw#Please Do [Show Rule Provenance Tracking]|Rule effect provenance tracking]]
6. [[Part 4 Raw#2.2 Rule Effect Producers|Rule providers]]

### Part 5
1. [[Part 5 Raw#Structuring effects around their target stats]]
2. [[Part 5 Raw#1. Rule Compilation|Turning effects into compiled rules]]

### Part 6
1. [[Part 6 Raw#6. Rule Effects|More comprehensive enum and structs]]
2. [[Part 6 Raw#Please do [Show concrete YAML rules for rage, sneak attack, spellcasting, and 2024 wildshape].|YAML feature examples with effects]]

# Common Data Structure Elements

### enum Effect
- grant something
	- proficiencies, features, senses
	- spell, spellcasting itself, spell list
	- choice-based feature
	- resource (eg CD, rages)
	- resistance/vulnerability
	- an action/bonus action/reaction
	- wildshape form
- set or override a value
	- speed, ability score
- bonus/modify a value, including constant values and dice
	- stat, speed, ability score
	- bonuses to attacks/DCs/AC/saves
- advantage/disadvantage
- apply a condition (literal and broader conditions like raging)

### struct ActiveEffect
- effect ID
- rule effects

### enum RollMode
- normal, advantage, disadvantage

### enum RuleSource
- type of rule source and the source's ID

### struct RuleIndex
- map from stats to the rule effects that effect them

### struct CompiledRule
- target stat
- function pointer to apply the rule
- the source of the rule

### traits EffectApplier and RuleProvider
- trait for things that can apply a rule effect to the character sheet
- trait for things that can provide rules (i.e. `collect_rules()`)

# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
enum Effect {
    ModifyStat { stat: Stat, value: i32 },
    GrantSkill { skill: Skill },
    AddSpell { spell: String },
    SetSpeed { value: u32 },
}

enum Requirement {
    Class(String),
    Level(u8),
    StatAbove { stat: Stat, value: u8 },
}
```

#### #2
```Rust
pub enum Effect {
    AbilityScoreBonus {
        ability: Ability,
        amount: i32,
    },

    GrantProficiency {
        proficiency: Proficiency,
    },

    GrantFeature {
        feature: FeatureId,
    },

    IncreaseSpeed {
        amount: u32,
    },

    Spellcasting {
        ability: Ability,
        spell_list: SpellListId,
    },
}
```

Example
```JSON
{
  "name": "Frostborn",
  "effects": [
    { "type": "resistance", "damage": "cold" },
    { "type": "spell", "spell": "armor_of_agathys" }
  ]
}
```

#### #3
```Rust
trait EffectApplier {
    fn apply(&self, sheet: &mut CharacterSheet);
}
```

#### #4
```Rust
pub enum Effect {
    AbilityBonus {
        ability: Ability,
        amount: i32,
    },

    SetSpeed {
        speed: u32,
    },

    GrantProficiency {
        proficiency: String,
    },

    GrantFeature {
        feature: String,
    },
}
```

### Part 2

#### #1
```Rust
pub enum Effect {  
  
    AbilityBonus {  
        ability: Ability,  
        amount: i32  
    },  
  
    SetSpeed {  
        speed: u32  
    },  
  
    GrantProficiency {  
        proficiency: Proficiency  
    },  
  
    GrantFeature {  
        feature: FeatureId  
    },  
  
    EnableSpellcasting {  
        ability: Ability,  
        spell_list: SpellListId  
    }  
}
```

#### #2
```Rust
pub enum Effect {  
    AbilityBonus { ability: Ability, amount: i32 },  
    GrantProficiency { proficiency: Proficiency },  
    SetSpeed { speed: u32 },  
    GrantFeature { feature: FeatureId },  
}
```

```JSON
{  
  "id": "elf_dex_bonus",  
  "effects": [  
    {  
      "type": "AbilityBonus",  
      "ability": "Dexterity",  
      "amount": 2  
    }  
  ]  
}
```

#### #3
```Rust
pub enum Effect {  
  
    ModifyAbility {  
        ability: Ability,  
        amount: i32,  
    },  
  
    GrantProficiency {  
        proficiency: Proficiency,  
    },  
  
}
```

#### #4
```YAML
id: belt_giant_strength  
name: Belt of Hill Giant Strength  
kind: magic  
attunement: true  
  
rules:  
  - type: set_ability_score  
    ability: strength  
    value: 21
```

```
id: longsword_plus_1  
name: Longsword +1  
  
rules:  
  - type: bonus  
    target: attack_roll  
    value: 1  
  - type: bonus  
    target: damage_roll  
    value: 1
```

### Part 3

#### #1
```Rust
pub struct ActiveEffect {  
    pub id: EffectId,  
    pub fragments: Vec<RuleFragment>,  
    pub duration: Duration,  
}
```

#### #2
```Rust
struct ItemEffect {  
    modifier: Modifier,  
}

// example
Modifier {  
    target: Stat::ArmorClass,  
    value: 1,  
}
```

### Part 4

#### #1
```YAML
effect:  
  type: modifier  
  target: defense.armor_class  
  value: 1
```

GWM
```YAML
effect:  
  operation: add  
  target: combat.damage_bonus  
  value: 10  
  
condition:  
  weapon_tag: heavy
```

Ring of Protection
```YAML
effects:  
  - operation: add  
    target: defense.armor_class  
    value: 1  
  
  - operation: add  
    target: defense.saving_throw.all  
    value: 1
```

Bless
```YAML
effects:  
  - operation: add_dice  
    target: combat.attack_roll  
    dice: 1d4  
  
  - operation: add_dice  
    target: defense.saving_throw  
    dice: 1d4
```

Poisoned
```YAML
effects:  
  - operation: disadvantage  
    target: combat.attack_roll
```

#### #3
```Rust
pub enum RuleEffect {  
    Modifier(ModifierRule),  
    SetValue(SetRule),  
    Override(OverrideRule),  
    GrantCondition(ConditionId),  
}

pub struct ModifierRule {  
    pub target: Stat,  
    pub value: i32,  
    pub bonus_type: BonusType,  
}  
  
pub struct SetRule {  
    pub target: Stat,  
    pub value: i32,  
}  
  
pub struct OverrideRule {  
    pub target: Stat,  
    pub value: i32,  
}

pub enum BonusType {  
    Untyped,  
    Armor,  
    Shield,  
    Enhancement,  
    Circumstance,  
}
```

#### #4
```YAML
id: polymorph  
  
effects:  
  - override:  
      stat: Strength  
      value: 19  
  
  - override:  
      stat: Dexterity  
      value: 14  
  
  - override:  
      stat: Speed  
      value: 40
```

```Rust
pub enum RollMode {  
    Normal,  
    Advantage,  
    Disadvantage,  
}

pub enum RuleEffect {  
    ...  
    RollModeModifier {  
        target: RollStat,  
        mode: RollMode,  
    }  
}

ModifierRule {  
    target: Stat::CarryingCapacity,  
    value: 2,  
    bonus_type: BonusType::Multiplier,  
}
```

#### #5
```Rust
pub struct ModifierRule {  
    pub target: Stat,  
    pub value: i32,  
    pub bonus_type: BonusType,  
    pub source: RuleSource,  
}

pub enum RuleSource {  
    Race(RaceId),  
    ClassFeature(ClassFeatureId),  
    Feat(FeatId),  
    Item(ItemInstanceId),  
    Spell(SpellId),  
    Condition(ConditionId),  
    System(SystemSource),  
}

pub enum SystemSource {  
    AbilityModifier,  
    Encumbrance,  
    BaseRule,  
}

pub struct OverrideRule {  
    pub target: Stat,  
    pub value: i32,  
    pub source: RuleSource,  
}
```

#### #6
```Rust
pub trait RuleProvider {  
    fn collect_rules(  
        &self,  
        character: &Character,  
        db: &RulesDatabase  
    ) -> Vec<RuleEffect>;  
}
```

### Part 5

#### #1
```Rust
pub struct RuleIndex {  
    pub by_stat: HashMap<Stat, Vec<RuleEffect>>,  
}
```

#### #2
```Rust
pub struct CompiledRule {  
    pub target: Stat,  
    pub apply: fn(&mut RuleNode),  
    pub source: RuleSource,  
}
```

### Part 6

#### #1
```Rust
pub enum RuleEffect {  
    Modifier(Modifier),  
    GrantRule(GrantRule),  
    GrantAction(GrantAction),  
    GrantResource(GrantResource),  
    GrantProficiency(GrantProficiency),  
    GrantCondition(GrantCondition),  
    GrantSpell(GrantSpell),  
    GrantFeature(GrantFeature),  
    GrantChoice(GrantChoice),  
    GrantFormSelection(GrantFormSelection),  
    ResourceRecovery(ResourceRecoveryRule),  
}

pub struct Modifier {  
    pub target: ModifierTarget,  
    pub operation: ModifierOperation,  
    pub value: ModifierValue,  
    pub stacking: ModifierStacking,  
}

pub enum ModifierTarget {  
    Stat(StatId),  
    AttackRoll,  
    DamageRoll,  
    ArmorClass,  
    Speed(SpeedId),  
}

pub struct GrantRule {  
    pub rule: RuleId  
}

pub struct GrantAction {  
    pub action: ActionId  
}

pub struct GrantResource {  
    pub resource: ResourceId  
}

pub struct ResourceRecoveryRule {  
    pub resource: ResourceId,  
    pub recovery: ResourceRecovery  
}

pub struct GrantProficiency {  
    pub proficiency: ProficiencyStatId  
}

pub struct GrantCondition {  
    pub condition: ConditionId  
}

pub struct GrantSpell {  
    pub spell: SpellId  
}

pub struct GrantFeature {  
    pub feature_id: FeatureId  
}

pub struct GrantChoice {  
    pub choice: ChoiceId  
}

pub struct GrantFormSelection {  
    pub count: u8,  
    pub filter: WildshapeFormFilter  
}

// examples

Rule {  
  id: RuleId("darkvision"),  
  source: RuleSource::SpeciesTrait(SpeciesId::Human),  
  phase: RulePhase::FeatureModifiers,  
  filter: None,  
  effect: RuleEffect::Modifier(...)  
}

Rule {  
  source: RuleSource::ClassFeature(ClassId::Fighter),  
  phase: RulePhase::Actions,  
  effect: RuleEffect::GrantAction(...)  
}

Rule {  
  source: RuleSource::ClassFeature(ClassId::Fighter),  
  phase: RulePhase::FeatureModifiers,  
  effect: RuleEffect::GrantChoice(...)  
}
```

#### #2
Rage
```YAML
id: barbarian_rage  
source:  
  type: class_feature  
  class: barbarian  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_resource  
      resource: rage  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: rage_activate  
  
  - phase: conditions  
    filter:  
      condition_active: raging  
    effect:  
      type: modifier  
      target: damage_roll  
      operation: add  
      value: "@barbarian.rage_damage"  
  
  - phase: conditions  
    filter:  
      condition_active: raging  
    effect:  
      type: modifier  
      target: defense  
      stat: damage_resistance  
      value: [bludgeoning, piercing, slashing]
```

Sneak Attack
```YAML
id: rogue_sneak_attack  
source:  
  type: class_feature  
  class: rogue  
  
rules:  
  
  - phase: actions  
    filter:  
      action_type: attack  
      weapon_property: finesse_or_ranged  
  
    effect:  
      type: modifier  
      target: damage_roll  
      operation: add_dice  
      value: "@rogue.sneak_attack_dice"
```

Wizard spellcasting
```YAML
id: wizard_spellcasting  
source:  
  type: class_feature  
  class: wizard  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_resource  
      resource: spell_slots  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_spell_list  
      spell_list: wizard  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_choice  
      choice:  
        type: spell_selection  
        list: wizard  
        count: "@wizard.starting_spells"
```

Fighting Style Choice
```YAML
id: fighter_fighting_style  
source:  
  type: class_feature  
  class: fighter  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_choice  
      choice:  
        type: fighting_style  
        options:  
          - archery  
          - defense  
          - dueling  
          - great_weapon_fighting
```

Fighting Style
```YAML
id: fighting_style_archery  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: modifier  
      target: attack_roll  
      weapon_category: ranged  
      operation: add  
      value: 2
```

Magic Missile
```YAML
id: magic_missile  
  
level: 1  
school: evocation  
  
rules:  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: cast_magic_missile  
  
scaling:  
  
  upcast:  
  
    - level: 2  
      effect:  
        missiles: +1  
  
    - level: 3  
      effect:  
        missiles: +2
```
alt
```YAML
scaling:  
  per_slot_level:  
    missiles: +1
```

Wildshape
```YAML
id: druid_wildshape  
source:  
  type: class_feature  
  class: druid  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_resource  
      resource: wildshape_uses  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: wildshape_transform  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_form_selection  
      system: wildshape  
      count: "@druid.wildshape_known_forms"  
      filter:  
        creature_type: beast  
        max_cr: "@druid.wildshape_cr"
```

Weapon Mastery Choice
```YAML
id: fighter_weapon_mastery  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_choice  
      choice:  
        type: weapon_mastery  
        count: "@fighter.mastery_count"
```

Weapon Mastery
```YAML
id: weapon_mastery_cleave  
  
rules:  
  
  - phase: actions  
    filter:  
      weapon_property: heavy  
    effect:  
      type: grant_action  
      action: cleave_attack
```

Species Feature
```YAML
id: species_darkvision  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: modifier  
      target: sense  
      sense_type: darkvision  
      value: 60
```

Feat
```YAML
id: feat_sharpshooter  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: modifier  
      target: ranged_attack_cover  
      operation: ignore  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: sharpshooter_power_attack
```

# (Pseudo)Code

### Part 1

#### #1
```Rust
fn apply_effect(character: &mut Character, effect: &Effect) {
    match effect {
        Effect::ModifyStat { stat, value } => {
            character.modify_stat(*stat, *value);
        }
        Effect::GrantSkill { skill } => {
            character.skills.insert(*skill);
        }
    }
}

fn check_requirement(character: &Character, req: &Requirement) -> bool
```

#### #4
```Rust
pub fn apply_effects(
    base: &AbilityScores,
    features: &[String],
    rules: &Rules,
) -> CharacterSheet {

    let mut sheet = CharacterSheet {
        abilities: base.clone(),
        speed: 30,
        proficiencies: HashSet::new(),
        features: features.to_vec(),
    };

    for feature_id in features {
        if let Some(feature) = rules.features.get(feature_id) {
            for effect in &feature.effects {
                apply_effect(effect, &mut sheet);
            }
        }
    }

    sheet
}

fn apply_effect(effect: &Effect, sheet: &mut CharacterSheet) {
    match effect {

        Effect::AbilityBonus { ability, amount } => {
            sheet.abilities.add(*ability, *amount);
        }

        Effect::SetSpeed { speed } => {
            sheet.speed = *speed;
        }

        Effect::GrantProficiency { proficiency } => {
            sheet.proficiencies.insert(proficiency.clone());
        }

        Effect::GrantFeature { feature } => {
            sheet.features.push(feature.clone());
        }
    }
}
```

### Part 2

#### #1
```Rust
pub fn apply_effects(  
    base: &AbilityScores,  
    features: &[FeatureId],  
    rules: &Rules  
) -> CharacterSheet {  
  
    let mut sheet = CharacterSheet {  
        abilities: base.clone(),  
        proficiency_bonus: 2,  
        max_hp: 0,  
        speed: 30,  
        proficiencies: HashSet::new(),  
        features: features.to_vec(),  
        spellcasting: None  
    };  
  
    for feature_id in features {  
  
        let feature = &rules.features[feature_id];  
  
        for effect in &feature.effects {  
            apply_effect(effect, &mut sheet);  
        }  
    }  
  
    sheet  
}

fn apply_effect(effect: &Effect, sheet: &mut CharacterSheet) {  
  
    match effect {  
  
        Effect::AbilityBonus { ability, amount } => {  
            sheet.abilities.add(*ability, *amount);  
        }  
  
        Effect::SetSpeed { speed } => {  
            sheet.speed = *speed;  
        }  
  
        Effect::GrantProficiency { proficiency } => {  
            sheet.proficiencies.insert(*proficiency);  
        }  
  
        Effect::EnableSpellcasting { ability, spell_list } => {  
  
            sheet.spellcasting = Some(Spellcasting {  
                ability: *ability,  
                spell_list: *spell_list,  
                slots: SpellSlotTable::default(),  
            });  
        }  
  
        _ => {}  
    }  
}
```

#### #3
```Rust
for effect in effects {  
    match effect {  
  
        Effect::ModifyAbility { ability, amount } => {  
            let scores = store.get_mut::<AbilityScores>().unwrap();  
            scores.add(*ability, *amount);  
        }  
  
        _ => {}  
    }  
}
```

### Part 3

#### #2
```Rust
fn gather_item_modifiers(world: &World) -> Vec<Modifier> {  
    let mut mods = Vec::new();  
  
    for (entity, item) in &world.items {  
        let def = get_definition(item.def);  
  
        if def.attunement_required && !world.attuned.contains_key(entity) {  
            continue;  
        }  
  
        for effect in &def.effects {  
            mods.push(effect.modifier.clone());  
        }  
    }  
  
    mods  
}
```

### Part 4

#### #2
```Rust
fn build_spell_effects(  
    spell: &SpellDefinition,  
    slot_level: u8  
) -> Vec<RuleEffect> {  
    let mut effects = spell.effects.clone();  
  
    for up in &spell.upcast {  
        if up.slot_level <= slot_level {  
            effects.extend(up.additional_effects.clone());  
        }  
    }  
  
    effects  
}
```

#### #3
```Rust
pub fn collect_rule_effects(  
    character: &Character,  
    db: &RulesDatabase,  
) -> Vec<RuleEffect> {  
    let mut effects = Vec::new();  
  
    effects.extend(race_rules(character, db));  
    effects.extend(class_rules(character, db));  
    effects.extend(feat_rules(character, db));  
    effects.extend(item_rules(character, db));  
    effects.extend(condition_rules(character, db));  
    effects.extend(spell_rules(character, db));  
  
    effects  
}
```

#### #4
```Rust
pub fn combine_roll_modes(modes: &[RollMode]) -> RollMode {  
  
    let has_adv = modes.contains(&RollMode::Advantage);  
    let has_dis = modes.contains(&RollMode::Disadvantage);  
  
    match (has_adv, has_dis) {  
        (true, true) => RollMode::Normal,  
        (true, false) => RollMode::Advantage,  
        (false, true) => RollMode::Disadvantage,  
        _ => RollMode::Normal,  
    }  
}
```

#### #6
```Rust
// example
impl RuleProvider for ItemInstance {  
    fn collect_rules(&self, _character: &Character, db: &RulesDatabase) -> Vec<RuleEffect> {  
        db.items[&self.definition].rules.clone()  
    }  
}
```

### Part 5

#### #1
```Rust
impl RuleIndex {  
    pub fn build(effects: Vec<RuleEffect>) -> Self {  
        let mut by_stat = HashMap::new();  
  
        for effect in effects {  
            let stat = effect.target_stat();  
  
            by_stat.entry(stat)  
                .or_insert_with(Vec::new)  
                .push(effect);  
        }  
  
        Self { by_stat }  
    }  
}
```

#### #2
```Rust
pub fn compile_rule(effect: RuleEffect) -> CompiledRule {  
    match effect {  
        RuleEffect::Modifier(rule) => {  
            CompiledRule {  
                target: rule.target,  
                apply: move |node| {  
                    node.modifiers.push(rule.clone());  
                },  
                source: rule.source,  
            }  
        }  
  
        RuleEffect::Override(rule) => {  
            CompiledRule {  
                target: rule.target,  
                apply: move |node| {  
                    node.overrides.push(rule.clone());  
                },  
                source: rule.source,  
            }  
        }  
    }  
}
```

