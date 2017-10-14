// Maze generation.

extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Dir {
	N, S, E, W,
}

impl Dir {
	fn rand() -> Dir {
		match rand::thread_rng().gen_range(0, 4) {
			0 => Dir::N,
			1 => Dir::S,
			2 => Dir::E,
			3 => Dir::W,
			_ => Dir::N,
		}
	}
	
	fn opposite(&self) -> Dir {
		match self {
			&Dir::N => Dir::S,
			&Dir::S => Dir::N,
			&Dir::E => Dir::W,
			&Dir::W => Dir::E,
		}
	}
}

#[derive(Debug, Hash, Eq)]
pub struct Cell {
	filled: bool,
	walls: Vec<Dir>,
}

impl Cell {
	fn new() -> Cell {
		Cell { filled: false, walls: vec![Dir::N, Dir::S, Dir::E, Dir::W], }
	}
}

impl PartialEq for Cell {
	fn eq(&self, other: &Cell) -> bool {
        self.filled == other.filled &&
        self.walls.contains(&Dir::N) == other.walls.contains(&Dir::N) &&
        self.walls.contains(&Dir::S) == other.walls.contains(&Dir::S) &&
        self.walls.contains(&Dir::E) == other.walls.contains(&Dir::E) &&
        self.walls.contains(&Dir::W) == other.walls.contains(&Dir::W)  
    }
}

#[derive(Debug, Hash, PartialEq)]
pub struct Maze {
	map: Vec<Vec<Cell>>,
	width: usize,
	height: usize,
	entry: (usize, usize),
	end: (usize, usize),
}

const WALL_SIZE: usize = 10;
const WALL_STROKE: usize = 1;
const SIZE: usize = 10;

impl Maze {
	pub fn new(width: usize, height: usize) -> Maze {
		let mut map: Vec<Vec<Cell>> = Vec::with_capacity(width);
		for i in 0..width {
			map.insert(i, Vec::with_capacity(height));
		}
		
		// There is probably a better way to initialize the maze with 1, but that will do for now
		for x in 0..width {
			for y in 0..height {
				map[x].insert(y, Cell::new());
			}
		}
		
		Maze { map: map, width: width, height: height, entry: (0, 0), end: (0, 0) }
	}
	
	pub fn generate(&mut self, start: (usize, usize)) {
		self.map[start.0][start.1].filled = true;
		let mut dir: Vec<Dir> = vec![];
		if start.0 > 0 {
			dir.push(Dir::W);
		}
		if start.0 < (self.width-1) {
			dir.push(Dir::E);
		}
		if start.1 > 0 {
			dir.push(Dir::N);
		}
		if start.1 < (self.height-1) {
			dir.push(Dir::S);
		}
		
		rand::thread_rng().shuffle(&mut dir);
		for d in dir.iter() {
			let mut x = start.0;
			let mut y = start.1;
			
			match d {
				&Dir::N => {y-=1;},
				&Dir::S => {y+=1;},
				&Dir::E => {x+=1;},
				&Dir::W => {x-=1;},
			}
			if !self.map[x][y].filled {		
				if let Some(n) = self.map[start.0][start.1].walls.iter().position(|x| x == d) {
					self.map[start.0][start.1].walls.remove(n);
				}
				if let Some(n) = self.map[x][y].walls.iter().position(|x| x == &d.opposite()) {
					self.map[x][y].walls.remove(n);
				}
				self.map[x][y].filled = true;
				self.generate((x, y));
			}		
		}
	}
	
	pub fn to_svg_file(&self, path: &str) {
		if let Ok(mut f) = File::create(path) {
			f.write_all(self.to_svg().as_bytes());
		}
	}
	
	pub fn to_svg(&self) -> String {
		let mut svg = String::from("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n");
		svg.push_str("<svg>");
		for x in 0..self.width {
			for y in 0..self.height {
				Maze::draw_cell(x, y, &self.map[x][y], &mut svg);
			}
		}
		svg.push_str("</svg>");
		svg
	}
	
	fn draw_cell(x: usize, y: usize, c: &Cell, s: &mut String) {
		s.push_str(&format!("<g stroke='{}' stroke-width='{}'>", "black", WALL_STROKE));
		for d in c.walls.iter() {
			s.push_str(&Maze::draw_wall(x, y, d));
		}
		s.push_str("</g>");
	}
	
	fn draw_wall(x: usize, y:usize, d: &Dir) -> String {
		match d {
				&Dir::N => format!("<line x1='{}' y1='{}' x2='{}' y2='{}'/>", x*SIZE, y*SIZE, x*SIZE+WALL_SIZE, y*SIZE),
				&Dir::S => format!("<line x1='{}' y1='{}' x2='{}' y2='{}'/>", x*SIZE, y*SIZE+WALL_SIZE, x*SIZE+WALL_SIZE, y*SIZE+WALL_SIZE),
				&Dir::E => format!("<line x1='{}' y1='{}' x2='{}' y2='{}'/>", x*SIZE+WALL_SIZE, y*SIZE, x*SIZE+WALL_SIZE, y*SIZE+WALL_SIZE),
				&Dir::W => format!("<line x1='{}' y1='{}' x2='{}' y2='{}'/>", x*SIZE, y*SIZE, x*SIZE, y*SIZE+WALL_SIZE),
		}
	}
}

