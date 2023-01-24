pub trait StrictPartialOrder {
    fn lt(&self, y: &Self) -> bool;
}
