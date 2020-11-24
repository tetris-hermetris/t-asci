use date_time::date_tuple::DateTuple;
use date_time::time_tuple::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct Spores(Vec<Spore>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Spore {
    pub tag: String,
    pub id: u16,
    pub next: Vec<u16>,
    pub prev: Vec<u16>,
    pub date: String,
    pub dur: String,
    pub state: SporeState,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SporeState {
    toDoState,
    DoingState,
    DoneState,
}

impl Spores {
    pub fn new() -> Spores {
        Spores(Vec::new())
    }

    pub fn insert_spore(&mut self, tag: String) {
        let spore: Spore = Spore {
            tag: tag.clone(),
            id: tag.into_bytes().iter().copied().map(u16::from).sum::<u16>(),
            next: vec![],
            prev: vec![],
            date: DateTuple::today().to_string(),
            dur: Duration::new(01, 00, 00).to_string(),
            state: SporeState::toDoState,
        };
        &self.0.push(spore);
    }

    pub fn len(self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for Spores {
    type Item = Spore;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_spores() {
        let spores = Spores::new();
        assert_eq!(spores, Spores::new())
    }

    #[test]
    fn test_insert_spore() {
        let mut spores = Spores::new();
        spores.insert_spore("new_spore".to_string());
        assert_eq!(1, spores.len())
    }

    #[test]
    fn test_iter_spores() {
        let mut spores = Spores::new();
        spores.insert_spore("new_spore".to_string());
        let s: Vec<Spore> = spores.clone().into_iter().collect();
        assert_eq!(spores.0, s)
    }
}
