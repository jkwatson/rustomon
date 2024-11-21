use crate::data::{Monsters, Monster, RawMonster, MonsterId, StatBlock};

pub fn get_monster_graph() -> Monsters {
    Monsters::new(load_monsters())
}

pub fn load_monsters() -> Vec<Monster> {
    let mut raw_monsters = Vec::new();
    let files = vec![
        include_str!("core.json"),
        include_str!("cs1.json"),
        include_str!("cs2.json"),
        include_str!("cs3.json"),
        include_str!("custom.json"),
        include_str!("unnatural_selection.json"),
        include_str!("dragontown.json"),
    ];
    for file in files {
        raw_monsters.append(&mut load_raw_monsters(file));
    }

    convert_to_monsters(raw_monsters)
}

fn load_raw_monsters(file: &str) -> Vec<RawMonster> {
    let raw_monsters = match serde_json::from_str::<Vec<RawMonster>>(file) {
        Ok(monsters) => monsters,
        Err(e) => {
            eprintln!("Error parsing file {}: {}", file, e);
            std::process::exit(1);
        }
    };
    raw_monsters
}

fn convert_to_monsters(raw_monsters: Vec<RawMonster>) -> Vec<Monster> {
    let mut id : MonsterId = 0;
    let monsters = raw_monsters.iter().map(|monster| {
        let monster = Monster {
            id,
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
                .filter(|b| !b.is_empty())
                .collect(),
            alignment: monster.alignment.clone(),
            move_amount: monster.move_amount.clone(),
            attack: monster.attack.clone(),
            page: monster.page.clone(),
            raw_stat_block: monster.stat_block.clone(),
            source: monster.source.clone(),
            stat_block: StatBlock::parse(&monster.stat_block)
        };
        id = id + 1;
        monster
    });
    return monsters.collect();
}
