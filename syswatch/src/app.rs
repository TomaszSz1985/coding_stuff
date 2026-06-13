pub struct App {
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub processes: Vec<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            cpu_usage: 0.0,
            ram_used: 0,
            ram_total: 0,
            processes: vec![],
            should_quit: false,
        }
    }
}
