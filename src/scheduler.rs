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
    cost: RefCell<i32>,
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
            cost: RefCell::new(cost as i32),
            action: action,
        }
    }
}


impl Ord for ActionEntry {
    fn cmp(&self, other: &ActionEntry) -> Ordering {
        other.cost.borrow().cmp(&self.cost.borrow())
    }
}

impl PartialOrd for ActionEntry {
    fn partial_cmp(&self, other: &ActionEntry) -> Option<Ordering> {
        Some(other.cost.borrow().cmp(&self.cost.borrow()))
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
        let ActionEntry { action, cost } = self.queue.pop().unwrap();

        let cost = *cost.borrow();
        let bonus_time = if cost > 0 {
            for entry in self.queue.iter() {
                *entry.cost.borrow_mut() -= cost as i32;
                // TODO: add bonus time when action returned with
                // negative action times
            }
            0
        } else {
            -cost
        } as u32;
        self.unassigned.push(
            UnassignedEntry {
                creature: action.actor().clone(),
                bonus_time: bonus_time,
            });
        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use creatures::Creature;
    use actions::Action::MockAction;
    use std::rc::Rc;

    fn mock_creature(name: &str) -> Rc<CreatureRef> {
        Rc::new(
            RefCell::new(
                Creature::demon(
                    String::from(name), 10, 10, Position { x: 10, y: 10, level: 0 }, 10)))
    }

    fn creatures_setup() -> Vec<Rc<CreatureRef>> {
        vec![
            mock_creature("Abaddon"),
            mock_creature("Asmodeus"),
            mock_creature("Baal"),
            mock_creature("Baphomet")
        ]
    }

    #[test]
    fn sequential_actions() {
        let creatures = creatures_setup();
        let mut scheduler = Scheduler::new();
        scheduler.post_action(MockAction(Rc::downgrade(&creatures[0]), 1));
        scheduler.post_action(MockAction(Rc::downgrade(&creatures[1]), 2));
        scheduler.post_action(MockAction(Rc::downgrade(&creatures[2]), 3));

        let expected: Vec<_> = creatures.iter()
            .take(3)
            .map(|actor| {
                let actor = actor.borrow();
                actor.name().to_owned()  
            })
            .collect();
        let mut result = vec![];
        loop {
            match scheduler.pop_next() {
                Ok(action) => {
                    let actor = action.actor().upgrade().unwrap();
                    result.push(actor.borrow().name().to_owned());
                },
                Err(SchedulerError::ActionNotAssigned(creature)) =>
                    scheduler.post_action(MockAction(creature, 5)),
                Err(SchedulerError::QueueIsEmpty) => break,
            }
        }
        assert_eq!(result,
                   expected,
                   "Actions nots equential:\n\tActual: {:?}\n\tExpected (must be {:?})",
                   result,
                   expected);
    }

    #[test]
    fn non_assigned() {
        let creatures = creatures_setup();
        let mut scheduler = Scheduler::new();
        scheduler.post_action(MockAction(Rc::downgrade(&creatures[0]), 1));
        scheduler.post_action(MockAction(Rc::downgrade(&creatures[1]), 2));
        scheduler.post_action(MockAction(Rc::downgrade(&creatures[2]), 3));
        scheduler.pop_next();
        assert!(scheduler.pop_next().is_err());
    }
}
