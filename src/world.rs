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
use map::tiles::Map;
use map::tiles::Tiles;
use map::generators::*;

/// Main entity holding entire game state with levels, creatures, player character etc.
pub struct World {
    current_level: usize,
    levels: Vec<Map>,
}

impl World {

    /// Creates new world with generating levels for it
    pub fn new() -> World {
        // current logic is stub, used only for debugging and testing
        let level = SimpleBoxGenerator::new(20, 20).generate();
        World {
            current_level: 0,
            levels: vec![level]
        }
    }

    /// Access method for iterating through current level tiles
    pub fn current_tiles(&self) -> Tiles {
        self.levels[self.current_level].tiles()
    }
}