use std::fmt::Display;

use rand::seq::SliceRandom;

use crate::{attr::Attr, derive_into_view};

#[derive(Clone, Copy)]
pub enum Sex {
    Female,
    Male,
}

impl Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Sex::*;

        write!(
            f,
            "{}",
            match self {
                Female => "F",
                Male => "M",
            }
        )
    }
}

derive_into_view!(Sex);

impl Attr for Sex {
    const TITLE: &'static str = "Sex";

    fn set() -> Self {
        use Sex::*;

        *[Female, Male].choose(&mut rand::thread_rng()).unwrap()
    }
}
