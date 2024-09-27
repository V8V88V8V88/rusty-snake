use piston_window::*;
use rand::Rng;
use std::path::Path;

const BLOCK_SIZE: f64 = 25.0;
const WIDTH: i32 = 30;
const HEIGHT: i32 = 20;
const FRAME_RATE: f64 = 60.0;
const UPDATES_PER_SECOND: f64 = 10.0;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

struct Snake {
    body: Vec<(i32, i32)>,
    direction: Direction,
}

struct Food {
    position: (i32, i32),
    is_bonus: bool,
    timer: f64,
}

struct Game {
    snake: Snake,
    food: Food,
    game_over: bool,
    score: u32,
    game_started: bool,
}

impl Game {
    fn new() -> Game {
        let snake = Snake {
            body: vec![(WIDTH / 2, HEIGHT / 2)],
            direction: Direction::None,
        };
        let mut game = Game {
            snake,
            food: Food {
                position: (0, 0),
                is_bonus: false,
                timer: 0.0,
            },
            game_over: false,
            score: 0,
            game_started: false,
        };
        game.place_food();
        game
    }

    fn place_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT));
            if !self.snake.body.contains(&new_food) {
                let is_bonus = rng.gen_ratio(1, 10); // 10% chance for bonus food
                self.food = Food {
                    position: new_food,
                    is_bonus,
                    timer: if is_bonus { 7.0 } else { 0.0 },
                };
                break;
            }
        }
    }

    fn update(&mut self, dt: f64) {
        if self.game_over || !self.game_started {
            return;
        }

        if self.food.is_bonus {
            self.food.timer -= dt;
            if self.food.timer <= 0.0 {
                self.place_food();
            }
        }

        let (head_x, head_y) = self.snake.body[0];
        let new_head = match self.snake.direction {
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::None => return,
        };

        if new_head.0 < 0 || new_head.0 >= WIDTH || new_head.1 < 0 || new_head.1 >= HEIGHT {
            self.game_over = true;
            return;
        }

        if self.snake.body.len() > 1
            && self.snake.body[..self.snake.body.len() - 1].contains(&new_head)
        {
            self.game_over = true;
            return;
        }

        self.snake.body.insert(0, new_head);

        if new_head == self.food.position {
            self.score += if self.food.is_bonus { 5 } else { 1 };
            self.place_food();
        } else {
            self.snake.body.pop();
        }
    }

    fn restart(&mut self) {
        *self = Game::new();
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [(WIDTH as f64) * BLOCK_SIZE, (HEIGHT as f64) * BLOCK_SIZE],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let ref font = Path::new("assets/FiraSans-Regular.ttf");
    let mut glyphs = window.load_font(font).expect("Could not load font");

    let mut game = Game::new();
    let mut events = Events::new(EventSettings::new().max_fps(FRAME_RATE as u64));
    let mut last_update_time = 0.0;

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if !game.game_over {
                match key {
                    Key::Right if game.snake.direction != Direction::Left => {
                        game.snake.direction = Direction::Right;
                        game.game_started = true;
                    }
                    Key::Left if game.snake.direction != Direction::Right => {
                        game.snake.direction = Direction::Left;
                        game.game_started = true;
                    }
                    Key::Up if game.snake.direction != Direction::Down => {
                        game.snake.direction = Direction::Up;
                        game.game_started = true;
                    }
                    Key::Down if game.snake.direction != Direction::Up => {
                        game.snake.direction = Direction::Down;
                        game.game_started = true;
                    }
                    _ => {}
                }
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            if game.game_over {
                let restart_button = [
                    (WIDTH as f64 * BLOCK_SIZE) / 2.0 - 50.0,
                    (HEIGHT as f64 * BLOCK_SIZE) / 2.0 + 40.0,
                    100.0,
                    40.0,
                ];
                if pos[0] >= restart_button[0]
                    && pos[0] <= restart_button[0] + restart_button[2]
                    && pos[1] >= restart_button[1]
                    && pos[1] <= restart_button[1] + restart_button[3]
                {
                    if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                        game.restart();
                    }
                }
            }
        }

        if let Some(args) = e.update_args() {
            last_update_time += args.dt;
            if last_update_time > 1.0 / UPDATES_PER_SECOND {
                game.update(last_update_time);
                last_update_time = 0.0;
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for &(x, y) in &game.snake.body {
                rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    [
                        x as f64 * BLOCK_SIZE,
                        y as f64 * BLOCK_SIZE,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    ],
                    c.transform,
                    g,
                );
            }

            let food_color = if game.food.is_bonus {
                [1.0, 1.0, 0.0, 1.0]
            } else {
                [1.0, 0.0, 0.0, 1.0]
            };
            rectangle(
                food_color,
                [
                    game.food.position.0 as f64 * BLOCK_SIZE,
                    game.food.position.1 as f64 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                ],
                c.transform,
                g,
            );

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
                .draw(
                    &format!("Score: {}", game.score),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 30.0),
                    g,
                )
                .unwrap();

            if game.game_over {
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                    .draw(
                        "Game Over!",
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(
                            (WIDTH as f64 * BLOCK_SIZE) / 2.0 - 80.0,
                            (HEIGHT as f64 * BLOCK_SIZE) / 2.0,
                        ),
                        g,
                    )
                    .unwrap();

                let restart_button = [
                    (WIDTH as f64 * BLOCK_SIZE) / 2.0 - 50.0,
                    (HEIGHT as f64 * BLOCK_SIZE) / 2.0 + 40.0,
                    100.0,
                    40.0,
                ];
                rectangle([0.2, 0.2, 0.2, 1.0], restart_button, c.transform, g);
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 20)
                    .draw(
                        "Restart",
                        &mut glyphs,
                        &c.draw_state,
                        c.transform
                            .trans(restart_button[0] + 20.0, restart_button[1] + 25.0),
                        g,
                    )
                    .unwrap();
            }

            glyphs.factory.encoder.flush(device);
        });
    }
}
