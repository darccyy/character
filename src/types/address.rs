use std::fmt::Display;

use rand::{seq::SliceRandom, Rng};

use crate::{attr::Attr, derive_into_view};

#[derive(Debug, Clone, Copy)]
pub enum HomeState {
    ACT,
    NSW,
    NT,
    QLD,
    SA,
    TAS,
    VIC,
    WA,
}

#[derive(Clone)]
pub struct HomeAddress {
    pub house: u32,
    pub street: (&'static str, &'static str),
    pub suburb: &'static str,
    pub state: HomeState,
}

impl Display for HomeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let HomeAddress {
            house,
            street: (name, kind),
            suburb,
            state,
        } = self;

        write!(f, "{house} {name} {kind}, {suburb}, {state:?}, Australia",)
    }
}

derive_into_view!(HomeAddress);

impl Attr for HomeAddress {
    const TITLE: &'static str = "Address";

    fn set() -> Self {
        use HomeState::*;

        const SUBURBS: &[&str] = &["Townland", "Villagetown", "Townsville", "Citytown"];
        const STREET_NAMES: &[&str] = &["Town", "Lane", "Main", "Bell"];
        const STREET_TYPES: &[&str] = &["St", "Rd", "Ln", "Ave"];

        let mut rng = rand::thread_rng();

        let state = *[ACT, NSW, NT, QLD, SA, TAS, VIC, WA]
            .choose(&mut rng)
            .unwrap();

        Self {
            house: rand::thread_rng().gen_range(1..100),
            street: (
                STREET_NAMES.choose(&mut rng).unwrap(),
                STREET_TYPES.choose(&mut rng).unwrap(),
            ),
            suburb: SUBURBS.choose(&mut rng).unwrap(),
            state,
        }
    }
}
