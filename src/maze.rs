// Maze generation.

extern crate rand;
use rand::Rng;

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
		for d in &dir {
			let mut x = 0;
			let mut y = 0;
			
			match d {
				&Dir::N => {y=start.1-1;},
				&Dir::S => {y=start.1+1;},
				&Dir::E => {x=start.0+1;},
				&Dir::W => {x=start.0-1;},
			}
			
			if (x, y) != start && !self.map[x][y].filled {
				println!("{:?}", (d, x, y));
			
				if let Some(n) = self.map[start.0][start.1].walls.iter().position(|x| x == d) {
					self.map[start.0][start.1].walls.remove(n);
				}
				if let Some(n) = self.map[x][y].walls.iter().position(|x| x == &d.opposite()) {
					self.map[x][y].walls.remove(n);
				}
				self.map[x][y].filled = true;
				self.map[start.0][start.1].filled = true;
				self.generate((x, y));
			}		
		}
	}
}

