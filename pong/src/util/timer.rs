use macroquad::prelude as mq;

pub struct Timer {
    initial_seconds: f32,
    seconds_left: f32,
}

impl Timer {
    pub fn new(seconds: f32) -> Self {
        Self {
            initial_seconds: seconds,
            seconds_left: seconds,
        }
    }

    pub fn is_over(&mut self) -> bool {
        if self.seconds_left <= 0.0 {
            self.seconds_left = self.initial_seconds;
            return true;
        }

        self.seconds_left -= mq::get_frame_time();
        false
    }
}
