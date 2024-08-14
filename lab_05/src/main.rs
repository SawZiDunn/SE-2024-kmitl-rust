use rand::Rng;
use std::io;

struct Player {
    hp: i32,
    power: i32,
    stamina: i32,
    gold: i32,
    steps: i32,
    actions: i32,
}

#[derive(Clone)]
struct Enemy {
    name: String,
    hp: i32,
    stamina: i32,
    power: i32,
    full_hp: i32,
}

enum EncounterType {
    Nothing,
    Bush,
    Meat,
    Water,
    Herb,
    IronOre,
    Enemy(EnemyType),
}

enum EnemyType {
    Rat,
    Wolf,
    Boar,
    Tiger,
    Dragon,
}

enum Direction {
    North,
    South,
    West,
    East,
}

impl Player {
    fn new() -> Player {
        Player {
            hp: 100,
            stamina: 50,
            power: 10,
            gold: 0,
            steps: 0,
            actions: 0,
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0 && self.stamina > 0
    }

    fn walk(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                println!("You just walked to North!");
            }
            Direction::East => {
                println!("You just walked to East!");
            }
            Direction::South => {
                println!("You just walked to South!");
            }
            Direction::West => {
                println!("You just walked to West!");
            }
        }

        self.steps += 1;
        self.stamina -= 1;
        println!("Your stamina is now {}.", self.stamina);
    }

    fn encounter(&mut self, enemies: &Vec<Enemy>) {
        let mut rng = rand::thread_rng();
        let encounter_type: EncounterType = match rng.gen_range(1..=100) {
            1..=17 => EncounterType::Nothing,
            18..=25 => EncounterType::Bush,
            26..=40 => EncounterType::Meat,
            41..=55 => EncounterType::Water,
            56..=70 => EncounterType::Herb,
            71..=75 => EncounterType::IronOre,
            76..=100 => {
                let enemy_type: EnemyType = match rng.gen_range(1..=100) {
                    1..=45 => EnemyType::Rat,
                    46..=70 => EnemyType::Wolf,
                    71..=85 => EnemyType::Wolf,
                    86..=95 => EnemyType::Tiger,
                    _ => EnemyType::Dragon,
                };
                EncounterType::Enemy(enemy_type)
            }
            _ => unreachable!(),
        };

        match encounter_type {
            EncounterType::Nothing => {
                println!("You found nothing!\n");
            }
            EncounterType::Bush => {
                println!("You bumped into a bush!\n-2 Stamina");
                self.stamina -= 1;
            }
            EncounterType::Meat => {
                println!("You found meat!\n+1 Power\n-1 Stamina");
                self.hp += 4;
            }
            EncounterType::Water => {
                println!("You just found Water Source!\n+2 Stamina");

                self.stamina += 2;
            }

            EncounterType::Herb => {
                println!("You bumped into a herb!\n+1 Power\n-1 Stamina");
                self.power += 1;
            }

            EncounterType::IronOre => {
                println!("You found IronOre!\n+10 Power\n-1 Stamina");
                self.power += 10;
            }

            EncounterType::Enemy(enemy_type) => {
                self.stamina -= 1;
                let enemy = self.create_enemy(&enemies, &enemy_type);
                println!("You bumped into a {}.", enemy.name);
                self.fight_or_flee(enemy);
            }
        }
        self.actions += 1;
    }

    fn create_enemy(&self, enemies: &Vec<Enemy>, enemy_type: &EnemyType) -> Enemy {
        match enemy_type {
            EnemyType::Rat => enemies[0].clone(),
            EnemyType::Boar => enemies[1].clone(),
            EnemyType::Tiger => enemies[2].clone(),
            EnemyType::Dragon => enemies[3].clone(),
            EnemyType::Wolf => enemies[4].clone(),
        }
    }

    fn show_status(&self) {
        println!(
            "Your Status:\nHP: {},\nStamina: {},\nPower: {},\nSteps: {},\nGold: {},\nActions: {}",
            self.hp, self.stamina, self.power, self.steps, self.gold, self.actions
        );
    }

    fn fight_or_flee(&mut self, mut enemy: Enemy) {
        let mut rng = rand::thread_rng();
        loop {
            println!(
                "Player - HP: {}, Stamina: {}, Power: {}, Gold: {}",
                self.hp, self.stamina, self.power, self.gold
            );
            println!(
                "Enemy - HP: {}, Stamina: {}, Power: {}",
                enemy.hp, enemy.stamina, enemy.power
            );
            println!("Do you want to (1)fight or (2)flee");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input!");
            let choice: u32 = choice.trim().parse().expect("Failed to read input!");

            match choice {
                1 => {
                    self.stamina -= 1;
                    let hit_chance: i32 = rng.gen_range(0..=100);
                    if hit_chance <= (100 - enemy.stamina / 2) {
                        enemy.hp -= self.power;
                        println!("You hit the enemy!");
                        println!("Enemy HP is now {}", enemy.hp);
                    } else {
                        println!("You missed!");
                    }

                    if enemy.hp <= 0 {
                        println!("You defeated the {}", enemy.name);
                        self.gold += enemy.full_hp;
                        println!("You just killed!");
                        println!("You get {} Gold.", enemy.full_hp);
                        println!("Your total Gold is now {}.", self.gold);
                        break;
                    }

                    let enemy_hit_chance: i32 = rng.gen_range(1..=100);
                    if enemy_hit_chance <= (100 - self.stamina / 2) {
                        self.hp -= enemy.power;
                        println!("You got hit!");
                        println!("Your current hp is {}.", self.hp);
                    } else {
                        println!("The enemy missed!");
                    }

                    if self.hp <= 0 {
                        println!("You are dead!");
                        break;
                    }
                }

                2 => {
                    self.stamina -= 2;
                    let flee_chance = rng.gen_range(1..=100);
                    if flee_chance <= 90 + self.stamina - enemy.stamina {
                        println!("You fled successfully!");
                        break;
                    } else {
                        println!("You could not flee!");
                        let enemy_hit_chance: i32 = rng.gen_range(1..=100);
                        if enemy_hit_chance <= 100 - self.stamina / 2 {
                            self.hp -= enemy.power;
                            println!("You got hit!");
                            println!("Your current hp is {}.", self.hp);
                        } else {
                            println!("The enemy missed!");
                        }

                        if self.hp <= 0 {
                            println!("You are dead!");
                            break;
                        }
                    }
                }
                _ => {
                    println!("Invalid Choice!\nPlease choose either 1 or 2!");
                    continue;
                }
            }
        }
    }
}

fn main() {
    let enemies: Vec<Enemy> = vec![
        Enemy {
            name: String::from("Rat"),
            hp: 10,
            stamina: 10,
            power: 2,
            full_hp: 10,
        },
        Enemy {
            name: String::from("Wolf"),
            hp: 20,
            stamina: 20,
            power: 10,
            full_hp: 20,
        },
        Enemy {
            name: String::from("Boar"),
            hp: 30,
            stamina: 40,
            power: 20,
            full_hp: 30,
        },
        Enemy {
            name: String::from("Tiger"),
            hp: 40,
            stamina: 50,
            power: 30,
            full_hp: 40,
        },
        Enemy {
            name: String::from("Dragon"),
            hp: 60,
            stamina: 60,
            power: 40,
            full_hp: 60,
        },
    ];

    let mut player = Player::new();
    println!("Welcome to the Adventure Game!");

    while player.is_alive() && player.gold < 200 {
        println!("Choose a direction to walk:\n(1) North, (2) East, (3) South, (4) West, (9) Exit");
        let mut direction = String::new();
        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read input!");
        let direction: u32 = direction.trim().parse().expect("Must be a number!");

        if direction == 9 {
            player.show_status();
            break;
        }

        let direction: Direction = match direction {
            1 => Direction::North,
            2 => Direction::East,
            3 => Direction::South,
            4 => Direction::West,
            _ => {
                println!("Invalid Choice!");
                continue;
            }
        };

        player.walk(direction);

        player.encounter(&enemies);

        if player.gold >= 200 {
            println!(
                "Congratulations! You have reached 200 Gold Coins. You have conqured the game!"
            );
            player.show_status();
            break;
        } else if !player.is_alive() {
            println!("Game Over!\nYou are dead!");
            player.show_status();
            break;
        }
    }
}
