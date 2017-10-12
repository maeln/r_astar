// Maze generation.

extern crate rand;

use rand::Rng;
use std::fmt;

pub struct Maze {
	map: Vec<Vec<u8>>,
	width: usize,
	height: usize,
	entry: (usize, usize),
	end: (usize, usize),
}

impl Maze {
	pub fn depth_first_gen(width: usize, height: usize) -> Maze {
		let mut map: Vec<Vec<u8>> = Vec::with_capacity(width);
		for i in 0..width {
			map.insert(i, Vec::with_capacity(height));
		}
		
		// There is probably a better way to initialize the maze with 1, but that will do for now
		for x in 0..width {
			for y in 0..height {
				map[x].insert(y, 1);
			}
		}
		
		// TODO: generation here.
		
		Maze { map: map, width: width, height: height, entry: (0, 0), end: (0, 0) }
	}
}

//Pretty printing for the maze.
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut s = String::with_capacity((self.width+2) * (self.height+2));
		for _ in 0..(self.width+2) {s.push('#');}
		s.push('\n');
		for x in 0..self.width {
			s.push('#');
			for y in 0..self.height {
				s.push(if self.map[x][y] == 0 {' '} else if self.map[x][y] == 2 {'.'} else {'#'});
			}
			s.push('#');
			s.push('\n');
		}
		for _ in 0..(self.width+2) {s.push('#');}
		s.push('\n');
		
        write!(f, "{}", s)
    }
}

