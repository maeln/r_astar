// Maze generation.

extern crate rand;

use std::collections::HashSet;
use rand::Rng;
use std::fmt;

#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
pub struct Point {
	x: usize,
	y: usize,
}

impl Point {
	pub fn new(x: usize, y: usize) -> Point {
		Point { x, y }
	}
	
	pub fn rand(xrange: usize, yrange: usize) -> Point {
		Point { x: rand::thread_rng().gen_range(0, xrange), y: rand::thread_rng().gen_range(0, yrange)}
	}
	
	pub fn dist(&self, p: Point) -> f64 {
		(((p.x as isize - self.x as isize).pow(2) + (p.y as isize - self.y as isize).pow(2)) as f64).sqrt()
	}
	
	pub fn manhattan(&self, p: Point) -> usize {
		((p.x as isize - self.x as isize).abs() + (p.y as isize - self.y as isize).abs()) as usize
	}
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

pub struct Maze {
	map: Vec<Vec<u8>>,
	width: usize,
	height: usize,
	entry: Point,
	end: Point,
}

impl Maze {
	pub fn generate(width: usize, height: usize) -> Maze {
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
		
		// Generating the maze.
		let mut walls: Vec<Point> = Vec::new();
		let start = Point::rand(width, height);
		map[start.x][start.y] = 0;
		for w in get_walls(&map, &start).iter() {
			walls.push(w.clone());
		}
		
		while !walls.is_empty() {
			let choice = walls.pop().unwrap();
			let neighwall = get_walls(&map, &choice);
			
			if get_neighbors(&map, &choice).len() < 2 {
				map[choice.x][choice.y] = 0;
				for w in neighwall.iter() {
					if !walls.contains(w) {
						walls.push(w.clone());
					}
				}
			}
		}
		
		Maze { map: map, width: width, height: height, entry: Point::new(0, 0), end: Point::new(0, 0) }
	}
	
	pub fn neighbors(&self, p: Point) -> Vec<Point> {
		let mut neigh = Vec::with_capacity(4);
		
		if p.x+1 < self.width && self.map[p.x+1][p.y] == 0 {
			neigh.push( Point::new(p.x+1, p.y) );
		}
	
		if p.y+1 < self.height && self.map[p.x][p.y+1] == 0 {
			neigh.push( Point::new(p.x, p.y+1) );
		}
	
		if p.x > 0 && self.map[p.x-1][p.y] == 0 {
			neigh.push( Point::new(p.x-1, p.y) );
		}
	
		if p.y > 0 && self.map[p.x][p.y-1] == 0 {
			neigh.push( Point::new(p.x, p.y-1) );
		}
		
		neigh
	}
	
	pub fn walls(&self, p: Point) -> Vec<Point> {
		let mut w = Vec::with_capacity(4);
		
		if p.x+1 < self.width && self.map[p.x+1][p.y] == 1 {
			w.push( Point::new(p.x+1, p.y) );
		}
	
		if p.y+1 < self.height && self.map[p.x][p.y+1] == 1 {
			w.push( Point::new(p.x, p.y+1) );
		}
	
		if p.x > 0 && self.map[p.x-1][p.y] == 1 {
			w.push( Point::new(p.x-1, p.y) );
		}
	
		if p.y > 0 && self.map[p.x][p.y-1] == 1 {
			w.push( Point::new(p.x, p.y-1) );
		}
		
		w
	}
}

fn get_walls(map: &Vec<Vec<u8>>, p: &Point) -> Vec<Point> {
		let mut w = Vec::with_capacity(4);
		
		if p.x+1 < map.len() && map[p.x+1][p.y] == 1 {
			w.push( Point::new(p.x+1, p.y) );
		}
	
		if p.y+1 < map[0].len() && map[p.x][p.y+1] == 1 {
			w.push( Point::new(p.x, p.y+1) );
		}
	
		if p.x > 0 && map[p.x-1][p.y] == 1 {
			w.push( Point::new(p.x-1, p.y) );
		}
	
		if p.y > 0 && map[p.x][p.y-1] == 1 {
			w.push( Point::new(p.x, p.y-1) );
		}
		
		w
}

fn get_neighbors(map: &Vec<Vec<u8>>, p: &Point) -> Vec<Point> {
		let mut w = Vec::with_capacity(4);
		
		if p.x+1 < map.len() && map[p.x+1][p.y] == 0 {
			w.push( Point::new(p.x+1, p.y) );
		}
	
		if p.y+1 < map[0].len() && map[p.x][p.y+1] == 0 {
			w.push( Point::new(p.x, p.y+1) );
		}
	
		if p.x > 0 && map[p.x-1][p.y] == 0 {
			w.push( Point::new(p.x-1, p.y) );
		}
	
		if p.y > 0 && map[p.x][p.y-1] == 0 {
			w.push( Point::new(p.x, p.y-1) );
		}
		
		w
}

//Pretty printing for the maze.
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut s = String::with_capacity((self.width+2) * (self.height+2));
		for _ in 0..(self.width+2) {s.push('#');}
		s.push('\n');
		for y in 0..self.height {
			s.push('#');
			for x in 0..self.width {
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

