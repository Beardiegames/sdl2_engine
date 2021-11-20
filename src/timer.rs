
use std::time::SystemTime;

pub struct UpdateTimer {
    play_time: SystemTime,
    target_time: u32,
    cycle_time: u32,
    delay_time: u32,

    pub delta_time: f32,
    pub frame_duration: u32,
    sample_delay:u32,
    num_samples:u32,
}

impl Default for UpdateTimer {
    fn default() -> Self {
        UpdateTimer {
            play_time: SystemTime::now(),
            target_time: 1000,
            cycle_time: 0,
            delay_time: 0,
            delta_time: 0.0,
            frame_duration: 0,
            sample_delay:0,
            num_samples:0,
        }
        
    }
}

impl UpdateTimer {
    pub fn new(target_fps: u64) -> Self {
        UpdateTimer {
            play_time: SystemTime::now(),
            target_time: 1000 / target_fps as u32,
            cycle_time: 0,
            delay_time: 0,
            delta_time: 0.0,
            frame_duration: 0,
            sample_delay: 0,
            num_samples: 0,
        }
    }

    pub fn sync(&mut self) {
        self.cycle_time = self.play_time.elapsed().unwrap().as_millis() as u32;
            
        if self.cycle_time > self.target_time { 
            self.frame_duration = self.cycle_time;
            self.delay_time = 0;
        } else {
            self.frame_duration = self.target_time;
            self.delay_time = self.target_time - self.cycle_time;
        }
        self.delta_time = self.frame_duration as f32 * 0.001; 
        //println!("fps: {}", self.delta_time);

        std::thread::sleep(std::time::Duration::from_millis((self.delay_time) as u64));
        self.play_time = SystemTime::now();

        self.sample_delay += self.frame_duration;
        self.num_samples += 1;
        if self.sample_delay > 3000 {
            println!("average fps: {}", self.sample_delay / self.num_samples);
            self.sample_delay = 0;
            self.num_samples = 0;
        }
    }
}