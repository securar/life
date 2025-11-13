use macroquad::prelude::get_time;

pub struct Clock {
    last_update_time: f64,
    pub interval: f64,
}

impl Clock {
    pub fn new(interval: f64) -> Self {
        Clock {
            last_update_time: get_time(),
            interval,
        }
    }

    pub fn can_update(&mut self) -> bool {
        let current_time = get_time();
        if current_time - self.last_update_time > self.interval {
            self.last_update_time = current_time;
            return true;
        }
        return false;
    }
}
