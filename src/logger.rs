pub struct Logger {
    on: bool,
}

impl Logger {
    pub fn new(set_on: bool) -> Self {
        Self { on: set_on }
    }

    pub fn print<S: AsRef<str>>(&self, data: S) {
        if self.on {
            println!("{:?}", data.as_ref());
        }
    }
}
