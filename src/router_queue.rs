use std::collections::VecDeque;
use crate::packet::Packet;
use crate::time::Time;

pub struct RouterQueue {
    q: VecDeque<Packet>,
    qd: Vec<f64>
}

impl RouterQueue {
    pub fn new() -> Self {
        Self {
            q: VecDeque::new(),
            qd: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.q.clear();
        self.qd.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn pop(&mut self, t: &Time) -> Option<Packet> {
        match self.q.pop_front() {
            Some(mut p) => {
                self.qd.push(p.send(&t));
                Some(p)
            },
            None => None
        }
    }

    pub fn push(&mut self, packet: Packet) {
        self.q.push_back(packet)
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn avg_qd(&self) -> f64 {
        self.qd.iter().sum::<f64>() / self.qd.len() as f64
    }
}