use crate::data::{Monster, RawMonster};

pub fn load_monsters() -> Vec<Monster> {
    let core_file = include_str!("core.json");

    let raw_monsters = load_raw_monsters(core_file);
    let monsters = convert_to_monsters(raw_monsters);
    monsters
}

pub fn load_raw_monsters(core_file: &str) -> Vec<RawMonster> {
    let raw_monsters = match serde_json::from_str::<Vec<RawMonster>>(core_file) {
        Ok(monsters) => monsters,
        Err(e) => {
            eprintln!("Error parsing core.json: {}", e);
            std::process::exit(1);
        }
    };
    raw_monsters
}

pub fn convert_to_monsters(
    raw_monsters: Vec<RawMonster>,
) -> Vec<Monster> {
    let monsters = raw_monsters.iter().map(|monster| Monster {
        name: monster.name.clone(),
        tags: monster
            .tags
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
        level: if monster.level == "*" {
            10
        } else {
            monster.level.parse().unwrap()
        },
        biomes: monster
            .biome
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
        alignment: monster.alignment.clone(),
        move_amount: monster.move_amount.clone(),
        attack: monster.attack.clone(),
        page: monster.page.clone(),
        statblock: monster.statblock.clone(),
        source: monster.source.clone(),
    });
    monsters.collect()
}
