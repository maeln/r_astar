// Maze generation.

extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

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
		
		Maze { map: map, width: width, height: height}
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
	
	pub fn to_svg_file(&self, path: &str, astar: &Vec<(usize, usize)>) {
		if let Ok(mut f) = File::create(path) {
			f.write_all(self.to_svg(astar).as_bytes());
		}
	}
	
	pub fn to_svg(&self, astar: &Vec<(usize, usize)>) -> String {
		let mut svg = String::from("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n");
		svg.push_str("<svg>");
		for x in 0..self.width {
			for y in 0..self.height {
				Maze::draw_cell(x, y, &self.map[x][y], &mut svg);
			}
		}
		if !astar.is_empty() {
			svg.push_str(&Maze::draw_path(astar));
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
	
	fn draw_path(path: &Vec<(usize, usize)>) -> String {
		let mut s = String::from(format!("<polyline fill='none' stroke='green' stroke-width='{}' points='", WALL_STROKE));
		for &(x, y) in path.iter() {
			s.push_str(&format!("{},{} ", x*SIZE+WALL_SIZE/2, y*SIZE+WALL_SIZE/2));
		}
		s.push_str("'/>");
		s
	}
	
	pub fn a_star(&self, start: (usize, usize), finish: (usize, usize)) -> Option<Vec<(usize, usize)>> {
		let mut closed: Vec<(usize, usize)> = Vec::new();
		let mut opened: Vec<(usize, usize)> = Vec::new();
		opened.push(start);
		let mut from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
		let mut gscore: HashMap<(usize, usize), usize> = HashMap::new();
		gscore.insert(start, 0);
		let mut fscore: HashMap<(usize, usize), usize> = HashMap::new();
		fscore.insert(start, Maze::manhattan(start, finish));
	
		while !opened.is_empty() {
			let mut current = start;
			for node in opened.iter() {
				if current == start || fscore.get(&current) > fscore.get(node) {
					current = node.clone();
				}
			}
		
			if current == finish {
				let mut final_path: Vec<(usize, usize)> = Vec::new();
				final_path.push(current);
				while from.contains_key(&current) {
					current = *from.get(&current).unwrap();
					final_path.push(current);
				}
			
				return Some(final_path);
			}
		
			if let Some(n) = opened.iter().position(|&x| x == current) {
				opened.remove(n);
			}
		
			if let None = closed.iter().position(|&x| x == current) {
				closed.push(current);
			}
		
			let neighbors = self.neighbors(current.0, current.1);
			println!("{:?}", neighbors);
			for n in neighbors.iter() {
				if let Some(_) = closed.iter().position(|x| x == n) {
					continue;
				}
			
				if let None = opened.iter().position(|x| x == n) {
					opened.push(n.clone());
				}
			
				let tgscore: usize = gscore.get(&current).unwrap() + 1;
				if gscore.contains_key(n) && &tgscore >= gscore.get(n).unwrap() {
					continue;
				}
			
				from.insert(n.clone(), current);
				gscore.insert(n.clone(), tgscore);
				fscore.insert(n.clone(), tgscore + Maze::manhattan(n.clone(), finish));
			}
		
		}
	
		None
	}

	
	fn manhattan(pa: (usize, usize), pb: (usize, usize)) -> usize {
		((pb.0 as isize - pa.0 as isize).abs() + (pb.1 as isize - pa.1 as isize).abs()) as usize
	}
	
	fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
		let mut n: Vec<(usize, usize)> = Vec::with_capacity(4);
		let c: &Cell = &self.map[x][y];
		
		if let None = c.walls.iter().position(|x| x == &Dir::E) {
			if x+1 < self.width {
				n.push( (x+1, y) );
			}	
		}
		
		if let None = c.walls.iter().position(|x| x == &Dir::S) {
			if y+1 < self.height {
				n.push( (x, y+1) );
			}
		}
		
		if let None = c.walls.iter().position(|x| x == &Dir::W) {
			if x > 0 {
				n.push( (x-1, y) );
			}
		}
		
		if let None = c.walls.iter().position(|x| x == &Dir::N) {
			if y > 0 {
				n.push( (x, y-1) );
			}
		}
		
		n
	}
}

