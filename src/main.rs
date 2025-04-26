mod data;
mod monster_loader;
mod monster_wrangler;

use crate::data::Monster;
use crate::monster_wrangler::{Choices, MonsterWrangler};

fn main() {
    let monsters = monster_loader::get_monster_graph();
    println!("Loaded {} monsters", monsters.len());

    let wrangler = MonsterWrangler::new(monsters);
    let mut choices = wrangler.choices();
    loop {
        choices = choose(&wrangler, choices);
        let randomness = read_randomness();
        choices = choices.with_randomness(Some(randomness));
        println!("Choices: {}, Randomness: {}", choices.state(), randomness);
        choices.cluster(5, &wrangler).iter().for_each(|monster| {
            println!("{}", monster.detailed_summary());
        });
    }
}

fn read_randomness() -> u8 {
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
            "\nChoose: [1:Level, 2:Biome, 3:Tag, 4: Search, 5: List, 6: Random, 7: Walk Group, g: Generate Group] (current: {}):",
            choices.state()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.is_empty() || input.trim() == "g" {
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
                let seed_monster = search(wrangler, &choices);
                if seed_monster.is_some() {
                    choices = choices.with_seed_monster(seed_monster);
                }
            }
            Ok(5) => {
                let monsters = wrangler.list(&choices);
                for monster in monsters {
                    println!("{}", monster.detailed_summary());
                }
            }
            Ok(6) => {
                let monster = wrangler.rando(&choices);
                println!("{}", monster.detailed_summary());

                // Ask if the user wants to use this monster as a seed
                println!("\nWould you like to use this monster as a seed? (y/n):");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim().to_lowercase() == "y" {
                    choices = choices.with_seed_monster(Some(monster.clone()));
                    println!("Selected seed monster: {}", monster.name);
                }
            }
            Ok(7) => {
                choices.walk(5, &wrangler).iter().for_each(|monster| {
                    println!("{}", monster.detailed_summary());
                });
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
    choices
}

fn search(wrangler: &MonsterWrangler, choices: &Choices) -> Option<Monster> {
    let mut search_term = String::new();
    std::io::stdin().read_line(&mut search_term).unwrap();
    let search = search_term.trim().to_string();
    let results = wrangler.search(&choices, &search);

    if results.is_empty() {
        println!("No monsters found matching that search term.");
        return None;
    }

    // Display the search results with numbers
    for (i, monster) in results.iter().enumerate() {
        println!("{}. {}", i + 1, monster.detailed_summary());
    }

    // Ask if the user wants to use one as a seed monster
    println!("\nWould you like to use one of these monsters as a seed? Enter the number (or 0 to skip):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<usize>().unwrap_or(0);

    if choice == 0 || choice > results.len() {
        return None;
    }

    let selected_monster = results[choice - 1].clone();
    println!("Selected seed monster: {}", selected_monster.name);
    Some(selected_monster)
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
