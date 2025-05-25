use crossterm::{cursor, event::{self, Event, KeyCode}, execute, terminal};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: u16,
    y: u16
}


struct SnakeGame {
    
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    width: u16,
    height: u16,
    score: u16,
    game_over: bool
}


impl SnakeGame {
    
    fn new(width:u16, height:u16) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position {x: width/2, y: height/2});
        
        let food = Position {
            x: rand::thread_rng().gen_range(1..width),
            y: rand::thread_rng().gen_range(1..height)
        };

        SnakeGame{
            snake,
            direction: Direction::Right,
            food,
            width, 
            height,
            score: 0,
            game_over: false
        }
    }
    
    fn render(&self) {
        
        execute!(stdout(), cursor::Hide).unwrap(); //hides the blinking cursor.
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap(); //Clears the screen
                                                                                //from the previous
                                                                                //artifacts and
                                                                                //leftovers.for 
        //in order to draw the elements in the terminal, we have to move the cursor to the location
        //before printing out the desired element.
        for x in 0..=self.width {
            execute!(stdout(), cursor::MoveTo(x, 0)).unwrap(); 
            println!("#");
        }

        for x in 0..=self.width {
            execute!(stdout(), cursor::MoveTo(x, self.height)).unwrap();
            println!("#");
        }

        for y in 0..=self.height {
            execute!(stdout(), cursor::MoveTo(0, y)).unwrap();
            println!("#");
            execute!(stdout(), cursor::MoveTo(self.width, y)).unwrap();
            println!("#");
        }

        for segment in &self.snake {
            execute!(stdout(), cursor::MoveTo(segment.x, segment.y)).unwrap();
            println!("o");
        }

        execute!(stdout(), cursor::MoveTo(self.food.x, self.food.y)).unwrap();
        println!("X");

        execute!(stdout(), cursor::MoveTo(0, self.height+1)).unwrap();
        println!("Score: {}", self.score);

        stdout().flush().unwrap() //forces any pending buffer to be written to the terminal
                                  //immedidately.
    }

    fn update(&mut self) {
        
        if self.game_over {
            return;
        }

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            //saturating_sub subtracts 1 from the y without causing any underflow to happen.
            //the origin is at the top left. Hence the substraction instead of addition.
            Direction::Up => Position {x: head.x, y: head.y.saturating_sub(1)},
            Direction::Down => Position {x: head.x, y: head.y + 1},
            Direction::Left => Position {x: head.x.saturating_sub(1), y: head.y},
            Direction::Right => Position {x: head.x + 1, y: head.y}
        };

        //wall collision check
        if new_head.x == 0 || new_head.y == 0 || new_head.x == self.width || new_head.y == self.height {
            self.game_over = true;
            return;
        }
        
        //if the snake bites its own tail.
        if self.snake.contains(&new_head){
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 1;
            self.food = Position{
                x: rand::thread_rng().gen_range(1..self.width),
                y: rand::thread_rng().gen_range(1..self.height)
            };
        } else {
            self.snake.pop_back();
        }

    }
    
    fn change_direction(&mut self, new_direction: Direction) {
        //prevent reverse movements
        if (self.direction == Direction::Up && new_direction == Direction::Down) ||
           (self.direction == Direction::Down && new_direction == Direction::Up) ||
           (self.direction == Direction::Left && new_direction == Direction::Right) ||
           (self.direction == Direction::Right && new_direction == Direction::Left) 
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

    terminal::enable_raw_mode().unwrap(); //by default, terminals are in cooked mode. In raw mode,
                                          //the inputs like the arrow keys are handled by the
                                          //program as opposed to the normal termina.
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap(); //enter an alternate state so that
                                                                 //when the program exits, the
                                                                 //original terminal remains the
                                                                 //same.
    let mut last_update = Instant::now();

    while !game.game_over {
        
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

        if last_update.elapsed() >= Duration::from_millis(200) {
            game.update();
            game.render();
            last_update = Instant::now();
        }
    }

    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    

    if game.game_over {
        println!("Game Over! Score: {}", game.score);
    }

}
