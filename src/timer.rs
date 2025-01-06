use std::time::Duration;
pub struct Timer {

}

impl Timer {
    pub fn run<F>(dur: &Duration, f: F) 
    where F: Fn(Duration)
    {
        let start = std::time::Instant::now();
        while start.elapsed() < *dur {
            let remaining = dur
                .checked_sub(start.elapsed())
                .unwrap_or(Duration::from_secs(0));
            
            f(remaining);
            

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}