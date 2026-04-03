use std::collections::VecDeque;

pub struct SlidingWindow {
    pub window: VecDeque<String>,
    pub max_size: usize,
}

impl SlidingWindow {
    pub fn new(size: usize) -> Self {
        Self {
            window: VecDeque::new(),
            max_size: size,
        }
    }

    pub fn add(&mut self, log: String) {
        if self.window.len() >= self.max_size {
            self.window.pop_front();
        }
        self.window.push_back(log);
    }
}
pub fn window_frequency(window: &VecDeque<String>) -> HashMap<String, u32> {
    let mut freq = HashMap::new();

    for line in window {
        *freq.entry(line.clone()).or_insert(0) += 1;
    }

    freq
}

