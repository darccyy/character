use rand::seq::SliceRandom;

use crate::{derive_display, Attr};

#[derive(Clone)]
pub struct First(String);

derive_display!(First);

impl Attr for First {
    const TITLE: &'static str = "First Name";

    fn set() -> Self {
        const NAMES: &[&str] = &["John", "Michael", "Steven", "Sophia", "Adam"];

        Self(NAMES.choose(&mut rand::thread_rng()).unwrap().to_string())
    }
}
