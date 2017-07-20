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
use super::map::tiles::Map;

use self::Creature::*;

pub struct Points {
    current: u32,
    max: u32,
}

pub enum Creature {
    Human {
        name: String,
        position: Position,
        health: Points,
        mana: Points,
        /*skills: Vec<Skill>*/},
    Demon {
        name: String,
        position: Position,
        health: Points,
        mana: Points,
        /*humane_skills: Vec<Skill>,*/
        /*demon_skills: Vec<Skill>,*/
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub level: u32,
    pub x: u32,
    pub y: u32,
}

impl Creature {
    pub fn demon(name: String, position: Position, health: u32, mana: u32) -> Creature {
        Creature::Demon {
            name: name,
            position: position,
            health: Points { current: health, max: health },
            mana: Points { current: mana, max: mana },
        }
    }

    pub fn position(&self) -> Position {
        match *self {
            Demon { position, .. } => position,
            Human { position, .. } => position,
        }
    }
}