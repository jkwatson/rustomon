mod monster_loader;
mod data;
use rand::prelude::*;

fn main() {
    let monsters = monster_loader::get_monster_graph();
    println!("Loaded {} monsters", monsters.len());

    let mut rng = thread_rng();
    let monster = monsters.get(rng.gen_range(0..monsters.len() as u32)).unwrap();
    println!("{}\n", monster.summary());
    
    let pals = monsters.get_adjacent(monster, 3);
    for pal in pals {
        println!("\t {}", pal.summary());
    }
}
