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
use std::cell::RefCell;

use world::*;
use utils::*;
use actions::Action;

/// Entry with creature and next action to be commited
struct ActionEntry {
    cost: RefCell<u32>,
    action: Action,
}

pub(crate) struct Scheduler {
    creatures_without_action: Vec<Weak<CreatureRef>>,
    queue: BinaryHeap<ActionEntry>,
}

pub(crate) enum SchedulerError {
    ActionNotAssigned(Weak<CreatureRef>),
    ActorIsDead,
    QueueIsEmpty,
}

impl ActionEntry {
    fn new(action: Action) -> ActionEntry {
        ActionEntry {
            cost: RefCell::new(action.cost()),
            action: action,
        }
    }
}


impl Ord for ActionEntry {
    fn cmp(&self, other: &ActionEntry) -> Ordering {
        self.cost.borrow().cmp(&other.cost.borrow())
    }
}

impl PartialOrd for ActionEntry {
    fn partial_cmp(&self, other: &ActionEntry) -> Option<Ordering> {
        Some(self.cost.borrow().cmp(&other.cost.borrow()))
    }
}

impl Eq for ActionEntry {}

impl PartialEq for ActionEntry {
    fn eq(&self, other: &ActionEntry) -> bool {
        *self.cost.borrow() == *other.cost.borrow()
    }
}

impl Scheduler {

    pub(crate) fn new() -> Scheduler {
        Scheduler {
            creatures_without_action: vec![],
            queue: BinaryHeap::new(),
        }
    }

    /// Adds action to schedulers priority queue
    pub(crate) fn post_action(&mut self, action: Action) {
        debug_assert!(!self.queue.iter()
                      .any(|&ActionEntry { action: ref entry, .. }|
                           identical(entry.actor(), action.actor())));
        if let Some(index) = self.creatures_without_action.iter()
            .position(|creature| identical(action.actor(), creature)) {
                self.creatures_without_action.swap_remove(index);
            }

        self.queue.push(ActionEntry::new(action));
    }

    /// Returns next action scheduled to apply
    pub(crate) fn pop_next_action(&mut self) -> Result<Action, SchedulerError> {
        while self.creatures_without_action.last()
            .map(|rf| rf.upgrade().is_none())
            .unwrap_or(false) {
                self.creatures_without_action.pop();
            }

        if self.creatures_without_action.len() != 0 {
            return Err(SchedulerError::ActionNotAssigned(self.creatures_without_action[0].clone()))
        }

        return match self.queue.pop() {
            Some(ActionEntry { action, .. }) => {
                for entry in self.queue.iter() {
                    // Note that we do not break the contract with BinaryHeap:
                    // all the values still preserve the same order
                    *entry.cost.borrow_mut() -= action.cost();
                    // TODO: add bonus time when action returned with
                    // negative action times
                }


                self.creatures_without_action.push(action.actor().clone());
                Ok(action)
            },
            None => Err(SchedulerError::QueueIsEmpty),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}





