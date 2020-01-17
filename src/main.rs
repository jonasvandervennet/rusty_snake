use rusty_snake::Game;

fn main() {
  const HEIGHT: usize = 10;
  const WIDTH: usize = 10;

  let mut game = Game::create(HEIGHT, WIDTH);
  game.start();
}
