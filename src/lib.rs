use std::process::Command;
use std::io::{BufWriter, stdout};
use std::io::Write;

pub struct Map {
    // all locations in order and their values
    pub map: Vec<Location>,

    // dimensions of the map
    height: usize,
    width: usize,

    // the current direction of the snake
    direction: Direction,
}

impl Map {
    pub fn create(height: usize, width: usize) -> Map {
        let mut map = Map{map: vec!(), height: height, width: width, direction: Direction::RIGHT};
        for _i in 0 .. height * width{
            map.map.push(Location::EMPTY);
        }
        map.map[3*width+3] = Location::FOOD;
        map.map[5*width+2] = Location::SNAKEHEAD;
        map
    }
    pub fn draw(&mut self) {
        // writebuffer from array representing playing field
        // flush buffer (with clear command):
        // println!("{}", String::from_utf8_lossy(&output.stdout));
        // holds output variable:
        let output = Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

        let mut stream = BufWriter::new(stdout());
        for (i, location) in self.map.iter().enumerate() {
            match location {
                Location::EMPTY => stream.write(b" "),
                Location::FOOD => stream.write(b"X"),
                Location::SNAKEBODY => stream.write(b"O"),
                Location::SNAKEHEAD => stream.write(b">"),
            }.expect("Invalid stream write");
            if i % (self.width - 1) == 0 {
                stream.write(b"\n").expect("Invalid stream write");
            }
        }

        println!("{}\tRUSTY SNAKE\n", String::from_utf8_lossy(&output.stdout));
        stream.flush().expect("Error flushing stream!");

    }
}

pub enum Location {
    EMPTY,
    FOOD,
    SNAKEBODY,
    SNAKEHEAD,
}

enum Direction {
   UP,
   DOWN,
   RIGHT,
   LEFT,
}
