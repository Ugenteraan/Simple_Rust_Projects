use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, terminal,
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

struct SnakeGame {
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    width: u16,
    height: u16,
    score: u16,
    game_over: bool,
}

impl SnakeGame {
    fn new(width: u16, height: u16) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position { x: width / 2, y: height / 2 });

        let food = Position {
            x: rand::thread_rng().gen_range(1..width),
            y: rand::thread_rng().gen_range(1..height),
        };

        SnakeGame {
            snake,
            direction: Direction::Right,
            food,
            width,
            height,
            score: 0,
            game_over: false,
        }
    }

    fn render(&self) {
    execute!(stdout(), cursor::Hide).unwrap();
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    // Draw the top border
    for x in 0..=self.width {
        execute!(stdout(), cursor::MoveTo(x, 0)).unwrap();
        print!("#");
    }

    // Draw the bottom border
    for x in 0..=self.width {
        execute!(stdout(), cursor::MoveTo(x, self.height)).unwrap();
        print!("#");
    }

    // Draw the left and right borders
    for y in 1..self.height {
        execute!(stdout(), cursor::MoveTo(0, y)).unwrap();
        print!("#");
        execute!(stdout(), cursor::MoveTo(self.width, y)).unwrap();
        print!("#");
    }

    // Render the snake
    for segment in &self.snake {
        execute!(stdout(), cursor::MoveTo(segment.x, segment.y)).unwrap();
        print!("O");
    }

    // Render the food
    execute!(stdout(), cursor::MoveTo(self.food.x, self.food.y)).unwrap();
    print!("X");

    // Render the score
    execute!(stdout(), cursor::MoveTo(0, self.height + 1)).unwrap();
    println!("Score: {}", self.score);

    stdout().flush().unwrap();
}



    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Calculate the new head position
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y.saturating_sub(1) },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x.saturating_sub(1), y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        // Check collision with walls
        if new_head.x == 0 || new_head.y == 0 || new_head.x >= self.width || new_head.y >= self.height {
            self.game_over = true;
            return;
        }

        // Check collision with itself
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Add new head to the snake
        self.snake.push_front(new_head);

        // Check if food is eaten
        if new_head == self.food {
            self.score += 1;
            self.food = Position {
                x: rand::thread_rng().gen_range(1..self.width),
                y: rand::thread_rng().gen_range(1..self.height),
            };
        } else {
            // Remove the tail
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        // Prevent reversing direction
        if (self.direction == Direction::Up && new_direction == Direction::Down)
            || (self.direction == Direction::Down && new_direction == Direction::Up)
            || (self.direction == Direction::Left && new_direction == Direction::Right)
            || (self.direction == Direction::Right && new_direction == Direction::Left)
        {
            return;
        }
        self.direction = new_direction;
    }
}

fn main() {
    let width: u16 = 30;
    let height: u16 = 20;
    let mut game = SnakeGame::new(width, height);

    terminal::enable_raw_mode().unwrap();
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();

    let mut last_update = Instant::now();

    while !game.game_over {
        // Handle input
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        // Update game state
        if last_update.elapsed() >= Duration::from_millis(200) {
            game.update();
            game.render();
            last_update = Instant::now();
        }
    }

    // Clean up
    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();

    if game.game_over {
        println!("Game Over! Final Score: {}", game.score);
    }
}
