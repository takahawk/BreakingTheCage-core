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

/// Representing tiled map for game
pub struct Map(Vec<Vec<Tile>>);

impl Map {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        assert!(!tiles.is_empty(), "Map can't be empty!");
        assert!(tiles.iter().all(|row| row.len() == tiles[0].len()),
                "Rows are not the same size!");
        Map(tiles)
    }

    pub fn tiles(&self) -> Tiles {
        Tiles { x: 0, y: 0, map: &self }
    }
}

pub struct Tiles<'a> {
    x: usize,
    y: usize,
    map: &'a Map,
}

impl <'a> Iterator for Tiles<'a> {
    type Item = (usize, usize, &'a Tile);

    fn next(&mut self) -> Option<(usize, usize, &'a Tile)> {
        let &Map(ref tiles) = self.map;

        match (self.x, self.y) {
            (_, y) if y >= tiles[0].len()   => None,
            (x, y) if x == tiles.len() - 1  => {
                self.y += 1;
                self.x = 0;
                Some((x, y, &tiles[x][y]))
            },
            (x, y) => {
                self.x += 1;
                Some((x, y, &tiles[x][y]))
            }
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        return Tile { tile_type }
    }
}

#[derive(Clone)]
pub enum TileType {
    Ground,
    Wall,
    Door { closed: bool },
    Stairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new_invalid_map() {
        use self::TileType::*;
        Map::new(vec![vec![Tile::new(Ground), Tile::new(Ground)],
                      vec![Tile::new(Ground)]]);
    }
}