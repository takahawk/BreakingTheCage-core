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

struct UnassignedEntry {
    creature: Weak<CreatureRef>,
    bonus_time: u32,
}

pub(crate) struct Scheduler {
    unassigned: Vec<UnassignedEntry>,
    queue: BinaryHeap<ActionEntry>,
}

pub(crate) enum SchedulerError {
    ActionNotAssigned(Weak<CreatureRef>),
    QueueIsEmpty,
}

impl ActionEntry {
    fn new(action: Action, cost: u32) -> ActionEntry {
        ActionEntry {
            cost: RefCell::new(cost),
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
            unassigned: vec![],
            queue: BinaryHeap::new(),
        }
    }

    /// Adds action to schedulers priority queue
    pub(crate) fn post_action(&mut self, action: Action) {
        debug_assert!(!self.queue.iter()
                      .any(|&ActionEntry { action: ref entry, .. }|
                           identical(entry.actor(), action.actor())));
        let mut cost = action.cost();
        if let Some(index) = self.unassigned.iter()
            .position(|&UnassignedEntry { ref creature, .. }|
                          identical(action.actor(), creature)) {
                let entry = self.unassigned.swap_remove(index);
                cost -= entry.bonus_time;
            }

        self.queue.push(ActionEntry::new(action, cost));
    }

    pub(crate) fn peek_next(&mut self) -> Result<&Action, SchedulerError> {
        while self.unassigned.last()
            .map(|&UnassignedEntry { ref creature, .. }| creature.upgrade().is_none())
            .unwrap_or(false) {
                self.unassigned.pop();
            }

        if self.unassigned.len() != 0 {
            return Err(SchedulerError::ActionNotAssigned(
                self.unassigned[0].creature.clone()))
        }

        match self.queue.peek() {
            Some(&ActionEntry { ref action, .. }) => Ok(action),
            None => Err(SchedulerError::QueueIsEmpty),
        }
        
    }

    /// Returns next action scheduled to apply
    pub(crate) fn pop_next(&mut self) -> Result<Action, SchedulerError> {
        self.peek_next()?; // all errors must be handled in peek_next()
        let ActionEntry { action, .. } = self.queue.pop().unwrap();

        for entry in self.queue.iter() {
            *entry.cost.borrow_mut() -= action.cost();
            // TODO: add bonus time when action returned with
            // negative action times
            
        }
        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
