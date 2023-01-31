use rand::Rng;

use crate::{Attr, derive_display};

#[derive(Clone, Debug)]
pub struct Age(u32);

derive_display!(Age);

impl Attr for Age {
    const TITLE: &'static str = "Age";

    fn random() -> Self {
        Self(rand::thread_rng().gen_range(18..40))
    }
}
