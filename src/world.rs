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
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use map::tiles::Map;
use map::generators::*;
use creatures::Creature;
use utils::*;

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
            30,
            30,
            Position { level: 0, x: 5, y: 5 },
            0)));
        let mut world = World {
            main_character: main_character.clone(),
            levels: vec![RefCell::new(level)],
            creatures: vec![],
        };
        world.add_creature(main_character);
        world
    }

    pub fn get_level(&self, level: usize) -> &MapRef {
        &self.levels[level]
    }

    fn add_creature(&mut self, creature: Rc<CreatureRef>) {
        let position = creature.borrow().position();
        let Map(ref mut map) = *self.levels[position.level].borrow_mut();
        map[position.x][position.y].creature = Some(Rc::downgrade(&creature));
        self.creatures.push(creature);
    }

    pub fn main_character(&self) -> Weak<CreatureRef> {
        Rc::downgrade(&self.main_character)
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
