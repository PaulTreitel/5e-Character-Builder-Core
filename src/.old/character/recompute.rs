use super::Character;



impl Character {
    pub fn recompute(&mut self) -> () {
        todo!("Recompute character sheet")
    }

    pub fn recompute_stats(&mut self) -> () {
        todo!("Recompute stats")
    }

    pub fn recompute_proficiencies(&mut self) -> () {
        self.recompute_skills();
        self.recompute_saves();
        self.recompute_armors();
        self.recompute_weapons();
        self.recompute_langs();
        self.recompute_tools();
        self.recompute_initiative();
    }

    pub fn recompute_creature_stats(&mut self) -> () {
        todo!("Recompute creature type and size")
    }

    pub fn recompute_speeds(&mut self) -> () {
        todo!("Recompute speeds")
    }

    pub fn recompute_senses(&mut self) -> () {
        todo!("Recompute senses")
    }

    pub fn recompute_health(&mut self) -> () {
        todo!("Recompute health")
    }

    pub fn recompute_defenses(&mut self) -> () {
        todo!("Recompute AC, resistances, vulnerabilities, and immunities")
    }

    fn recompute_skills(&mut self) -> () {
        todo!("Recompute skill proficiencies")
    }

    fn recompute_saves(&mut self) -> () {
        todo!("Recompute save proficiencies")
    }

    fn recompute_armors(&mut self) -> () {
        todo!("Recompute armor proficiencies")
    }

    fn recompute_weapons(&mut self) -> () {
        todo!("Recompute weapon proficiencies")
    }
    
    fn recompute_langs(&mut self) -> () {
        todo!("Recompute language proficiencies")
    }

    fn recompute_tools(&mut self) -> () {
        todo!("Recompute tool proficiencies")
    }

    fn recompute_initiative(&mut self) -> () {
        todo!("Recompute initiative proficiency")
    }
}