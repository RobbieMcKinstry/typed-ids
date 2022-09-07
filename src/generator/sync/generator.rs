pub struct Generator<T: 'static> {
    callback: Box<dyn FnMut() -> T + 'static>,
}

impl<T: 'static> Generator<T> {
    pub fn new<F>(next: F) -> Self
    where
        F: FnMut() -> T + 'static,
    {
        Self {
            callback: Box::new(next),
        }
    }

    pub fn next(&mut self) -> T {
        (self.callback)()
    }
}
