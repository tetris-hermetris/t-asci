use date_time::date_tuple::DateTuple;
use date_time::time_tuple::Duration;

#[derive(Debug, PartialEq, Clone)]
pub struct Spores<'a> (Vec<Spore<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Spore<'a> {
    pub tag: String,
    pub id: u16,
    pub next: Vec<&'a Spore<'a>>,
    pub prev: Vec<&'a Spore<'a>>,
    pub date: DateTuple,
    pub dur: Duration,
    pub state: SporeState,
}

impl Spores<'static> {
    
    pub fn new() -> Spores<'static> {
        Spores (Vec::new())
    }

    pub fn insert_spore(&mut self, tag: String) {
        let spore: Spore = Spore {
            tag: tag.clone(),
            id: tag.into_bytes().iter().copied().map(u16::from).sum::<u16>(),
            next: vec![],
            prev: vec![],
            date: DateTuple::today(),
            dur: Duration::new(01, 00, 00),
            state: SporeState::DoState,
        };
        &self.0.push(spore);
    }

    pub fn len(self) -> usize {
        self.0.len()
    }

}

impl<'a> IntoIterator for Spores<'a> {
    type Item = Spore<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SporeState {
    DoState,
    DoingState,
    DoneState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_spores() {
        let spores = Spores::new();
        assert_eq!( spores, Spores::new())
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

