use rusty_snake::Map;

fn main() {
  const HEIGHT: usize = 10;
  const WIDTH: usize = 10;

  let mut map: Map = Map::create(HEIGHT, WIDTH);
  for _i in 0 .. 1000000 {
    map.draw();
  }
}
