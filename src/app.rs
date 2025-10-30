use crate::profile::Profile;

pub struct App {
    pub should_quit: bool,
    pub profiles: Vec<Profile>,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            profiles: vec![
                Profile::new(
                    "Tokio".to_string(),
                    "vless://uuid@us.example.com:443".to_string(),
                ),
                Profile::new(
                    "Moskow".to_string(),
                    "vless://uuid@us.example.com:443".to_string(),
                ),
            ],
            selected_index: 0,
        }
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
