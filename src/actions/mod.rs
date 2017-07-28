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
mod moving;

use std;
use std::rc::Weak;

use World;
use utils::*;
use world::*;

pub type Result = std::result::Result<(), ActionError>;

/// Represents nerror that prevented action to be commited
#[derive(Debug)]
pub enum ActionError {
    /// The [`Creature`] which was supposed to take action is dead (no more reference to it exists
    /// in the [`World`])
    SubjectIsDead,
    /// Out of bounds (of [`Map`]), if position is None - it's not exist at all (for example it is negative
    /// and can't be represented by usize)
    OutOfBounds { position: Option<Position>, width: usize, height: usize},
    /// [`Tile`] is occupied by some [`Creature`]
    TileIsOccupied(Weak<CreatureRef>),
    /// [`Tile`] is impassable by it's nature
    TileIsImpassable(Position),
}

pub enum Action {
    Move(Weak<CreatureRef>, Direction)
}

/// Abstracts the action to be commited in the [`World`]
impl Action {
    pub(crate) fn apply(&self, world: &mut World) -> Result {
        match *self {
            Action::Move(ref creature, direction) =>
                moving::move_creature(world, creature, direction)
        }
    }

    /// Cost of action in time-points, used for scheduling (
    pub(crate) fn cost(&self) -> u32 {
        match *self {
            Action::Move(ref creature, direction) =>
                moving::move_cost(creature, direction)
        }
    }

    pub(crate) fn is_valid(&self, world: &World) -> Result {
        match *self {
            Action::Move(ref creature, direction) =>
                moving::is_move_valid(world, creature, direction)
        }
    }

    pub fn actor(&self) -> &Weak<CreatureRef> {
        match *self {
            Action::Move(ref creature, _) => creature
        }
    }
}
