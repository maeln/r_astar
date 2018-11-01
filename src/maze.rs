// Maze generation.

extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Dir {
	N,
	S,
	E,
	W,
}

impl Dir {
	fn opposite(&self) -> Dir {
		match self {
			&Dir::N => Dir::S,
			&Dir::S => Dir::N,
			&Dir::E => Dir::W,
			&Dir::W => Dir::E,
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Point {
	x: usize,
	y: usize,
}

impl Point {
	pub fn new(x: usize, y: usize) -> Point {
		Point { x: x, y: y }
	}
}

#[derive(Debug, Hash, Eq, Clone)]
pub struct Cell {
	filled: bool,
	walls: Vec<Dir>,
}

impl Cell {
	fn new() -> Cell {
		Cell {
			filled: false,
			walls: vec![Dir::N, Dir::S, Dir::E, Dir::W],
		}
	}

	pub fn remove_wall(&mut self, d: Dir) {
		if let Some(n) = self.walls.iter().position(|x| x == &d) {
			self.walls.remove(n);
		}
	}
}

impl PartialEq for Cell {
	fn eq(&self, other: &Cell) -> bool {
		self.filled == other.filled
			&& self.walls.contains(&Dir::N) == other.walls.contains(&Dir::N)
			&& self.walls.contains(&Dir::S) == other.walls.contains(&Dir::S)
			&& self.walls.contains(&Dir::E) == other.walls.contains(&Dir::E)
			&& self.walls.contains(&Dir::W) == other.walls.contains(&Dir::W)
	}
}

#[derive(Debug, Hash, PartialEq)]
pub struct Maze {
	map: Vec<Vec<Cell>>,
	width: usize,
	height: usize,
	pub trace: bool,
}

const WALL_SIZE: usize = 10;
const WALL_STROKE: usize = 1;
const SIZE: usize = 10;

impl Maze {
	pub fn new(width: usize, height: usize, trace: bool) -> Maze {
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

		Maze {
			map: map,
			width: width,
			height: height,
			trace: trace,
		}
	}

	pub fn generate(&mut self, start: Point) {
		self.maze_iter(start);
	}

	pub fn maze_iter(&mut self, start: Point) {
		let mut stack: LinkedList<Point> = LinkedList::new();
		stack.push_front(start);

		let mut counter = 0;
		while !stack.is_empty() {
			let current = stack.pop_front().unwrap();
			self.map[current.x][current.y].filled = true;

			if self.trace {
				self.to_svg_file(&format!("maze_{}.svg", counter), current, &Vec::new());
				counter += 1;
			}

			let neighbors = self.unvisited_neighbor(current);

			if !neighbors.is_empty() {
				stack.push_front(current);
				let p = rand::thread_rng().choose(&neighbors).unwrap();
				self.take_down_wall(current, *p);
				self.map[p.x][p.y].filled = true;
				stack.push_front(*p);
			}
		}
	}

	fn take_down_wall(&mut self, p1: Point, p2: Point) {
		let xdiff = p1.x as isize - p2.x as isize;
		let ydiff = p1.y as isize - p2.y as isize;

		// Check if they are neighbors
		if xdiff.abs() + ydiff.abs() > 1 {
			return;
		}
		if xdiff > 1 || xdiff < -1 {
			return;
		}
		if ydiff > 1 || ydiff < -1 {
			return;
		}

		if xdiff > 0 {
			self.map[p1.x][p1.y].remove_wall(Dir::W);
			self.map[p2.x][p2.y].remove_wall(Dir::W.opposite());
		} else if xdiff < 0 {
			self.map[p1.x][p1.y].remove_wall(Dir::E);
			self.map[p2.x][p2.y].remove_wall(Dir::E.opposite());
		} else if ydiff > 0 {
			self.map[p1.x][p1.y].remove_wall(Dir::N);
			self.map[p2.x][p2.y].remove_wall(Dir::N.opposite());
		} else if ydiff < 0 {
			self.map[p1.x][p1.y].remove_wall(Dir::S);
			self.map[p2.x][p2.y].remove_wall(Dir::S.opposite());
		}
	}

	fn unvisited_neighbor(&self, p: Point) -> Vec<Point> {
		let mut dir: Vec<Point> = Vec::with_capacity(4);

		if p.x > 0 && !self.map[p.x - 1][p.y].filled {
			dir.push(Point::new(p.x - 1, p.y));
		}
		if p.x < (self.width - 1) && !self.map[p.x + 1][p.y].filled {
			dir.push(Point::new(p.x + 1, p.y));
		}
		if p.y > 0 && !self.map[p.x][p.y - 1].filled {
			dir.push(Point::new(p.x, p.y - 1));
		}
		if p.y < (self.height - 1) && !self.map[p.x][p.y + 1].filled {
			dir.push(Point::new(p.x, p.y + 1));
		}

		dir
	}

	pub fn to_svg_file(&self, path: &str, current: Point, astar: &Vec<Point>) {
		if let Ok(mut f) = File::create(path) {
			match f.write_all(self.to_svg(current, astar).as_bytes()) {
				Err(e) => {
					panic!(format!("Error while saving the SVG: {}", e));
				}
				Ok(_) => println!("SVG saved."),
			}
		}
	}

	pub fn to_svg(&self, current: Point, astar: &Vec<Point>) -> String {
		let mut svg = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
		svg.push_str(&format!(
			"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\">",
			self.width * SIZE,
			self.height * SIZE
		));
		for x in 0..self.width {
			for y in 0..self.height {
				Maze::draw_cell(self, Point::new(x, y), current, &self.map[x][y], &mut svg);
			}
		}
		if !astar.is_empty() {
			svg.push_str(&Maze::draw_path(astar));
		}
		svg.push_str("</svg>");
		svg
	}

	fn draw_cell(&self, pos: Point, current: Point, c: &Cell, s: &mut String) {
		if self.trace && c.filled {
			s.push_str(&format!(
				"<rect x='{}' y ='{}' width='{}' height='{}' fill='{}' />",
				pos.x * SIZE,
				pos.y * SIZE,
				WALL_SIZE,
				WALL_SIZE,
				if pos.x == current.x && pos.y == current.y {
					"red"
				} else {
					"green"
				}
			));
		}

		s.push_str(&format!(
			"<g stroke='{}' stroke-width='{}'>",
			"black", WALL_STROKE
		));
		for d in c.walls.iter() {
			s.push_str(&Maze::draw_wall(pos.x, pos.y, d));
		}
		s.push_str("</g>");
	}

	fn draw_wall(x: usize, y: usize, d: &Dir) -> String {
		match d {
			&Dir::N => format!(
				"<line x1='{}' y1='{}' x2='{}' y2='{}'/>",
				x * SIZE,
				y * SIZE,
				x * SIZE + WALL_SIZE,
				y * SIZE
			),
			&Dir::S => format!(
				"<line x1='{}' y1='{}' x2='{}' y2='{}'/>",
				x * SIZE,
				y * SIZE + WALL_SIZE,
				x * SIZE + WALL_SIZE,
				y * SIZE + WALL_SIZE
			),
			&Dir::E => format!(
				"<line x1='{}' y1='{}' x2='{}' y2='{}'/>",
				x * SIZE + WALL_SIZE,
				y * SIZE,
				x * SIZE + WALL_SIZE,
				y * SIZE + WALL_SIZE
			),
			&Dir::W => format!(
				"<line x1='{}' y1='{}' x2='{}' y2='{}'/>",
				x * SIZE,
				y * SIZE,
				x * SIZE,
				y * SIZE + WALL_SIZE
			),
		}
	}

	fn draw_path(path: &Vec<Point>) -> String {
		let mut s = String::from(format!(
			"<polyline fill='none' stroke='green' stroke-width='{}' points='",
			WALL_STROKE
		));
		for Point { x, y } in path.iter() {
			s.push_str(&format!(
				"{},{} ",
				x * SIZE + WALL_SIZE / 2,
				y * SIZE + WALL_SIZE / 2
			));
		}
		s.push_str("'/>");
		s
	}

	pub fn a_star(&self, start: Point, finish: Point) -> Option<Vec<Point>> {
		let mut closed: Vec<Point> = Vec::new();
		let mut opened: Vec<Point> = Vec::new();
		opened.push(start);
		let mut from: HashMap<Point, Point> = HashMap::new();
		let mut gscore: HashMap<Point, usize> = HashMap::new();
		gscore.insert(start, 0);
		let mut fscore: HashMap<Point, usize> = HashMap::new();
		fscore.insert(start, Maze::manhattan(start, finish));

		while !opened.is_empty() {
			let mut current = start;
			for node in opened.iter() {
				if current == start || fscore.get(&current) > fscore.get(node) {
					current = *node;
				}
			}

			if current == finish {
				let mut final_path: Vec<Point> = Vec::new();
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

			let neighbors = self.neighbors(current);
			for n in neighbors.iter() {
				if let Some(_) = closed.iter().position(|x| x == n) {
					continue;
				}

				if let None = opened.iter().position(|x| x == n) {
					opened.push(*n);
				}

				let tgscore: usize = gscore.get(&current).unwrap() + 1;
				if gscore.contains_key(n) && &tgscore >= gscore.get(n).unwrap() {
					continue;
				}

				from.insert(*n, current);
				gscore.insert(*n, tgscore);
				fscore.insert(*n, tgscore + Maze::manhattan(*n, finish));
			}
		}

		None
	}

	fn manhattan(pa: Point, pb: Point) -> usize {
		((pb.x as isize - pa.x as isize).abs() + (pb.y as isize - pa.y as isize).abs()) as usize
	}

	fn neighbors(&self, p: Point) -> Vec<Point> {
		let Point { x, y } = p;
		let mut n: Vec<Point> = Vec::with_capacity(4);
		let c: &Cell = &self.map[x][y];

		if let None = c.walls.iter().position(|x| x == &Dir::E) {
			if x + 1 < self.width {
				n.push(Point::new(x + 1, y));
			}
		}

		if let None = c.walls.iter().position(|x| x == &Dir::S) {
			if y + 1 < self.height {
				n.push(Point::new(x, y + 1));
			}
		}

		if let None = c.walls.iter().position(|x| x == &Dir::W) {
			if x > 0 {
				n.push(Point::new(x - 1, y));
			}
		}

		if let None = c.walls.iter().position(|x| x == &Dir::N) {
			if y > 0 {
				n.push(Point::new(x, y - 1));
			}
		}

		n
	}
}
