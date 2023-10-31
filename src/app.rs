use std::error;
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
