use std::process::Command;
use std::io::{BufWriter, stdout};
use std::io::Write;

pub struct Game {
    // dimensions of the map
    height: usize,
    width: usize,

    // all locations in order and their values
    pub map: Vec<Location>,

    // snake
    snake: Snake
}

pub struct Location {
    x: usize,
    y: usize,
    location_type: LocationType,
}

impl Location {
    pub fn from(location: &Location) -> Location {
        Location{x: location.x, y: location.y, location_type: location.location_type}
    }
    pub fn advance(&self, dir: &Direction) -> Location {
        match dir {
            Direction::UP => Location{x: self.x - 1, y: self.y + 0, location_type: self.location_type},
            Direction::DOWN => Location{x: self.x + 1, y: self.y + 0, location_type: self.location_type},
            Direction::RIGHT => Location{x: self.x + 0, y: self.y + 1, location_type: self.location_type},
            Direction::LEFT => Location{x: self.x + 0, y: self.y - 1, location_type: self.location_type},
        }
    }
}

struct Snake {
    // the current direction of the snake
    direction: Direction,
    // food found this round
    found: bool, 

    // body locations in order starting from the head
    body: Vec<Location>,

    dead: bool,
    score: usize,
}

impl Snake {
    pub fn new() -> Snake {
        let mut snake = Snake{direction: Direction::RIGHT, body: vec!(), dead: false, found: false, score: 0};
        snake.body.push(Location{x: 5, y: 2, location_type: LocationType::SNAKE});
        snake
    }
    pub fn contains(&self, location: &Location) -> bool {
        for part in self.body.iter(){
            if part.x == location.x && part.y == location.y {return true}
        }
        false
    }
    pub fn update(&mut self) {
        let mut body: Vec<Location> = vec!();
        let newhead = Location::from(self.body.first().expect("Error getting snake head.")).advance(&self.direction);
        if self.contains(&newhead) {self.dead = true;}
        body.push(newhead);
        self.body.iter().for_each(|loc| body.push(Location::from(loc)));

        if !self.found {body.remove(body.len() - 1);}
        else {self.score += 1;}
        self.found = false;
        self.body = body;
    }
}

impl Game {
    pub fn create(height: usize, width: usize) -> Game {
        let snake: Snake = Snake::new();
        let mut map = Game{map: vec!(), height: height, width: width, snake: snake};
        for x in 0 .. height {
            for y in 0 .. width {
                map.map.push(Location{x: x, y: y, location_type: LocationType::EMPTY});
            }
        }
        map.map[3*width+3].location_type = LocationType::FOOD;
        map
    }
    pub fn start(&mut self){
        self.draw();
        // TODO: wait for first keyboard press to start
        loop {
            self.update();
            self.draw();
            if self.is_finished() {
                println!("Game over!\tYou died with a score of {}", self.snake.score);
                break;
            }
        }
    }
    fn update(&mut self) {
        self.snake.update();
        self.update_map();
    }
    fn update_map(&mut self) {
        let mut newmap = vec!();
        self.map.iter().for_each(|loc| newmap.push(Location::from(loc)));
        for loc in self.map.iter() {
            match loc.location_type {
                LocationType::SNAKE => if !self.snake.contains(loc) {newmap[loc.x * self.height + loc.y].location_type = LocationType::EMPTY},
                _ => (),
            }
        }
        self.map = newmap;
    }
    fn is_finished(&self) -> bool {
        // should I check for extra cases?
        // I think everything is accounted for
        if self.snake.dead {return true}
        for part in self.snake.body.iter(){
            if part.x == 0 || part.x == self.width || part.y == 0 || part.y == self.height {return true}
        }
        false
    }
    fn draw(&self) {
        // writebuffer from array representing playing field
        // flush buffer (with clear command):
        // println!("{}", String::from_utf8_lossy(&output.stdout));
        // holds output variable:
        let output = Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

        let mut stream = BufWriter::new(stdout());
        // TODO: add border to game using | and ¨¨¨¨¨¨¨¨¨and ___________
        for (i, location) in self.map.iter().enumerate() {
            match location.location_type {
                LocationType::EMPTY => stream.write(b" "),
                LocationType::FOOD => stream.write(b"X"),
                LocationType::SNAKE => stream.write(b"O"),
            }.expect("Invalid stream write");
            if i % (self.width - 1) == 0 {
                stream.write(b"\n").expect("Invalid stream write");
            }
        }

        println!("{}\tRUSTY SNAKE\n", String::from_utf8_lossy(&output.stdout));
        stream.flush().expect("Error flushing stream!");

    }
}

#[derive(Copy, Clone)]
pub enum LocationType {
    EMPTY,
    FOOD,
    SNAKE,
}

#[derive(Copy, Clone)]
pub enum Direction {
   UP,
   DOWN,
   RIGHT,
   LEFT,
}
