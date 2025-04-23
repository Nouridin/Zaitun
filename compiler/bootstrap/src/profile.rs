use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Profiler {
    start_time: Instant,
    section_times: HashMap<String, Duration>,
    current_section: Option<(String, Instant)>,
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            start_time: Instant::now(),
            section_times: HashMap::new(),
            current_section: None,
        }
    }
    
    pub fn start_section(&mut self, name: &str) {
        if self.current_section.is_some() {
            self.end_section();
        }
        
        self.current_section = Some((name.to_string(), Instant::now()));
    }
    
    pub fn end_section(&mut self) {
        if let Some((name, start)) = self.current_section.take() {
            let duration = start.elapsed();
            *self.section_times.entry(name).or_insert(Duration::new(0, 0)) += duration;
        }
    }
    
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("Profiling Report:\n");
        
        let total = self.start_time.elapsed();
        report.push_str(&format!("Total time: {:?}\n", total));
        
        for (name, duration) in &self.section_times {
            let percentage = duration.as_secs_f64() / total.as_secs_f64() * 100.0;
            report.push_str(&format!("{}: {:?} ({:.2}%)\n", name, duration, percentage));
        }
        
        report
    }
}