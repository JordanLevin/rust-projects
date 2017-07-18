extern crate readline;
extern crate rand;

use readline::readline;
use rand::Rng;
    
const SIZE: usize = 20;

#[derive(Debug, Copy, Clone)]
enum TileType {
    Forest,
    Field,
    Mountain,
    Water,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    pub Type: TileType,
    pub enemy_chance: i32,
    pub loot_chance: i32,
    pub difficulty: i32,
}

impl Tile {
    pub fn default() -> Tile {
        Tile {Type: TileType::Field, enemy_chance: 0, loot_chance: 0, difficulty: 0}
    }

    pub fn new(Type: TileType, enemy_chance: i32, loot_chance: i32, difficulty: i32) -> Tile {
        Tile {Type: Type, enemy_chance: enemy_chance, loot_chance: loot_chance, difficulty:difficulty}
    }
}

#[derive(Debug, Copy, Clone)]
struct World {
    pub Tiles: [[Tile; SIZE]; SIZE],
}

impl World {
    pub fn new() -> World {
        let mut tiles: [[Tile; SIZE]; SIZE] = [[Tile::default(); SIZE]; SIZE];
        for i in 0..20 {
            for j in 0..20 {
                let diff: i32 = rand::thread_rng().gen_range(15, 30) + (i+j);
                //TODO enemy chance and loot chance are temp, better system later
                tiles[i as usize][j as usize] = Tile::new(TileType::Field, diff, diff, diff);
            }
        }
        World {Tiles: tiles}
    }
}

#[derive(Debug)]
struct Player {
    pub name: String,
    pub level: i32,
    pub inv: [Item; 10],
    pub attack: i32,
    pub defense: i32,
    pub health: i32,
    pub x: i32,
    pub y: i32,
}

impl Player {
   pub fn new(name: String, level: i32) -> Player {
       Player {name: name, level: level, inv: [Item::new(0, 0); 10], x: 0, y: 0}
   } 
}


#[derive(Debug, Copy, Clone)]
struct Item {
    pub damage: i32,
    pub durability: i32,
}

impl Item {
    pub fn new(damage: i32, durability: i32) -> Item {
        Item {damage: damage, durability: durability}
    }
}

#[derive(Debug)]
struct Enemy {
    pub class: String,
    pub level: i32,
    pub attack: i32,
    pub defense: i32,
    pub heaktg: i32,
}

impl Enemy{
    pub fn new(name: String, level: i32) -> Enemy {
        //TODO attack and level are temp, will make a better system later
        Enemy(class: class, level: level, attack: level, defense: level)
    }
}

fn fight(player: mut &Player, enemies: &[Enemy]){

}

fn small_fight(player: mut &Player enemy: &Enemy){
    
}

fn print_world(world: &World){
    for i in world.Tiles.iter(){
        for j in i{
            print!("{} ", j.difficulty);
        }
        println!("");
    }
}

fn main() {
    let name: String = match readline(&String::from("Enter a name: ")){
        Ok(val) => val,
        Err(_) => String::from("Test"),
    };

    let player: Player = Player::new(name, 0);
    println!("Name chosen is: {}", player.name);
    let world: World = World::new();
    print_world(&world);
}
