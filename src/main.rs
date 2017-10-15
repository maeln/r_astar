// A* implementation in rust.

extern crate rand;

mod maze;

use maze::Maze;

fn main() {
    let mut m = Maze::new(32, 16);
    m.generate((0,0));
    m.to_svg_file("maze.svg", &Vec::new());
    if let Some(n) = m.a_star((0,0), (31, 15)) {
    	m.to_svg_file("solved_maze.svg", &n);
    } else {
    	println!("No Path in maze.");
    }
}

