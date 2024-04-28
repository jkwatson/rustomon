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
    pub id: u32,
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

pub type Strength = i32;
pub type MonsterId = u32;

pub struct Graph<VId, E = (), V = ()> {
    vertices: HashMap<VId, V>,
    adjacency: HashMap<VId, Vec<(VId, E)>>,
}

impl<VId, E, V> Graph<VId, E, V>
where
    VId: Eq + Hash + Clone,
    V: Hash,
    E: Clone,
{
    pub fn new() -> Graph<VId, E, V> {
        Graph {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn push_vertex(self: &mut Graph<VId, E, V>, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(self: &mut Graph<VId, E, V>, from: VId, to: VId, edge: E) {
        let adjacent_to_from = self.adjacency.entry(from).or_default();
        adjacent_to_from.push((to, edge));
    }

    pub fn len(self: &Graph<VId, E, V>) -> usize {
        self.vertices.len()
    }

    pub fn get_vertex(self: &Graph<VId, E, V>, vid: &VId) -> Option<&V> {
        self.vertices.get(vid)
    }
}
