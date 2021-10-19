
use std::time::SystemTime;

pub struct UpdateTimer {
    play_time: SystemTime,
    target_time: u64,
    cycle_time: u64,
    delay_time: u64,

    pub delta_time: u64,
    sample_delay:u64,
    num_samples:u64,
}

impl UpdateTimer {
    pub fn new(target_fps: u64) -> Self {
        UpdateTimer {
            play_time: SystemTime::now(),
            target_time: 1_000 / target_fps,
            cycle_time: 0,
            delay_time: 0,
            delta_time: 0,
            sample_delay: 0,
            num_samples: 0,
        }
    }

    pub fn sync(&mut self) {
        self.cycle_time = self.play_time.elapsed().unwrap().as_millis() as u64;
            
        if self.cycle_time > self.target_time { 
            self.delta_time = self.cycle_time;
            self.delay_time = 0;
        } else {
            self.delta_time = self.target_time;
            self.delay_time = self.target_time - self.cycle_time;
        }

        std::thread::sleep(std::time::Duration::from_millis(self.delay_time));
        self.play_time = SystemTime::now();

        self.sample_delay += self.delta_time;
        self.num_samples += 1;
        if self.sample_delay > 3_000 {
            println!("average fps: {}", (1_000 * self.num_samples) / self.sample_delay);
            self.sample_delay = 0;
            self.num_samples = 0;
        }
    }
}