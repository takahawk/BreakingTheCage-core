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
use std;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Weak;

use world::*;
use actions::Action;
use self::ActionEntry::*;

type Result = std::result::Result<(), SchedulerError>;

/// Entry with creature and next action to be commited
enum ActionEntry {
    /// Creature without assigned action. Action must be assigned in order for scheduler can continue
    CreatureWithoutAction(Weak<CreatureRef>),
    /// Action to be commited
    Action(Box<Action>)
}

pub(crate) struct Scheduler {
    queue: BinaryHeap<ActionEntry>,
}

enum SchedulerError {
    ActionNotAssigned(Weak<CreatureRef>),
    ActorIsDead,
}

impl ActionEntry {
    fn is_creature_without_action(&self) -> bool {
        match self {
            &CreatureWithoutAction(_) => true,
            _ => false
        }
    }
}

impl Ord for ActionEntry {
    fn cmp(&self, other: &ActionEntry) -> Ordering {
        // all entries without an assigned action must be in the beginning
        // of the queue, rest must be sorted by cost descendingly
        match (self, other) {
            (&CreatureWithoutAction(_), _) => Ordering::Greater,
            (_, &CreatureWithoutAction(_)) => Ordering::Less,
            (&Action(ref this_action), &Action(ref that_action)) => {
                that_action.cost().cmp(&that_action.cost())
            }
        }
    }
}

impl PartialOrd for ActionEntry {
    fn partial_cmp(&self, other: &ActionEntry) -> Option<Ordering> {
        Some(match (self, other) {
            (&CreatureWithoutAction(_), _) => Ordering::Greater,
            (_, &CreatureWithoutAction(_)) => Ordering::Less,
            (&Action(ref this_action), &Action(ref that_action)) => {
                that_action.cost().cmp(&that_action.cost())
            }
        })
    }
}

impl Eq for ActionEntry {}

impl PartialEq for ActionEntry {
    fn eq(&self, other: &ActionEntry) -> bool {
        match (self, other) {
            (&CreatureWithoutAction(_), &CreatureWithoutAction(_)) => true,
            (&Action(ref this_action), &Action(ref that_action)) => {
                this_action.cost() == that_action.cost()
            },
            _ => false
        }
    }
}

impl Scheduler {
    fn post_action(&mut self, action: Box<Action>) {
        unimplemented!()
    }

    fn do_next() -> self::Result {
        unimplemented!()
    }
}