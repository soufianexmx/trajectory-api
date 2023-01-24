struct Observable<T> {
    value: T
}

impl<T> Observable<T> {

    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}