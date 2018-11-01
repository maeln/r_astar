extern crate clap;
use clap::{App, Arg};

extern crate rand;

mod maze;

use maze::Maze;
use maze::Point;

fn main() {
    let matches = App::new("R_A*")
        .arg(
            Arg::with_name("width")
                .short("w")
                .help("Width of the maze")
                .takes_value(true),
        ).arg(
            Arg::with_name("height")
                .short("h")
                .help("Height of the maze")
                .takes_value(true),
        ).arg(
            Arg::with_name("step")
                .short("s")
                .help("Output every step of the maze generation"),
        ).get_matches();

    let width = matches
        .value_of("width")
        .unwrap_or("32")
        .parse::<usize>()
        .unwrap();
    let height = matches
        .value_of("height")
        .unwrap_or("16")
        .parse::<usize>()
        .unwrap();

    let step = matches.is_present("step");

    let mut m = Maze::new(width, height, step);
    m.generate(Point::new(0, 0));

    m.trace = false;
    m.to_svg_file("maze.svg", Point::new(0, 0), &Vec::new());
    if let Some(n) = m.a_star(Point::new(0, 0), Point::new(width - 1, height - 1)) {
        m.to_svg_file("solved_maze.svg", Point::new(0, 0), &n);
    } else {
        println!("No Path in maze.");
    }
}
