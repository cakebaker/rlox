pub struct Reporter {
    errors: Vec<String>,
}

impl Reporter {
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub const fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn report_error(&mut self, error: String) {
        self.errors.push(error);
    }
}
