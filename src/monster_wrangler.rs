use crate::data::{Monster, Monsters};
use rand::prelude::*;

pub struct MonsterWrangler {
    monsters: Monsters,
}

impl MonsterWrangler {
    pub fn new(monsters: Monsters) -> MonsterWrangler {
        MonsterWrangler { monsters }
    }

    pub fn choices(&self) -> Choices {
        Choices::default()
    }

    pub fn list(&self, choices: &Choices) -> Vec<Monster> {
        choices.apply_filters(&self.monsters)
    }
    
    pub fn rando(&self, choices: &Choices) -> Monster {
        choices.rando(&self.monsters)
    }

    pub fn search(&self, choices: &Choices, search_term: &String) -> Vec<Monster> {
        let search_term = search_term.to_lowercase();
        choices
            .apply_filters(&self.monsters)
            .iter()
            .filter(|&monster| {
                monster.name.to_lowercase().contains(&search_term)
                    || monster
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&search_term))
                    || monster
                        .biomes
                        .iter()
                        .any(|biome| biome.to_lowercase().contains(&search_term))
            })
            .map(|monster| monster.clone())
            .collect()
    }
}

#[derive(Default)]
pub struct Choices {
    level: Option<u8>,
    biome: Option<String>,
    tag: Option<String>,
    randomness: Option<u8>,
}

impl Choices {
    pub fn cluster(&self, number: i32, monster_wrangler: &MonsterWrangler) -> Vec<Monster> {
        let seed = self.rando(&monster_wrangler.monsters);
        let size = self.randomness.unwrap() as i32 * number;
        let adjacent = monster_wrangler.monsters.get_adjacent(&seed, size as u32);
        let mut result = vec![seed];
        result.shuffle(&mut thread_rng());
        let mut returned = 0;
        for monster in adjacent {
            returned += 1;
            if returned > number {
                break;
            }
            result.push(monster.clone());
        }
        result
    }

    pub fn rando(&self, monsters: &Monsters) -> Monster {
        let filtered = self.apply_filters(monsters);
        let mut rng = thread_rng();
        let index = rng.gen_range(0..filtered.len());
        filtered[index].clone()
    }

    fn apply_filters(&self, monsters: &Monsters) -> Vec<Monster> {
        monsters
            .all()
            .iter()
            .filter(|&&monster| match &self.biome {
                None => true,
                Some(biome) => monster.biomes.contains(biome),
            })
            .filter(|&&monster| match &self.level {
                None => true,
                Some(level) => monster.level == *level,
            })
            .filter(|&&monster| match &self.tag {
                None => true,
                Some(tag) => monster.tags.contains(tag),
            })
            .map(|&monster| monster.clone())
            .collect()
    }

    pub fn with_biome(&self, biome: String) -> Choices {
        Choices {
            level: self.level,
            biome: if biome.is_empty() { None } else { Some(biome) },
            tag: self.tag.clone(),
            randomness: self.randomness,
        }
    }

    pub fn with_tag(&self, tag: String) -> Choices {
        Choices {
            level: self.level,
            biome: self.biome.clone(),
            tag: if tag.is_empty() { None } else { Some(tag) },
            randomness: self.randomness,
        }
    }

    pub fn with_level(&self, level: Option<u8>) -> Choices {
        Choices {
            level,
            biome: self.biome.clone(),
            tag: self.tag.clone(),
            randomness: self.randomness,
        }
    }

    pub fn with_randomness(&self, randomness: Option<u8>) -> Choices {
        Choices {
            level: self.level,
            biome: self.biome.clone(),
            tag: self.tag.clone(),
            randomness,
        }
    }

    pub fn biomes(&self, wrangler: &MonsterWrangler) -> Vec<String> {
        let choices = self.with_biome(String::new());
        let mut all = choices.filter(&wrangler.monsters, |monster: &&Monster| {
            monster.biomes.clone()
        });
        all.sort_unstable();
        all.dedup();
        all
    }

    pub fn tags(&self, wrangler: &MonsterWrangler) -> Vec<String> {
        let choices = self.with_tag(String::new());
        let mut all = choices.filter(&wrangler.monsters, |monster: &&Monster| {
            monster.tags.clone()
        });
        all.sort_unstable();
        all.dedup();
        all
    }

    pub fn levels(&self, wrangler: &MonsterWrangler) -> Vec<u8> {
        let choices = self.with_level(None);
        let mut all = choices.filter(&wrangler.monsters, |monster: &&Monster| vec![monster.level]);
        all.sort_unstable();
        all.dedup();
        all
    }

    fn filter<T>(&self, monsters: &Monsters, x: fn(&&Monster) -> Vec<T>) -> Vec<T> {
        monsters
            .all()
            .iter()
            .filter(|monster| match &self.biome {
                None => true,
                Some(biome) => monster.biomes.contains(biome),
            })
            .filter(|monster| match &self.level {
                None => true,
                Some(level) => monster.level == *level,
            })
            .filter(|monster| match &self.tag {
                None => true,
                Some(tag) => monster.tags.contains(tag),
            })
            .flat_map(x)
            .collect()
    }

    pub(crate) fn state(&self) -> String {
        let mut result = "".to_string();
        result = match &self.level {
            None => result,
            Some(x) => result + &format!("level={}", x),
        };

        result = match &self.biome {
            None => result,
            Some(x) => {
                if result.is_empty() {
                    result + &format!("biome={}", x)
                } else {
                    result + &format!(", biome={}", x)
                }
            }
        };

        result = match &self.tag {
            None => result,
            Some(x) => {
                if result.is_empty() {
                    result + &format!("tag={}", x)
                } else {
                    result + &format!(", tag={}", x)
                }
            }
        };
        if result.is_empty() {
            "[]".to_string()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::monster_loader;
    use crate::monster_wrangler::{Choices, MonsterWrangler};

    #[test]
    fn state_empty() {
        let choices = Choices::default();
        assert_eq!(choices.state(), "[]");
    }

    #[test]
    fn state_biome() {
        let choices: Choices = Choices::default().with_biome(String::from("forest"));
        assert_eq!(choices.state(), "biome=forest");
    }

    #[test]
    fn state_level() {
        let choices = Choices::default().with_level(Some(4));
        assert_eq!(choices.state(), "level=4");
    }

    #[test]
    fn state_tag() {
        let choices = Choices::default().with_tag(String::from("cheese"));
        assert_eq!(choices.state(), "tag=cheese");
    }

    #[test]
    fn state_full() {
        let choices = Choices {
            level: Some(4),
            biome: Some(String::from("forest")),
            tag: Some(String::from("cheese")),
            randomness: None,
        };
        assert_eq!(choices.state(), "level=4, biome=forest, tag=cheese");
    }

    #[test]
    fn no_empty_biomes() {
        let monsters = monster_loader::get_monster_graph();
        let wrangler = MonsterWrangler::new(monsters);
        let biomes = wrangler.choices().biomes(&wrangler);
        assert!(!biomes.contains(&"".to_string()));
        assert!(!biomes.contains(&"*".to_string()));
    }
}
