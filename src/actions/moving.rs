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
use world::*;
use map::tiles::*;
use super::*;

pub(super) fn is_move_valid(
    world: &World,
    creature: &Weak<CreatureRef>,
    direction: Direction) -> Result {
    if let Some(creature) = creature.upgrade() {
        let creature = creature.borrow();
        let Map(ref map) = *world.get_level(creature.position().level).borrow_mut();
        let new_pos = (creature.position() + direction)
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

        let ref tile = map[new_pos.x][new_pos.y];
        if let Some(ref creature) = tile.creature {
            return Err(ActionError::TileIsOccupied(creature.clone())) // weak ref
        }
        if !tile.is_passable() {
            return Err(ActionError::TileIsImpassable(new_pos))
        }

        Ok(())
    } else {
        Err(ActionError::SubjectIsDead)
    }
}


pub(super) fn move_creature(
    world: &mut World,
    creature: &Weak<CreatureRef>,
    direction: Direction) -> Result {
    // TODO: change unwraps to customized expect-like function
    //       saying that all checks must be performed in corresponding
    //       is_valid_function
    let creature_ref = creature;
    is_move_valid(world, creature, direction)?;
    let mut creature = creature.upgrade().unwrap();
    let mut creature = creature.borrow_mut();
    let Map(ref mut map) = *world.get_level(creature.position().level).borrow_mut();
    let new_pos = (creature.position() + direction).unwrap();
    map[new_pos.x][new_pos.y].creature = Some(creature_ref.clone());
    map[creature.position().x][creature.position().y].creature = None;
    creature.set_position(new_pos);
    Ok(())
}


pub(super) fn move_cost(creature: &Weak<CreatureRef>, direction: Direction) -> u32 {
    match creature.upgrade() {
        Some(_) => 100, // TODO: replace hardcode with more creature-specific calculation
        None => 0,
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
        move_creature(&mut world, &character, Direction::Right);
        let actual = character.upgrade().map(|a| a.borrow().position()).unwrap();
        let expected = (pos + Direction::Right).unwrap();
        assert_eq!(actual, expected);
    }
}
