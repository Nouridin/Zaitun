use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fmt;

pub struct Benchmark {
    name: String,
    iterations: usize,
    setup_fn: Option<Box<dyn Fn()>>,
    bench_fn: Box<dyn Fn()>,
    teardown_fn: Option<Box<dyn Fn()>>,
}

impl Benchmark {
    pub fn new(name: &str, bench_fn: Box<dyn Fn()>) -> Self {
        Benchmark {
            name: name.to_string(),
            iterations: 100,
            setup_fn: None,
            bench_fn,
            teardown_fn: None,
        }
    }
    
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }
    
    pub fn with_setup(mut self, setup_fn: Box<dyn Fn()>) -> Self {
        self.setup_fn = Some(setup_fn);
        self
    }
    
    pub fn with_teardown(mut self, teardown_fn: Box<dyn Fn()>) -> Self {
        self.teardown_fn = Some(teardown_fn);
        self
    }
    
    pub fn run(&self) -> BenchmarkResult {
        let mut durations = Vec::with_capacity(self.iterations);
        
        for _ in 0..self.iterations {
            // Run setup if provided
            if let Some(setup) = &self.setup_fn {
                setup();
            }
            
            // Run benchmark
            let start = Instant::now();
            (self.bench_fn)();
            let duration = start.elapsed();
            durations.push(duration);
            
            // Run teardown if provided
            if let Some(teardown) = &self.teardown_fn {
                teardown();
            }
        }
        
        // Calculate statistics
        durations.sort();
        
        let total: Duration = durations.iter().sum();
        let mean = total / self.iterations as u32;
        
        let median = if self.iterations % 2 == 0 {
            let mid_right = self.iterations / 2;
            let mid_left = mid_right - 1;
            (durations[mid_left] + durations[mid_right]) / 2
        } else {
            durations[self.iterations / 2]
        };
        
        let min = durations[0];
        let max = durations[self.iterations - 1];
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.iterations,
            mean,
            median,
            min,
            max,
        }
    }
}

pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub mean: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Benchmark: {}", self.name)?;
        writeln!(f, "  Iterations: {}", self.iterations)?;
        writeln!(f, "  Mean:       {:?}", self.mean)?;
        writeln!(f, "  Median:     {:?}", self.median)?;
        writeln!(f, "  Min:        {:?}", self.min)?;
        writeln!(f, "  Max:        {:?}", self.max)?;
        Ok(())
    }
}

pub struct BenchmarkSuite {
    benchmarks: HashMap<String, Benchmark>,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        BenchmarkSuite {
            benchmarks: HashMap::new(),
            results: Vec::new(),
        }
    }
    
    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        self.benchmarks.insert(benchmark.name.clone(), benchmark);
    }
    
    pub fn run_all(&mut self) {
        self.results.clear();
        
        for (_, benchmark) in &self.benchmarks {
            let result = benchmark.run();
            self.results.push(result);
        }
        
        // Sort results by mean time (ascending)
        self.results.sort_by(|a, b| a.mean.cmp(&b.mean));
    }
    
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("Benchmark Results:\n");
        report.push_str("=================\n\n");
        
        for result in &self.results {
            report.push_str(&format!("{}\n", result));
        }
        
        report
    }
}