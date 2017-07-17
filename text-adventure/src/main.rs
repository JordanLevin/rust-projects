extern crate readline;
extern crate rand;

use readline::readline;
use rand::Rng;
    
const SIZE: i32 = 20;

enum TileType {
    Forest,
    Field,
    Mountain,
    Water,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    pub Type: TileType,
    pub EnemyChance: i32,
    pub LootChance: i32,
    pub Difficulty: i32,
}

impl Tile {
    pub fn default() -> Tile {
        Tile {Type: TileType::Plains, EnemyChance: 0, LootChance: 0, Difficulty: 0}
    }

    pub fn new(Type: TileType, EnemyChance: i32, LootChance: i32, Difficulty: i32) -> Tile {
        Tile {Type: Type, EnemyChance: EnemyChance, LootChance: LootChance, Difficulty:Difficulty}
    }
}

#[derive(Debug, Copy, Clone)]
struct World {
    pub Tiles: [[Tile; SIZE]; SIZE],
}

impl World {
    pub fn new() -> World {
        tiles: [[Tile; SIZE]; SIZE] = [[Tile::default; SIZE]; SIZE];
        for i in 0..20 {
            for j in 0..20 {
                let diff = rand::thread_rng().gen_range(1, 100) * (i+j)/10;
                tiles[i][j] = Tile::new(TileType::Field, EnemyChance: diff, 
                                        LootChance: diff, Difficulty: diff);
            }
        }
        World {Tiles: tiles}
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
    pub name: String,
    pub level: i32,
    pub inv: [Item; 10],
}

impl Player {
   pub fn new(name: String, level: i32) -> Player {
       Player {name: name, level: level, inv: [Item::new(0, 0); 10]}
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

//fn world_gen() -> World{
    
//}

fn main() {
    let name: String = match readline(&String::from("Enter a name: ")){
        Ok(val) => val,
        Err(_) => String::from("Test"),
    };

    let player: Player = Player::new(name, 0);
    println!("Name chosen is: {}", player.name);
}
