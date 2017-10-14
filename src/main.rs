// A* implementation in rust.

extern crate rand;

mod maze;

use rand::Rng;
use std::collections::HashMap;
use maze::Maze;

const WIDTH: usize = 32;
const HEIGHT: usize = 16;
const NBWALL: usize = WIDTH*HEIGHT/4;

fn main() {
    let mut adjmat: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    random_wall(&mut adjmat, NBWALL);
    
    print!("{}", pretty_print_adjmat(&adjmat));
    let path = a_star(&adjmat, (0,0), (WIDTH-1,HEIGHT-1));
    if path.is_none() {
    	println!("No path possible.");
    } else {
		for &(x, y) in path.unwrap().iter() {
			adjmat[y][x] = 2;
		}
		print!("{}", pretty_print_adjmat(&adjmat));
    }
    
    let mut m = Maze::new(6, 6);
    m.generate((0,0));
    m.to_svg_file("/home/maeln/test.svg");
}

// Since we can't go in diagonal direction, manhattant distance is a good heuristic for A*.
fn manhattan(pa: (usize, usize), pb: (usize, usize)) -> usize {
	((pb.0 as isize - pa.0 as isize).abs() + (pb.1 as isize - pa.1 as isize).abs()) as usize
}

// A*
fn a_star(mat: &[[i32; WIDTH]; HEIGHT], start: (usize, usize), finish: (usize, usize)) -> Option<Vec<(usize, usize)>> {
	let mut closed: Vec<(usize, usize)> = Vec::new();
	let mut opened: Vec<(usize, usize)> = Vec::new();
	opened.push(start);
	let mut from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
	let mut gscore: HashMap<(usize, usize), usize> = HashMap::new();
	gscore.insert(start, 0);
	let mut fscore: HashMap<(usize, usize), usize> = HashMap::new();
	fscore.insert(start, manhattan(start, finish));
	
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
		
		let neighbors = get_neighbor(mat, current.0, current.1);
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
			fscore.insert(n.clone(), tgscore + manhattan(n.clone(), finish));
		}
		
	}
	
	None
}

// Print the adjacence matrix of the graph like a maze.
fn pretty_print_adjmat(mat: &[[i32; WIDTH]; HEIGHT]) -> String {
	let mut s = String::with_capacity((WIDTH+2) * (HEIGHT+2));
	for _ in 0..(WIDTH+2) {s.push('#');}
	s.push('\n');
	for y in 0..HEIGHT {
		s.push('#');
		for x in 0..WIDTH {
			s.push(if mat[y][x] == 0 {' '} else if mat[y][x] == 2 {'.'} else {'#'});
		}
		s.push('#');
		s.push('\n');
	}
	for _ in 0..(WIDTH+2) {s.push('#');}
	s.push('\n');
	
	s
}

// Create random "wall" in the "maze" (disconnect some node from their neighbor).
fn random_wall(mat: &mut [[i32; WIDTH]; HEIGHT], nb_wall: usize) {
	let mut wall_to_add: usize = nb_wall;
    while wall_to_add > 0 {
    	let x: usize = rand::thread_rng().gen_range(0, WIDTH);
    	let y: usize = rand::thread_rng().gen_range(0, HEIGHT);
 		
 		if (x == 0 && y == 0) || (x == WIDTH-1 && y == HEIGHT-1) {
 			continue;
 		}
 		 		
    	if mat[y][x] == 0 {
    		mat[y][x] = 1;
    		wall_to_add -= 1;
    	}
    }
}

// Get the neighbor of any node.
fn get_neighbor(mat: &[[i32; WIDTH]; HEIGHT], x: usize, y: usize) -> Vec<(usize, usize)> {
	let mut neighbor: Vec<(usize, usize)> = Vec::new();
	
	if x+1 < WIDTH && mat[y][x+1] == 0 {
		neighbor.push( (x+1, y) );
	}
	
	if y+1 < HEIGHT && mat[y+1][x] == 0 {
		neighbor.push( (x, y+1) );
	}
	
	if x > 0 && mat[y][x-1] == 0 {
		neighbor.push( (x-1, y) );
	}
	
	if y > 0 && mat[y-1][x] == 0 {
		neighbor.push( (x, y-1) );
	}
	
	neighbor
}


