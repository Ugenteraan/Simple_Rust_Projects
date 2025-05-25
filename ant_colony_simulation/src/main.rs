
use crossterm::{cursor, execute, terminal};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};


#[derive(Debug)]
enum AntType {
    Soldier,
    Worker
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PheromoneType {
    Weak,
    Strong
}

#[derive(Debug)]
enum AntMode {
    Wandering,
    Returning
}

#[derive(Debug)]
enum PredatorMode{
    Wandering,
    Attacking
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pheromone {
    position: Position,
    pheromone_type: PheromoneType,
    duration: u32,
    time_elapsed: u32
}


#[derive(Debug)]
struct Ant {
    position: Position,
    ant_type: AntType,
    moves: u16,
    lifespan: u32,
    starvation: u32,
    food_in_hand: bool,
    mode: AntMode,
    is_alive: bool,
    time_elapsed: u32,
    explored_pheromones: Option<Pheromone>

}

// #[derive(Debug)]
// struct Predator {
//     position: Position,
//     handle_n_ants: u32,
//     moves: u8,
//     lifespan: u32,
//     starvation: u32,
//     is_alive: bool
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u16,
    y: u16
}

#[derive(Debug)]
struct Food {
    position: Position,
    lifespan: u32,
    food_size: u32,
    time_elapsed: u32

}

#[derive(Debug)]
struct Colony {
    position: Position,
    food_amount: u32,
}

#[derive(Debug)]
struct World {
    height: u16,
    width: u16,
    game_over: bool,
    colony: Colony,
    ants: VecDeque<Ant>,
    pheromones: VecDeque<Pheromone>,
    foods: VecDeque<Food>,
    food_spawn_rate: f64,
}







impl Ant {

    fn new(initial_position: Position, ant_type: AntType, 
           moves: u16, lifespan: u32, 
           starvation: u32, food_in_hand: bool, 
           mode: AntMode) -> Self {

        Ant{
            position: initial_position,
            ant_type: ant_type,
            moves: moves,
            lifespan: lifespan,
            starvation: starvation,
            food_in_hand: food_in_hand,
            mode: mode,
            is_alive: true,
            time_elapsed: 0,
            explored_pheromones: None
        }

    }
}


impl Pheromone {

    fn new(position: Position, pheromone_type: PheromoneType) -> Self{

        let mut duration: u32;

        match pheromone_type {
            PheromoneType::Weak => {duration = 10},
            PheromoneType::Strong => {duration = 20}
        }

        Pheromone {
            position: position,
            pheromone_type: pheromone_type,
            duration: duration,
            time_elapsed: 0
        }

    }   
}

impl Food {
    fn new(width: &u16, height: &u16) -> Self {

        let food_position: Position =  Position {
            x: rand::rng().random_range(1..width-1),
            y: rand::rng().random_range(1..height-1),
        };

        Food {
            position: food_position,
            lifespan: rand::rng().random_range(30..200),
            food_size: rand::rng().random_range(50..100),
            time_elapsed: 0
        }
    }
}



fn wandering_ant(height: &u16, width: &u16, ant: &mut Ant, pheromones: &mut VecDeque<Pheromone>) {


        for pheromone in pheromones.clone() {

            if pheromone.pheromone_type == PheromoneType::Weak{
                continue;
            }

            if let Some(explored_pheromones) = ant.explored_pheromones {
                if explored_pheromones.position.x == pheromone.position.x && explored_pheromones.position.y == pheromone.position.y {

                    continue;

                }
            } else {

                let x_difference = ant.position.x.abs_diff(pheromone.position.x);
                let y_difference = ant.position.y.abs_diff(pheromone.position.y); 

                if x_difference == 1 || y_difference == 1 {
                    ant.position = pheromone.position;
                    ant.explored_pheromones = Some(pheromone.clone());
                }
                return;
            }
        }

        
        

        let random_x: i32 = rand::rng().random_range(-1..=1);        
        let random_y: i32 = rand::rng().random_range(-1..=1);

        if random_x == 1 && ant.position.x != width - 1 {
           ant.position.x = ant.position.x.saturating_add(1);
        }else if random_x == -1 {
           ant.position.x = ant.position.x.saturating_sub(1);
        }

        ant.position.x = ant.position.x.max(1);

        if random_y == 1 && ant.position.y != height - 1{
            ant.position.y = ant.position.y.saturating_add(1);
        }else if random_y == -1 {
            ant.position.y = ant.position.y.saturating_sub(1);
        }

        ant.position.y = ant.position.y.max(1);


        let wandering_pheromone: Pheromone = Pheromone::new(ant.position.clone(), PheromoneType::Weak);


        pheromones.push_back(wandering_pheromone);

    }


fn check_ant_found_food(ant: &mut Ant, foods: &mut VecDeque<Food>) {

    for food in foods {
        if ant.position == food.position {
            ant.mode = AntMode::Returning;
            ant.food_in_hand = true;

            food.food_size = food.food_size.saturating_sub(2);
        }

    }
}


fn returning_ant(height: &u16, width: &u16, ant: &mut Ant, 
                pheromones: &mut VecDeque<Pheromone>, colony_position: &Position) {
    
    //check if the colony is nearby.

    if ant.position.x.abs_diff(colony_position.x) == 1 || 
       ant.position.y.abs_diff(colony_position.y) == 1 {

        ant.position = colony_position.clone();
        return;
    }   


    //check for pheromones around it.

    let mut strongest_pheromone: Option<Pheromone> = None;

    for pheromone in pheromones.clone() {

        let x_difference = ant.position.x.abs_diff(pheromone.position.x);
        let y_difference = ant.position.y.abs_diff(pheromone.position.y);



        if x_difference == 1 || y_difference == 1 {

            if pheromone.pheromone_type == PheromoneType::Strong{
                strongest_pheromone = Some(pheromone.clone());
            } else if pheromone.pheromone_type == PheromoneType::Weak {

                let ph_to_colony_diff_x = pheromone.position.x.abs_diff(colony_position.x);
                let ph_to_colony_diff_y = pheromone.position.y.abs_diff(colony_position.y);

                let ant_to_colony_diff_x = ant.position.x.abs_diff(colony_position.x);
                let ant_to_colony_diff_y = ant.position.y.abs_diff(colony_position.y);

                if ph_to_colony_diff_x < ant_to_colony_diff_x || ph_to_colony_diff_y < ant_to_colony_diff_y {
                    strongest_pheromone = Some(pheromone);
                }
            } 

        }
    }

    if let Some(pheromone) = strongest_pheromone {
        ant.position = pheromone.position.clone();
    }else{

        let random_x: i32 = rand::rng().random_range(-1..=1);        
        let random_y: i32 = rand::rng().random_range(-1..=1);

        if random_x == 1 && ant.position.x != width - 1 {
           ant.position.x = ant.position.x.saturating_add(1);
        }else if random_x == -1 {
           ant.position.x = ant.position.x.saturating_sub(1);
        }

        ant.position.x = ant.position.x.max(1);

        if random_y == 1 && ant.position.y != height - 1{
            ant.position.y = ant.position.y.saturating_add(1);
        }else if random_y == -1 {
            ant.position.y = ant.position.y.saturating_sub(1);
        }

        ant.position.y = ant.position.y.max(1);

    }   

    let returning_pheromone: Pheromone = Pheromone::new(ant.position.clone(), PheromoneType::Strong);

    pheromones.push_back(returning_pheromone);


}


fn update_pheromones(pheromones: &mut VecDeque<Pheromone>) {

    pheromones.retain(|pheromone| pheromone.time_elapsed < pheromone.duration);
}

fn create_food(height: &u16, width: &u16, foods: &mut VecDeque<Food>) {

    foods.push_back(Food::new(width, height));
}

fn update_foods(foods: &mut VecDeque<Food>) {
    foods.retain(|food| food.time_elapsed < food.lifespan && food.food_size > 0);
}

fn update_ants(ants: &mut VecDeque<Ant>) {
    ants.retain(|ant| ant.time_elapsed < ant.lifespan);
}



impl World {


    fn new(width: u16, height:u16) -> Self {

        let colony_position: Position =  Position {
            x: rand::rng().random_range(1..width),
            y: rand::rng().random_range(1..height),
        };

        let colony: Colony = Colony {
            position: colony_position.clone(),
            food_amount: 10
        };

        let mut ants = VecDeque::new();
        ants.push_back(Ant::new(colony_position, AntType::Worker, 
                                 1, 100, 
                                 20, false, 
                                 AntMode::Wandering));

        let mut pheromones = VecDeque::new();
        let mut foods = VecDeque::new();

        World {
            height: height,
            width: width,
            game_over: false,
            colony: colony,
            ants: ants,
            pheromones: pheromones,
            foods: foods,
            food_spawn_rate: 0.1
        }
    }

    fn render(&mut self) {
        execute!(stdout(), cursor::Hide).unwrap(); //hides the blinking cursor.
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap(); //Clears the screen
                                                                                //from the previous
                                                                                //artifacts and
                                                                                //leftovers. 
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


        for pheromone in &mut self.pheromones {


            match pheromone.pheromone_type {
                PheromoneType::Weak => {execute!(stdout(), cursor::MoveTo(pheromone.position.x, pheromone.position.y)).unwrap();
                                        println!(".");},
                PheromoneType::Strong => {execute!(stdout(), cursor::MoveTo(pheromone.position.x, pheromone.position.y)).unwrap();
                                        println!("-");},
            }

            pheromone.time_elapsed = pheromone.time_elapsed.saturating_add(1);

        }

        

        for ant in &self.ants {
            execute!(stdout(), cursor::MoveTo(ant.position.x, ant.position.y)).unwrap();
            println!("🐜");
        }

        for food in &mut self.foods {

            execute!(stdout(), cursor::MoveTo(food.position.x, food.position.y)).unwrap();
            println!("🍎");

            food.time_elapsed = food.time_elapsed.saturating_add(1);
        }


        execute!(stdout(), cursor::MoveTo(self.colony.position.x, self.colony.position.y)).unwrap();
        println!("🏠");

        

        stdout().flush().unwrap() //forces any pending buffer to be written to the terminal
                                  //immedidately.
        

    }



    //at every step.
    fn update(&mut self) {

        if self.game_over {
            return;
        }

        if (self.ants.len() as u32) < self.colony.food_amount {
            self.ants.push_back(Ant::new(self.colony.position.clone(), AntType::Worker, 
                                 1, 20, 
                                 10, false, 
                                 AntMode::Wandering));
        }


        //move ants randomly or return them back to colony.
        for ant in &mut self.ants {

            match ant.mode {
                AntMode::Wandering => wandering_ant(&self.height, &self.width, ant, &mut self.pheromones),
                AntMode::Returning => returning_ant(&self.height, &self.width, ant, 
                                                    &mut self.pheromones, &self.colony.position)
            }


            check_ant_found_food(ant, &mut self.foods);

            
        }

        update_ants(&mut self.ants);

        update_pheromones(&mut self.pheromones);

        let mut rng = rand::rng();
        if rng.random::<f64>() < self.food_spawn_rate {
            create_food(&self.height, &self.width, &mut self.foods);
        }

        update_foods(&mut self.foods);

    }


}








fn main() {


    let width: u16 = 80;
    let height: u16 = 20;


    let mut game = World::new(width, height);

    terminal::enable_raw_mode().unwrap(); //by default, terminals are in cooked mode. In raw mode,
                                          //the inputs like the arrow keys are handled by the
                                          //program as opposed to the normal termina.
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap(); //enter an alternate state so that
                                                                 //when the program exits, the
                                                                 //original terminal remains the
                                                                 //same.

    //main simulation loop

    let mut last_update = Instant::now();
    while !game.game_over {

        if last_update.elapsed() >= Duration::from_millis(500) {
            game.update();
            game.render();
            last_update = Instant::now();
        }

    }


}











