use crate::spine::{Spine, SpineDemo};
use macroquad::prelude::*;
use std::{collections::HashMap, panic};

pub struct Character {
    pub name: String,
    spine: Spine,
    texture: Texture2D,
    pub meshs: Vec<Mesh>,
}

impl Character {
    pub fn update(&mut self, delta_time: f32) {
        self.spine.controller.update(delta_time);
        self.meshs =
            self.spine
                .get_mesh(self.texture.clone(), screen_width() / 2.0, screen_height());
    }
}

pub struct CharacterManager {
    pub characters: HashMap<String, Character>,
}

// CharacterManager 可以获取 TextureLoader 的实现
impl CharacterManager {
    pub async fn register(
        &mut self,
        name: String,
        texture: Texture2D,
        spinedemo: SpineDemo,
    ) -> Result<(), String> {
        let texture = texture.clone();
        let mut spine = Spine::load(spinedemo);
        let meshs = spine.get_mesh(texture.clone(), screen_width() / 2.0, screen_height());
        self.characters.insert(
            name.clone(),
            Character {
                name: name,
                texture: texture.clone(),
                spine: spine,
                meshs: meshs,
            },
        );
        Ok(())
    }
}

impl CharacterManager {
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
        }
    }
    pub fn update(&mut self, name: &str, delta_time: f32) -> Result<(), String> {
        if let Some(character) = self.characters.get_mut(name) {
            character.update(delta_time);
            Ok(())
        } else {
            Err(format!("Character {} not found", name))
        }
    }

    pub fn get_meshs(&self, name: &str) -> &Vec<Mesh> {
        &self
            .characters
            .get(name)
            .unwrap_or_else(|| panic!("can not found character {}.", name))
            .meshs
    }
    pub fn get_decrible(&self, name: &str) -> String {
        self.characters.get(name).unwrap().name.clone()
    }
}

mod test {
    #[test]
    fn test() {}
}
