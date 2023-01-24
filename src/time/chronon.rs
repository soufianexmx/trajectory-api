pub struct Chronon<T> {
    time: T,
}

impl<T> Chronon<T> {
    pub fn new(time: T) -> Self {
        Self { time }
    }

    pub fn time(&self) -> &T {
        &self.time
    }
}
