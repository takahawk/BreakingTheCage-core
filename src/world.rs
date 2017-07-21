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
use creatures::Position;

const MAIN_CHARACTER_ID: usize = 0;

pub type CreatureRef = RefCell<Creature>;

/// Main entity holding entire game state with levels, creatures, player character etc.
pub struct World {
    current_level: usize,
    levels: Vec<Map>,
    creatures: HashMap<usize, Rc<CreatureRef>>,
    next_id: usize,
}

impl World {

    /// Creates new world with generating levels for it
    pub fn new() -> World {
        // current logic is stub, used only for debugging and testing
        let level = SimpleBoxGenerator::new(20, 20).generate();
        let mut world = World {
            current_level: 0,
            levels: vec![level],
            creatures: HashMap::new(),
            next_id: MAIN_CHARACTER_ID,
        };

        // adding main character
        world.add_creature(
            Creature::demon(
                String::from("Very Evil Demon"),
                Position { level: 0, x: 5, y: 5 },
                30,
                30));
        world
    }

    pub fn level_map(&self, level: usize) -> &Map {
        &self.levels[level]
    }

    /// Returns weak reference to a creature reference
    /// On the moment of use that creature ref may be dead, so I found it reasonable
    pub fn get_creature(&self, id: usize) -> Option<Weak<CreatureRef>> {
        self.creatures.get(&id).map(|creature| Rc::downgrade(creature))
    }

    fn add_creature(&mut self, creature: Creature) {
        self.creatures.insert(self.next_id, Rc::new(RefCell::new(creature)));
        self.next_id += 1;
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