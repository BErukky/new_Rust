use piston_window::{Button, Key, G2d};
use piston_window::graphics::Context;
type Color = [f32; 4];
use rand::{rng, Rng};
//use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle, to_coord};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.93, 0.0, 0.0, 0.5];

//const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: u32,
    height: u32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            snake: Snake::new(2, 2),
            food_exists: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }
        self.update_snake(dir);
    }
    pub fn draw<G: piston_window::graphics::Graphics>(&self, con: &Context, g: &mut G) {
        self.snake.draw(con, g);
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        // draw borders (convert grid coords to pixels)
        draw_rectangle(BORDER_COLOR, 0.0, 0.0, to_coord(self.width as i32), to_coord(self.height as i32), con, g);
        draw_rectangle(BORDER_COLOR, 0.0, 0.0, to_coord(1), to_coord(self.height as i32), con, g);
        draw_rectangle(BORDER_COLOR, to_coord(self.width as i32 - 1), 0.0, to_coord(1), to_coord(self.height as i32), con, g);
        draw_rectangle(BORDER_COLOR, 0.0, to_coord(self.height as i32 - 1), to_coord(self.width as i32), to_coord(1), con, g);
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0.0, 0.0, to_coord(self.width as i32), to_coord(self.height as i32), con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.game_over {
            self.waiting_time += delta_time;
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }
        self.update_snake(None);
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);
        if self.snake.inside_snake(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < (self.width as i32) && next_y < (self.height as i32)
    }
    fn add_food(&mut self) {
    let mut rng = rand::rng();
    let mut new_x = rng.random_range(1..(self.width as i32 - 1));
    let mut new_y = rng.random_range(1..(self.height as i32 - 1));
    while self.snake.overlap_with_self(new_x, new_y) {
        new_x = rng.random_range(1..(self.width as i32 - 1));
        new_y = rng.random_range(1..(self.height as i32 - 1));
    }
    self.food_x = new_x;
    self.food_y = new_y;
    self.food_exists = true;
}


    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food_exists = false;
        self.game_over = false;
        self.waiting_time = 0.0;
    }
}
