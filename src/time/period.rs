use super::order::StrictPartialOrder;

pub struct Period<T> {
    inf: T,
    sup: T,
}

impl<T: StrictPartialOrder> Period<T> {
    pub fn new(inf: T, sup: T) -> Option<Period<T>> {
        if inf.lt(&sup) {
            Some(Self { inf, sup })
        } else {
            None
        }
    }

    pub fn inf(&self) -> &T {
        &self.inf
    }

    pub fn sup(&self) -> &T {
        &self.sup
    }
}
