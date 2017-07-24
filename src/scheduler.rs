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
use utils::*;
use actions::Action;

type Result = std::result::Result<(), SchedulerError>;

/// Entry with creature and next action to be commited
struct ActionEntry(Box<Action>);

pub(crate) struct Scheduler {
    creatures_without_action: Vec<Weak<CreatureRef>>,
    queue: BinaryHeap<ActionEntry>,
}

enum SchedulerError {
    ActionNotAssigned(Weak<CreatureRef>),
    ActorIsDead,
}

impl Ord for ActionEntry {
    fn cmp(&self, other: &ActionEntry) -> Ordering {
        self.0.cost().cmp(&other.0.cost())
    }
}

impl PartialOrd for ActionEntry {
    fn partial_cmp(&self, other: &ActionEntry) -> Option<Ordering> {
        Some(self.0.cost().cmp(&other.0.cost()))
    }
}

impl Eq for ActionEntry {}

impl PartialEq for ActionEntry {
    fn eq(&self, other: &ActionEntry) -> bool {
        self.0.cost() == other.0.cost()
    }
}

impl Scheduler {

    /// Adds action to schedulers priority queue
    fn post_action(&mut self, action: Box<Action>) {
        debug_assert!(!self.queue.iter()
                      .any(|&ActionEntry(ref entry)| identical(entry.actor(), action.actor())));
        if let Some(index) = self.creatures_without_action.iter()
            .position(|creature| identical(action.actor(), creature)) {
                self.creatures_without_action.swap_remove(index);
            }

        self.queue.push(ActionEntry(action));
    }

    fn do_next() -> self::Result {
        unimplemented!()
    }
}




