use rand::seq::SliceRandom;

use crate::{derive_display, Attr};

#[derive(Clone, Debug)]
pub struct Last(String);

derive_display!(Last);

impl Attr for Last {
    const TITLE: &'static str = "Last Name";

    fn set() -> Self {
        const NAMES: &[&str] = &["Smith", "Brown", "Person", "Bloggs"];

        Self(NAMES.choose(&mut rand::thread_rng()).unwrap().to_string())
    }
}
