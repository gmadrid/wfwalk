type Insanity = String;

trait SanityCheck {
    fn sanity_check(&self) -> Vec<Insanity>;
}

impl SanityCheck for super::Stock {
    fn sanity_check(&self) -> Vec<Insanity> {
        unimplemented!()
    }
}
