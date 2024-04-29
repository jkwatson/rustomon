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
    pub statblock: String,
    pub source: String,
}

#[derive(Debug, Clone, Hash)]
pub struct Monster {
    pub id: MonsterId,
    pub name: String,
    pub tags: Vec<String>,
    pub level: i32,
    pub biomes: Vec<String>,
    pub alignment: String,
    pub move_amount: String,
    pub attack: String,
    pub page: String,
    pub statblock: String,
    pub source: String,
}

impl Monster {
    pub(crate) fn summary(&self) -> String {
        format!("{}: {} {}", self.name, self.level, self.statblock,)
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

    pub fn len(self: &Monsters) -> usize {
        self.vertices.len()
    }

    pub fn get(self: &Monsters, id: MonsterId) -> Option<&Monster> {
        self.vertices.get(&id)
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
        let level_strength = 30 - 10 * (m1.level - m2.level).abs();
        let alignment_bonus = if m1.alignment == m2.alignment { 30 } else { 0 };
        let biome_bonus = 20 * common_biomes.min(3);
        let tag_bonus = 10 * common_tags;
        level_strength + tag_bonus + biome_bonus + alignment_bonus
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
