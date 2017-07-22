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
use std::rc::Weak;

use utils::*;
use actions::action;
use actions::action::*;
use creatures::*;
use world::*;
use map::tiles::*;

pub struct Move {
    creature: Weak<CreatureRef>,
    direction: Direction,
}

pub enum MoveError {
    OutOfBounds,
    TileIsOccupied,
    TileIsImpassable,
}

impl Applicable for Move {
    fn apply(&self, world: &mut World) -> action::Result {
        if let Some(creature) = self.creature.upgrade() {
            let mut creature = creature.borrow_mut();
            let Map(ref mut map) = *world.get_level(creature.position().level).borrow_mut();
            let new_pos = (creature.position() + self.direction).ok_or(MoveError::OutOfBounds)?;

            if map.len() <= new_pos.x || map[0].len() <= new_pos.y {
                return Err(ActionError::MoveError(MoveError::OutOfBounds))
            }



            {
                // first: set creature as current to new position
                let ref mut tile = map[new_pos.x][new_pos.y];

                if let Some(_) = tile.creature {
                    return Err(ActionError::MoveError(MoveError::TileIsOccupied))
                }

                if !tile.is_passable() {
                    return Err(ActionError::MoveError(MoveError::TileIsImpassable))
                }
                tile.creature = Some(self.creature.clone()); // weak ref
            }

            {
                // second: remove creature ref from prev position
                let ref mut tile = map[creature.position().x][creature.position().y];
                tile.creature = None;
            }

            // finally - set new position to creature entity
            creature.set_position(new_pos);

            Ok(())
        } else {
            Err(ActionError::SubjectIsDead)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_move() {
        let mut world = World::new();
    }
}