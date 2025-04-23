use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct TestRunner {
    tests: HashMap<String, TestCase>,
    results: Vec<TestResult>,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            tests: HashMap::new(),
            results: Vec::new(),
        }
    }
    
    pub fn register_test(&mut self, name: &str, test_fn: Box<dyn Fn() -> Result<(), String>>) {
        self.tests.insert(name.to_string(), TestCase {
            name: name.to_string(),
            test_fn,
        });
    }
    
    pub fn run_all(&mut self) -> TestSummary {
        let start_time = Instant::now();
        self.results.clear();
        
        for (name, test) in &self.tests {
            let test_start = Instant::now();
            let result = (test.test_fn)();
            let duration = test_start.elapsed();
            
            let status = match result {
                Ok(_) => TestStatus::Passed,
                Err(message) => TestStatus::Failed(message),
            };
            
            self.results.push(TestResult {
                name: name.clone(),
                status,
                duration,
            });
        }
        
        let total_duration = start_time.elapsed();
        let passed = self.results.iter().filter(|r| r.status.is_passed()).count();
        let failed = self.results.len() - passed;
        
        TestSummary {
            total: self.results.len(),
            passed,
            failed,
            duration: total_duration,
        }
    }
    
    pub fn report(&self) -> String {
        let mut report = String::new();
        
        for result in &self.results {
            match &result.status {
                TestStatus::Passed => {
                    report.push_str(&format!("✓ {} ({:?})\n", result.name, result.duration));
                }
                TestStatus::Failed(message) => {
                    report.push_str(&format!("✗ {} ({:?})\n  Error: {}\n", result.name, result.duration, message));
                }
            }
        }
        
        report
    }
}

struct TestCase {
    name: String,
    test_fn: Box<dyn Fn() -> Result<(), String>>,
}

struct TestResult {
    name: String,
    status: TestStatus,
    duration: Duration,
}

enum TestStatus {
    Passed,
    Failed(String),
}

impl TestStatus {
    fn is_passed(&self) -> bool {
        match self {
            TestStatus::Passed => true,
            TestStatus::Failed(_) => false,
        }
    }
}

struct TestSummary {
    total: usize,
    passed: usize,
    failed: usize,
    duration: Duration,
}