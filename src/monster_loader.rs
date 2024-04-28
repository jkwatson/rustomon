use crate::data::{Graph, Monster, MonsterId, RawMonster, Strength};

pub fn get_monster_graph() -> Graph<MonsterId, Strength, Monster> {
    build_graph(load_monsters())
}

fn build_graph(monsters: Vec<Monster>) -> Graph<MonsterId, Strength, Monster> {
    let mut graph: Graph<MonsterId, Strength, Monster> = Graph::new();
    let mut size: u32 = 0;
    for monster in monsters {
        size += 1;
        graph.push_vertex(monster.id, monster);
    }
    for id in 0..size {
        //clone the monster here so we can use it in the loop below
        let monster = graph.get_vertex(&id).unwrap().clone();
        for other_id in 0..size {
            if id == other_id {
                continue;
            }
            let other_monster = graph.get_vertex(&other_id).unwrap();
            let strength = calculate_connection_strength(&monster, other_monster);
            if strength > 0 {
                graph.push_edge(id, other_id, strength);
            }
        }
    }
    return graph;
}

fn calculate_connection_strength(m1: &Monster, m2: &Monster) -> Strength {
    let common_tags = m1.tags.iter().filter(|t| m2.tags.contains(t)).count() as i32;
    let common_biomes = m1.biomes.iter().filter(|t| m2.biomes.contains(t)).count() as i32;
    let level_strength = 30 - 10 * (m1.level - m2.level).abs();
    let alignment_bonus = if m1.alignment == m2.alignment { 30 } else { 0 };
    let biome_bonus = 20 * common_biomes.min(3);
    let tag_bonus = 10 * common_tags;
    level_strength + tag_bonus + biome_bonus + alignment_bonus
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
    ];
    for file in files {
        raw_monsters.append(&mut load_raw_monsters(file));
    }

    return convert_to_monsters(raw_monsters);
}

pub fn load_raw_monsters(file: &str) -> Vec<RawMonster> {
    let raw_monsters = match serde_json::from_str::<Vec<RawMonster>>(file) {
        Ok(monsters) => monsters,
        Err(e) => {
            eprintln!("Error parsing file {}: {}", file, e);
            std::process::exit(1);
        }
    };
    return raw_monsters;
}

pub fn convert_to_monsters(raw_monsters: Vec<RawMonster>) -> Vec<Monster> {
    let mut id = 0;
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
                .collect(),
            alignment: monster.alignment.clone(),
            move_amount: monster.move_amount.clone(),
            attack: monster.attack.clone(),
            page: monster.page.clone(),
            statblock: monster.statblock.clone(),
            source: monster.source.clone(),
        };
        id = id + 1;
        monster
    });
    return monsters.collect();
}
