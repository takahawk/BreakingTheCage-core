use std::ops::Add;
use std::rc::Weak;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub level: usize,
    pub x: usize,
    pub y: usize,
}



#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left, Right, Up, Down
}

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, direction: Direction) -> Option<Position> {
        use self::Direction::*;
        let (x, y) = (self.x as i32, self.y as i32);
        let (dx, dy): (i32, i32) = match direction {
            Left => (-1, 0),
            Right => (1, 0),
            Up => (0, -1),
            Down => (0, 1)
        };

        let (x, y) = (x + dx, y + dy);
        if x < 0 || y < 0 {
            None
        } else {
            Some(Position { x: x as usize, y: y as usize, level: self.level })
        }
    }

}

/// Returns true if weak references points to the same data
/// and false if not or either points to already deallocated data
pub(crate) fn identical<T>(first: &Weak<T>, second: &Weak<T>) -> bool {
    if let (Some(first), Some(second)) = (first.upgrade(), second.upgrade()) {
        (first.as_ref() as *const _) == (second.as_ref() as *const _)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak_refs_identical() {
        let a = Rc::new(5);
        let ref1 = Rc::downgrade(&a);
        let ref2 = Rc::downgrade(&a);
        assert!(identical(&ref1, &ref2));
    }

    #[test]
    fn weak_refs_not_identical() {
        let a = Rc::new(5);
        let b = Rc::new(5);
        let ref1 = Rc::downgrade(&a);
        let ref2 = Rc::downgrade(&b);
        assert!(!identical(&ref1, &ref2));
    }

    #[test]
    fn weak_refs_deallocated() {
        let a = Rc::new(5);
        let ref1 = Rc::downgrade(&a);
        let ref2 = Rc::downgrade(&a);
        drop(a);
        assert!(!identical(&ref1, &ref2));
    }
}

