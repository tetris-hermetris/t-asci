use date_time::date_tuple::DateTuple;
use date_time::time_tuple::Duration;

#[derive(Debug, PartialEq)]
pub struct Spores<'a> (Vec<&'a Spore<'a>>);

#[derive(Debug, PartialEq)]
pub struct Spore<'a> {
    pub tag: String,
    pub id: u16,
    pub next: Vec<&'a Spore<'a>>,
    pub prev: Vec<&'a Spore<'a>>,
    pub date: DateTuple,
    pub dur: Duration,
    pub state: SporeState,
}

impl<'a> Spores<'_>  {
    pub fn new() -> Spores<'static> {
        Spores (Vec::new())
        }
    }

#[derive(Debug, PartialEq)]
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
}

