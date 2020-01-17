use std::process::Command;
use std::io::{BufWriter, stdout};
use std::io::Write;
use std::{thread, time};  // For sleeping

pub struct Game {
    // dimensions of the map
    height: usize,
    width: usize,

    // all locations containing food
    pub food_locations: Vec<Location>,

    // snake
    snake: Snake
}

#[derive(Debug, Clone, Copy)]
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
    pub fn matches(&self, x: usize, y: usize) -> bool {
        self.x == x && self.y == y
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
        let mut game = Game{food_locations: vec!(), height: height, width: width, snake: snake};
        game.food_locations.push(Location{x: 3, y: 3, location_type: LocationType::FOOD});
        game
    }
    pub fn start(&mut self){
        self.draw();
        // TODO: wait for first keyboard press to start
        loop {
            self.snake.update();
            self.draw();
            if self.is_finished() {
                println!("Game over!\tYou died with a score of {}", self.snake.score);
                break;
            }
            // TODO: sleep so an acceptable playing speed is reached
            let sleeptime = time::Duration::from_secs(1);
            thread::sleep(sleeptime);
        }
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

        let mut map: Vec<Location> = vec!();
        // TODO: add border to game using | and ¨¨¨¨¨¨¨¨¨and ___________

        // search for positions in food and snake sets
        // TODO: optimize, lots of repeated useless checks every drawcycle!
        for x in 0 .. self.height {
            for y in 0 .. self.width {
                let mut location = Location{x: x, y: y, location_type: LocationType::EMPTY};
                let mut found_location = false;
                for food_loc in self.food_locations.iter() {
                    if food_loc.matches(x, y) {
                        found_location = true;
                        location = Location::from(food_loc);
                        break;
                    }
                }
                if !found_location {
                    for snake_loc in self.snake.body.iter() {
                        if snake_loc.matches(x, y) {
                            location = Location::from(snake_loc);
                            break;
                        }
                    }
                }
                map.push(location);
            }
        }

        let mut stream = BufWriter::new(stdout());
        // Draw the actual map
        for (i, location) in map.iter().enumerate() {
            // write!(stream, "{}", i).expect("Bad i write");
            if i % (self.width) == 0 {
                stream.write(b"\n").expect("Invalid stream write");
            }
            match location.location_type {
                LocationType::EMPTY => stream.write(b"E"),
                LocationType::FOOD => stream.write(b"X"),
                LocationType::SNAKE => stream.write(b"O"),
            }.expect("Invalid stream write");
            
        }

        println!("{}\tRUSTY SNAKE\n", String::from_utf8_lossy(&output.stdout));
        println!("Amount of tiles: {}", map.len());
        stream.flush().expect("Error flushing stream!");

    }
}

#[derive(Copy, Clone, Debug)]
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
