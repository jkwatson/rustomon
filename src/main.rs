mod monster_loader;
mod data;

fn main() {
    let monsters = monster_loader::get_monster_graph();
    println!("Loaded {} monsters", monsters.len());
}
