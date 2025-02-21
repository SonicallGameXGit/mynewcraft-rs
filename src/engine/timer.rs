use std::time::Instant as Time;

pub struct Timer {
    time: Time,

    last_time: f32,
    delta: f32,
}

impl Timer {
    pub fn create() -> Self {
        let time = Time::now();

        Self {
            time,

            last_time: time.elapsed().as_secs_f32(),
            delta: 0.0,
        }
    }

    pub fn update(&mut self) {
        let current_time: f32 = self.get_time();
        
        self.delta = current_time - self.last_time;
        self.last_time = current_time;
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    pub fn get_time(&self) -> f32 {
        self.time.elapsed().as_secs_f32()
    }
}