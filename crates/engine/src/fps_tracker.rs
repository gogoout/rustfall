use std::time;

#[derive(Debug)]
pub struct FpsTracker {
    fps_start_time: time::Instant,
    fps_frames: usize,
    fps: f64,
}

impl Default for FpsTracker {
    fn default() -> Self {
        Self {
            fps_start_time: time::Instant::now(),
            fps_frames: 0,
            fps: 0.0,
        }
    }
}

impl FpsTracker {
    pub fn track_fps(&mut self) {
        self.fps_frames += 1;
        let elapsed = (time::Instant::now() - self.fps_start_time).as_millis();
        if elapsed >= 1000 {
            self.fps = self.fps_frames as f64 / elapsed as f64 * 1000.0;
            self.fps_frames = 0;
            self.fps_start_time = time::Instant::now();
        }
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }
}
