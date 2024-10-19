use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
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
    pub fn detailed_summary(&self) -> String {
        format!("{} [ref: {}]\n\t{}\t{}\t{}\tLV:{}\tAL:{}\n\t{}\n\t{}", self.name, self.page, self.stat_block.ac, self.stat_block.hp, self.stat_block.move_amount, self.level, self.alignment, self.stat_block.attack, self.stat_block.stats)
    }
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type MonsterId = u32;

pub struct Strength {
    pub total: i32,
    pub level: i32,
    pub tag: i32,
    pub biome: i32,
    pub alignment: i32,
    pub source: i32,
}

impl fmt::Display for Strength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "strength: {} (lvl: {} tag: {} bio: {} al: {} src: {})", self.total, self.level, self.tag, self.biome, self.alignment, self.source)
    }
}

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
                let strength = Self::calculate_connection_strength(v, other, false);
                if strength.total > 0 {
                    let adjacent_to_from = self.adjacency.entry(v.id).or_default();
                    adjacent_to_from.push((other.id, strength));
                }
            }
        }
        for adjacents in self.adjacency.values_mut() {
            adjacents.sort_by(|a, b| b.1.total.cmp(&a.1.total));
        }
    }

    fn calculate_connection_strength(m1: &Monster, m2: &Monster, debug: bool) -> Strength {
        let common_tags = m1.tags.iter().filter(|t| m2.tags.contains(t)).count() as i32;
        let common_biomes: i32;
        if m1.biomes.contains(&"*".to_string()) {
            common_biomes = m2.biomes.len() as i32;
        } else if m2.biomes.contains(&"*".to_string()) {
            common_biomes = m1.biomes.len() as i32;
        } else {
            common_biomes = m1.biomes.iter().filter(|t| m2.biomes.contains(t)).count() as i32;
        }
        
        let level_strength = 10 - 7 * (m1.level as i32 - m2.level as i32).abs();
        let alignment_bonus = if m1.alignment == m2.alignment { 15 } else { 0 };
        let biome_bonus = 5 * common_biomes.min(3);
        let tag_bonus = 10 * common_tags;
        let source_bonus = if m1.source == m2.source { 10 } else { 0 };
        let result = level_strength + tag_bonus + biome_bonus + alignment_bonus + source_bonus;
        let strength = Strength {
            total: result,
            level: level_strength,
            tag: tag_bonus,
            biome: biome_bonus,
            alignment: alignment_bonus,
            source: source_bonus,
        };
        if debug {
            println!(
                "{} -> {}: {} = {}",
                m1.name, m2.name, result, strength
            );
        }
        return strength;
    }

    pub fn get_adjacent(self: &Monsters, seed: &Monster, limit: u32) -> Vec<&Monster> {
        let mut adjacent = Vec::new();
        let mut count = 0;
        if let Some(neighbors) = self.adjacency.get(&seed.id) {
            for (id, _) in neighbors {
                if count >= limit {
                    break;
                }
                let neighbor = self.vertices.get(id);
                Self::calculate_connection_strength(seed, neighbor.unwrap(), true);
                if let Some(neighbor) = neighbor {
                    adjacent.push(neighbor);
                }
                count = count + 1;
            }
        }
        adjacent
    }
    
    pub fn get_neighbor_excluding(self: &Monsters, seed: &Monster, excluding: &Vec<Monster>, distance: &i32) -> &Monster {
        let options: &Vec<(MonsterId, Strength)> = self.adjacency.get(&seed.id).unwrap();
        let excluded_ids: Vec<_>= excluding.iter().map(|m| m.id).collect();
        let mut count = 0;
        for (id, strength) in options {
            count = count + 1;
            if count < *distance {
                continue;
            }
            if !excluded_ids.contains(id) {
                println!("{} -> {} @ {}: {}", seed.name, self.vertices.get(id).unwrap().name, distance, strength);
                return self.vertices.get(id).unwrap();
            }
        }
        panic!("No valid neighbors found");
    }
}
