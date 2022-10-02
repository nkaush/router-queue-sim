use rand::{Rng, distributions::Uniform};
use crate::router_queue::RouterQueue;
use std::collections::HashMap;
use crate::packet::Packet;
use crate::time::Time;
use std::cmp::max;

const GRANULARITY_INV: usize = 10;

pub struct SingleSimulation {
    arrival_rate: usize,
    total_time: usize,
    router_queue: RouterQueue,
    packet_arrivals: HashMap<Time, usize>,
}

impl SingleSimulation {
    pub fn new(arrival_rate: usize) -> Self {
        let mut this = Self {
            arrival_rate,
            total_time: 1000,
            router_queue: RouterQueue::new(),
            packet_arrivals: Default::default(),
        };

        this.generate_times();
        this
    }

    fn reset(&mut self) {
        self.router_queue.reset();
        self.generate_times();
    }

    fn generate_times(&mut self) {
        self.packet_arrivals.clear();
        let num_packets = self.arrival_rate * self.total_time;
        let max_time = (self.total_time * GRANULARITY_INV) + 1;

        let rng = rand::thread_rng();
        let range = Uniform::from(0..max_time);

        rng.sample_iter(&range)
            .take(num_packets)
            .map(|t: usize| Time::init(t))
            .for_each(|t: Time| {
                match self.packet_arrivals.get(&t) {
                    Some(c) => self.packet_arrivals.insert(t, c + 1),
                    None => self.packet_arrivals.insert(t, 1)
                };
            });
    }

    pub fn simulate_and_reset(&mut self) -> (f64, usize) {
        let mut max_queue_size = 0;
        let max_time = (self.total_time * GRANULARITY_INV) + 1;

        let mut tick = 0;
        while tick < max_time || !self.router_queue.is_empty() {
            let t = Time::init(tick);
            let count = self.packet_arrivals.get(&t).cloned();
            for _ in 0..count.unwrap_or_default() {
                self.router_queue.push(Packet::new(&t));
            }

            max_queue_size = max(max_queue_size, self.router_queue.len());
            self.router_queue.pop(&t);
            tick += 1;
        }

        let res = (self.router_queue.avg_qd(), max_queue_size);
        self.reset();
        res
    }
}