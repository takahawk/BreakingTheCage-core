/*
    MIT License

    Copyright (c) 2017 Dan Hawk

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
    */
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use map::tiles::Map;
use map::tiles::Tiles;
use map::generators::*;
use creatures::Creature;
use utils::*;

const MAIN_CHARACTER_ID: usize = 0;

pub type CreatureRef = RefCell<Creature>;
pub type MapRef = RefCell<Map>;

/// Main entity holding entire game state with levels, creatures, player character etc.
pub struct World {
    main_character: Rc<CreatureRef>,
    levels: Vec<MapRef>,
    creatures: Vec<Rc<CreatureRef>>,
}

impl World {

    /// Creates new world with generating levels for it
    pub fn new() -> World {
        // current logic is stub, used only for debugging and testing
        let level = SimpleBoxGenerator::new(20, 20).generate();

        let main_character = Rc::new(RefCell::new(Creature::demon(
            String::from("Very Evil Demon"),
            Position { level: 0, x: 5, y: 5 },
            30,
            30)));
        let mut world = World {
            main_character: main_character.clone(),
            levels: vec![RefCell::new(level)],
            creatures: vec![main_character],
        };

        world
    }

    pub fn get_level(&self, level: usize) -> &MapRef {
        &self.levels[level]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_new_world() {
        // World created without panicking
        World::new();
    }
}