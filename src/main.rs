mod data;
mod monster_loader;
mod monster_wrangler;

use crate::monster_wrangler::{Choices, MonsterWrangler};

fn main() {
    let monsters = monster_loader::get_monster_graph();
    println!("Loaded {} monsters", monsters.len());

    let wrangler = MonsterWrangler::new(monsters);
    let mut choices = wrangler.choices();
    loop {
        choices = choose(&wrangler, choices);
        let randomness = read_randomness();
        choices = choices.with_randomness(Some(randomness as u8));
        println!("Choices: {}, Randomness: {}", choices.state(), randomness);
        choices.cluster(5, &wrangler).iter().for_each(|monster| {
            println!("{}", monster.summary());
        });
    }
}

fn read_randomness() -> i32 {
    loop {
        println!("Randomness? [1-5] (default 1):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let randomness = input.trim().parse().unwrap_or(1);
        if (randomness < 1) || (randomness > 5) {
            println!("Invalid randomness");
            continue;
        }
        return randomness;
    }
}

fn choose(wrangler: &MonsterWrangler, choices: Choices) -> Choices {
    let mut choices = choices;
    loop {
        println!(
            "\nChoose: [1:Level, 2:Biome, 3:Tag, 4: Search, 5: List, q:done] (current: {}):",
            choices.state()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.is_empty() || input.trim() == "q" {
            break;
        }

        let choice = input.trim().parse();
        match choice {
            Ok(1) => {
                let level = choose_level(&wrangler, &choices);
                choices = choices.with_level(level);
            }
            Ok(2) => {
                let biome = choose_biome(&wrangler, &choices);
                choices = choices.with_biome(biome);
            }
            Ok(3) => {
                let tag = choose_tag(&wrangler, &choices);
                choices = choices.with_tag(tag);
            }
            Ok(4) => {
                println!("Search: ");
                search(wrangler, &choices);
            }
            Ok(5) => {
                let monsters = wrangler.list(&choices);
                for monster in monsters {
                    println!("{}", monster.summary());
                }
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
    choices
}

fn search(wrangler: &MonsterWrangler, choices: &Choices) {
    let mut search_term = String::new();
    std::io::stdin().read_line(&mut search_term).unwrap();
    let search = search_term.trim().to_string();
    let results = wrangler.search(&choices, &search);
    for monster in results {
        println!("{}", monster.summary());
    }
}

fn choose_tag(wrangler: &&MonsterWrangler, choices: &Choices) -> String {
    println!("tag? (default random) {:?}: ", choices.tags(&wrangler));
    loop {
        let mut tag = String::new();
        std::io::stdin().read_line(&mut tag).unwrap();
        let tag = tag.trim().to_string();
        if choices.tags(&wrangler).contains(&tag) || tag.is_empty() {
            return tag;
        }
        println!("Please choose a valid tag (or none)");
    }
}

fn choose_biome(wrangler: &&MonsterWrangler, choices: &Choices) -> String {
    println!("biome? (default random) {:?}: ", choices.biomes(&wrangler));
    loop {
        let mut biome = String::new();
        std::io::stdin().read_line(&mut biome).unwrap();
        let biome = biome.trim().to_string();
        if (choices.biomes(&wrangler).contains(&biome)) || biome.is_empty() {
            return biome;
        }
        println!("Please choose a valid biome (or none)");
    }
}

fn choose_level(wrangler: &&MonsterWrangler, choices: &Choices) -> Option<u8> {
    println!(
        "dungeon level? (default random) {:?}: ",
        choices.levels(&wrangler)
    );
    loop {
        let mut level = String::new();
        std::io::stdin().read_line(&mut level).unwrap();
        let level_choice = level.trim();
        if level_choice.is_empty() {
            return None;
        }
        let level = match level_choice.parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Level must be a number");
                continue;
            },
        };
        if (choices.levels(&wrangler).contains(&level)) || level == 0 {
            return Some(level);
        }
        println!("Please choose a valid level (or none)");
    }
}
