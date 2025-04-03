pub struct GameFeatures {
    pub aimbot_enabled: bool,
    pub speed_hack_enabled: bool,
    pub infinite_health_enabled: bool,
}

impl GameFeatures {
    pub fn new() -> Self {
        GameFeatures {
            aimbot_enabled: false,
            speed_hack_enabled: false,
            infinite_health_enabled: false,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    pub fn toggle_speed_hack(&mut self) {
        self.speed_hack_enabled = !self.speed_hack_enabled;
    }

    pub fn toggle_infinite_health(&mut self) {
        self.infinite_health_enabled = !self.infinite_health_enabled;
    }
}