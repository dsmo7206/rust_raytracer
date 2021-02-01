use std::time::Instant;

pub struct Timer<'a> {
    name: &'a str,
    start_time: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        Timer {
            name,
            start_time: Instant::now(),
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed().as_millis();
        println!("{} took {} ms", self.name, duration);
    }
}
