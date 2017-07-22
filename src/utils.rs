use std::ops::Add;

#[derive(Clone, Copy, Debug)]
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
