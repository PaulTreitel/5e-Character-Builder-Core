use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CharDescription {
    pub age: u32,
    pub height: String,
    pub weight: String,
    pub eye_color: String,
    pub hair_color: String,
    pub skin_color: String,
    pub general_appearance: String,
    pub backstory: String,
    pub allies_organizations: String,
}

impl CharDescription {
    pub fn new(
        age: u32,
        height: String,
        weight: String,
        eye_color: String,
        hair_color: String,
        skin_color: String,
        appearance: String,
        backstory: String,
        allies_organizations: String
    ) -> Self {
        CharDescription { 
            age, 
            height, 
            weight, 
            eye_color, 
            hair_color, 
            skin_color, 
            general_appearance: appearance,
            backstory, 
            allies_organizations
        }
    }
}