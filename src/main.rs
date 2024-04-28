mod monster_loader;
mod data;

fn main() {
    let monsters = monster_loader::load_monsters();
    println!("Loaded {} monsters", monsters.len());
}
