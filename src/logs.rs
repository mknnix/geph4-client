use std::collections::VecDeque;
use std::time::{Instant, Duration};

#[derive(Clone)]
pub struct LogRotate {
    buf: LogBuffer,
    timeout: Duration,
    started: Instant,
}
impl LogRotate {
    pub fn new(mem: usize, maxtime: Duration) -> Self {
        Self{
            buf: LogBuffer::new(mem),
            timeout: Instant::now(),
            started: run
        }
    }

    pub fn add_line(&mut self, line: &str) -> bool {
        let res: bool;

        if self.started.elapsed(Instant::now()) > self.timeout {
            self.buf.clear();
            println!("[Geph4-cli] removed all logs from memory (due to timeout {:?})");

            res = true;
        } else {
            res = false;
        }

        self.buf.add_line(line);
        return res;
    }

    pub fn logs(&self) -> LogBuffer {
        let mut snap = self.clone();
        snap.add_line(
            &format!(
                "Geph logs dumped by time {:?}",
                std::time::SystemTime::now()
            )
        );

        snap.buf
    }

    pub fn get_logs(&self) -> String {
        self.logs().get_logs()
    }
}

#[derive(Clone)]
pub struct LogBuffer {
    logs: VecDeque<char>,
    mem_limit: usize, // in # of characters
}

impl LogBuffer {
    pub fn new(mem_limit: usize) -> Self {
        Self {
            logs: VecDeque::new(),
            mem_limit,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        for c in line.chars() {
            self.logs.push_back(c);
        }
        self.logs.push_back('\n');

        while self.logs.len() > self.mem_limit {
            self.logs.pop_front();
        }
    }

    pub fn clear(&mut self) -> bool {
        if self.logs.len() <= 0 {
            return false;
        }

        self.logs = VecDeque::new();
        return true;
    }

    pub fn get_logs(&self) -> String {
        self.logs.clone().into_iter().collect()
    }
}
