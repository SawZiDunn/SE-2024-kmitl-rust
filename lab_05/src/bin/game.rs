use rand::Rng;
use std::io::{self, Write};

// Player struct with required stats
struct Player {
    hp: i32,
    stamina: i32,
    power: i32,
    gold: i32,
}

// Enemy struct with required attributes
struct Enemy {
    name: String,
    hp: i32,
    power: i32,
    gold_reward: i32,
}

// Direction enum for movement
enum Direction {
    North,
    South,
    East,
    West,
}

// Encounter enum for random events
enum Encounter {
    Nothing,
    Meat,
    Water,
    Herb,
    Enemy,
}

impl Player {
    // Create new player with initial stats
    fn new() -> Player {
        Player {
            hp: 100,
            stamina: 50,
            power: 10,
            gold: 0,
        }
    }

    // Check if player is still alive (HP > 0 and Stamina > 0)
    fn is_alive(&self) -> bool {
        self.hp > 0 && self.stamina > 0
    }

    // Check if player has won (Gold >= 100)
    fn has_won(&self) -> bool {
        self.gold >= 100
    }

    // Display current player status
    fn show_status(&self) {
        println!("\n=== Player Status ===");
        println!("HP: {}/100", self.hp);
        println!("Stamina: {}", self.stamina);
        println!("Power: {}", self.power);
        println!("Gold: {}", self.gold);
        println!("===================\n");
    }

    // Move in a direction and reduce stamina
    fn move_direction(&mut self, direction: Direction) {
        let direction_str = match direction {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        };

        println!("You move {}...", direction_str);
        self.stamina -= 1;
        println!("Stamina reduced by 1. Current stamina: {}", self.stamina);
    }

    // Generate random encounter using the specified probabilities
    fn generate_encounter(&self) -> Encounter {
        let roll = rand::thread_rng().gen_range(0..100);
        match roll {
            0..=24 => Encounter::Nothing, // 25% chance
            25..=44 => Encounter::Meat,   // 20% chance
            45..=64 => Encounter::Water,  // 20% chance
            65..=79 => Encounter::Herb,   // 15% chance
            _ => Encounter::Enemy,        // 20% chance
        }
    }

    // Handle the encounter based on type
    fn handle_encounter(&mut self) {
        let encounter = self.generate_encounter();

        match encounter {
            Encounter::Nothing => {
                println!("Nothing happens... The area is quiet.");
            }
            Encounter::Meat => {
                println!("You found some fresh meat!");
                let old_hp = self.hp;
                self.hp = (self.hp + 5).min(100); // Cap at 100 HP
                println!(
                    "HP restored by {}. Current HP: {}",
                    self.hp - old_hp,
                    self.hp
                );
            }
            Encounter::Water => {
                println!("You found a fresh water source!");
                self.stamina += 2;
                println!("Stamina increased by 2. Current stamina: {}", self.stamina);
            }
            Encounter::Herb => {
                println!("You found a magical herb!");
                self.power += 1;
                println!("Power increased by 1. Current power: {}", self.power);
            }
            Encounter::Enemy => {
                let enemy = self.generate_enemy();
                println!("A wild {} appears!", enemy.name);
                self.combat(enemy);
            }
        }
    }

    // Generate random enemy based on encounter chances
    fn generate_enemy(&self) -> Enemy {
        let roll = rand::thread_rng().gen_range(0..100);
        match roll {
            0..=59 => Enemy {
                // 60% chance
                name: "Rat".to_string(),
                hp: 10,
                power: 2,
                gold_reward: 10,
            },
            60..=89 => Enemy {
                // 30% chance
                name: "Wolf".to_string(),
                hp: 20,
                power: 5,
                gold_reward: 20,
            },
            _ => Enemy {
                // 10% chance
                name: "Boar".to_string(),
                hp: 30,
                power: 10,
                gold_reward: 30,
            },
        }
    }

    // Simplified combat logic
    fn combat(&mut self, mut enemy: Enemy) {
        println!("\n=== COMBAT START ===");
        println!("You are fighting a {}!", enemy.name);
        println!("Enemy stats - HP: {}, Power: {}", enemy.hp, enemy.power);

        while enemy.hp > 0 && self.hp > 0 {
            println!("\nYour turn!");
            println!("Player HP: {}, Enemy HP: {}", self.hp, enemy.hp);

            // Player attacks first
            let damage_dealt = self.power;
            enemy.hp -= damage_dealt;
            println!("You attack for {} damage!", damage_dealt);

            if enemy.hp <= 0 {
                println!("You defeated the {}!", enemy.name);
                self.gold += enemy.gold_reward;
                println!(
                    "You gained {} gold! Total gold: {}",
                    enemy.gold_reward, self.gold
                );
                break;
            }

            println!("Enemy HP remaining: {}", enemy.hp);

            // Enemy attacks back
            println!("\nEnemy's turn!");
            let enemy_damage = enemy.power;
            self.hp -= enemy_damage;
            println!(
                "The {} attacks you for {} damage!",
                enemy.name, enemy_damage
            );
            println!("Your HP: {}", self.hp);

            if self.hp <= 0 {
                println!("You have been defeated!");
                break;
            }
        }
        println!("=== COMBAT END ===\n");
    }

    // Get direction input from user
    fn get_direction_input() -> Option<Direction> {
        print!("Enter direction (N/S/E/W) or Q to quit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_uppercase();

        match input.as_str() {
            "N" | "NORTH" => Some(Direction::North),
            "S" | "SOUTH" => Some(Direction::South),
            "E" | "EAST" => Some(Direction::East),
            "W" | "WEST" => Some(Direction::West),
            "Q" | "QUIT" => None,
            _ => {
                println!("Invalid input! Please enter N, S, E, W, or Q to quit.");
                Player::get_direction_input()
            }
        }
    }
}

fn main() {
    println!("=== RUST TEXT ADVENTURE GAME - LAB 06 ===");
    println!("Goal: Collect 100 gold before running out of HP or stamina!");
    println!("You start with: HP: 100, Stamina: 50, Power: 10, Gold: 0");
    println!("==========================================\n");

    let mut player = Player::new();

    // Main game loop
    loop {
        player.show_status();

        // Check win condition
        if player.has_won() {
            println!("🎉 CONGRATULATIONS! 🎉");
            println!("You have collected 100+ gold and won the game!");
            break;
        }

        // Check lose conditions
        if !player.is_alive() {
            println!("💀 GAME OVER! 💀");
            if player.hp <= 0 {
                println!("You have run out of HP!");
            } else if player.stamina <= 0 {
                println!("You have run out of stamina!");
            }
            println!("Better luck next time!");
            break;
        }

        // Get player input for movement
        match Player::get_direction_input() {
            Some(direction) => {
                player.move_direction(direction);

                // Check if player can still continue after movement
                if player.is_alive() {
                    player.handle_encounter();
                }
            }
            None => {
                println!("Thanks for playing!");
                player.show_status();
                break;
            }
        }

        println!("\n{}\n", "=".repeat(50));
    }

    println!("Final stats:");
    player.show_status();
}
