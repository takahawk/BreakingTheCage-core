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

/// Represents atomic motion of a creature on the map
pub struct Move {
    creature: Weak<CreatureRef>,
    direction: Direction,
}

impl Move {
    pub(crate) fn new(creature: Weak<CreatureRef>, direction: Direction) -> Move {
        Move {
            creature: creature,
            direction: direction,
        }
    }
}

impl Action for Move {

    fn apply(&self, world: &mut World) -> action::Result {
        if let Some(creature) = self.creature.upgrade() {
            let mut creature = creature.borrow_mut();
            let Map(ref mut map) = *world.get_level(creature.position().level).borrow_mut();
            let new_pos = (creature.position() + self.direction)
                .ok_or(ActionError::OutOfBounds {
                    position: None,
                    width: map.len(),
                    height: map[0].len(),
                })?;

            if map.len() <= new_pos.x || map[0].len() <= new_pos.y {
                return Err(ActionError::OutOfBounds {
                    position: Some(new_pos),
                    width: map.len(),
                    height: map[0].len(),
                })
            }



            {
                // first: set creature as current to new position
                let ref mut tile = map[new_pos.x][new_pos.y];

                if let Some(ref creature) = tile.creature {
                    return Err(ActionError::TileIsOccupied(creature.clone())) // weak ref
                }

                if !tile.is_passable() {
                    return Err(ActionError::TileIsImpassable(new_pos))
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

    fn cost(&self) -> u32 {
        match self.creature.upgrade() {
            Some(creature) => 100, // TODO: replace hardcode with more creature-specific calculation
            None => 0,
        }
    }

    fn actor(&self) -> Weak<CreatureRef> {
        self.creature.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (World, Weak<CreatureRef>, Position) {
        let mut world = World::new();
        let character = world.main_character();
        let mut position = match character.upgrade() {
            Some(ref character) => character.borrow().position(),
            None => panic!("No main character!")
        };
        (world, character, position)
    }

    #[test]
    fn valid_move() {
        let (mut world, character, pos) = setup();
        let action = Move::new(character.clone(), Direction::Right);
        action.apply(&mut world).expect("Valid action return error!");
        let actual = character.upgrade().map(|a| a.borrow().position()).unwrap();
        let expected = (pos + Direction::Right).unwrap();
        assert_eq!(actual, expected);
    }
}