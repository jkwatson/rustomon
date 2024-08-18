use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug)]
pub struct RawMonster {
    pub name: String,
    pub tags: String,
    pub level: String,
    pub biome: String,
    pub alignment: String,
    #[serde(rename = "move")]
    pub move_amount: String,
    pub attack: String,
    pub page: String,
    #[serde(rename = "statblock")]
    pub stat_block: String,
    pub source: String,
}

#[derive(Debug, Clone, Hash)]
pub struct Monster {
    pub id: MonsterId,
    pub name: String,
    pub tags: Vec<String>,
    pub level: u8,
    pub biomes: Vec<String>,
    pub alignment: String,
    pub move_amount: String,
    pub attack: String,
    pub page: String,
    pub raw_stat_block: String,
    pub source: String,
    pub stat_block: StatBlock,
}

#[derive(Debug, Clone, Hash)]
pub struct StatBlock {
    move_amount: String,
    attack: String,
    ac: String,
    hp: String,
    stats: String,
}

impl StatBlock {
    pub(crate) fn parse(full: &String) -> StatBlock {
        let pieces: Vec<&str> = full.split(",").collect();

        StatBlock {
            ac: pieces[0].to_string(),
            hp: pieces[1].to_string(),
            attack: pieces[2].trim().to_string(),
            move_amount: pieces[3].to_string(),
            stats: pieces[4..10].join(",").trim().to_string(),
        }
    }
}

impl Monster {
    pub fn summary(&self) -> String {
        format!("{}: {} {}", self.name, self.level, self.raw_stat_block, )
    }

    pub fn detailed_summary(&self) -> String {
        format!("{} [ref: {}]\n\t{}\t{}\t{}\tLV:{}\tAL:{}\n\t{}\n\t{}", self.name, self.page, self.stat_block.ac, self.stat_block.hp, self.stat_block.move_amount, self.level, self.alignment, self.stat_block.attack, self.stat_block.stats)
    }
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type Strength = i32;
pub type MonsterId = u32;

#[derive(Default)]
pub struct Monsters {
    vertices: HashMap<MonsterId, Monster>,
    adjacency: HashMap<MonsterId, Vec<(MonsterId, Strength)>>,
}

impl Monsters {
    pub fn new(monsters: Vec<Monster>) -> Monsters {
        let mut graph = Monsters::default();
        graph.build_graph(monsters);
        graph
    }

    pub fn all(self: &Monsters) -> Vec<&Monster> {
        self.vertices.values().collect()
    }

    pub fn len(self: &Monsters) -> usize {
        self.vertices.len()
    }

    fn build_graph(self: &mut Monsters, vs: Vec<Monster>) {
        for v in vs {
            self.vertices.insert(v.id, v);
        }
        for v in self.vertices.values() {
            for other in self.vertices.values() {
                if v == other {
                    continue;
                }
                let strength = Self::calculate_connection_strength(v, other);
                if strength > 0 {
                    let adjacent_to_from = self.adjacency.entry(v.id).or_default();
                    adjacent_to_from.push((other.id, strength));
                }
            }
        }
        for adjacents in self.adjacency.values_mut() {
            adjacents.sort_by(|a, b| b.1.cmp(&a.1));
        }
    }

    fn calculate_connection_strength(m1: &Monster, m2: &Monster) -> Strength {
        let common_tags = m1.tags.iter().filter(|t| m2.tags.contains(t)).count() as i32;
        let common_biomes = m1.biomes.iter().filter(|t| m2.biomes.contains(t)).count() as i32;
        let level_strength = 30 - 10 * (m1.level as i32 - m2.level as i32).abs();
        let alignment_bonus = if m1.alignment == m2.alignment { 30 } else { 0 };
        let biome_bonus = 20 * common_biomes.min(3);
        let tag_bonus = 10 * common_tags;
        let source_bonus = if m1.source == m2.source { 10 } else { 0 };
        level_strength + tag_bonus + biome_bonus + alignment_bonus + source_bonus
    }

    pub fn get_adjacent(self: &Monsters, seed: &Monster, limit: u32) -> Vec<&Monster> {
        let mut adjacent = Vec::new();
        let mut count = 0;
        if let Some(neighbors) = self.adjacency.get(&seed.id) {
            for (id, _) in neighbors {
                if count >= limit {
                    break;
                }
                if let Some(neighbor) = self.vertices.get(id) {
                    adjacent.push(neighbor);
                }
                count = count + 1;
            }
        }
        adjacent
    }
}
