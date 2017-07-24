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
use std::cmp;
use utils::*;
use map::tiles::Map;

const MAX_DEMONICITY: u32 = 100;

#[derive(Debug)]
pub struct Points {
    current: u32,
    max: u32,
}

#[derive(Debug)]
pub struct Creature {
    name: String,
    position: Position,
    health: Points,
    mana: Points,
    creature_type: CreatureType,
}

#[derive(Debug)]
pub enum CreatureType {
    Human,
    Demon { demonicity: Points },
}


impl Creature {
    pub(crate) fn demon(name: String,
                 health: u32,
                 mana: u32,
                 position: Position,
                 initial_demonicity: u32) -> Creature {
        Creature {
            name: name,
            position: position,
            mana: Points { current: mana, max: mana },
            health: Points { current: health, max: health },
            creature_type: CreatureType::Demon {
                demonicity: Points {
                    current: cmp::min(initial_demonicity, MAX_DEMONICITY),
                    max: MAX_DEMONICITY
                },
            },
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub(crate) fn set_position(&mut self, position: Position) {
        self.position = position
    }
}
