use crate::router_queue::RouterQueue;
use std::collections::HashMap;
use crate::packet::Packet;
use crate::time::Time;
use rand::Rng;

const GRANULARITY_INV: usize = 10;

pub struct SingleSimulation {
    arrival_rate: usize,
    total_time: usize,
    router_queue: RouterQueue,
    packet_arrivals: HashMap<Time, usize>
}

impl SingleSimulation {
    pub fn new(arrival_rate: usize) -> Self {
        let mut this = Self {
            arrival_rate,
            total_time: 1000,
            router_queue: RouterQueue::new(),
            packet_arrivals: Default::default()
        };

        this.generate_times();
        this
    }

    fn generate_times(&mut self) {
        let num_packets = self.arrival_rate * self.total_time;
        let max_time = (self.total_time * GRANULARITY_INV) + 1;
        let mut rng = rand::thread_rng();
        let ts: Vec<Time> = (0..num_packets)
            .map(|_| Time::init(rng.gen_range(0..max_time)))
            .collect();

        for t in ts.into_iter() {
            match self.packet_arrivals.get(&t) {
                Some(c) => self.packet_arrivals.insert(t, c + 1),
                None => self.packet_arrivals.insert(t, 1)
            };
        }
    }

    pub fn simulate(mut self) -> (f64, usize) {
        let mut _num_packets_pulled = 0;
        let mut num_packets_pushed = 0;
        let mut max_queue_size = 0;
        let max_time = (self.total_time * GRANULARITY_INV) + 1;

        for t in (0..max_time).map(|t| Time::init(t)) {
            let count = self.packet_arrivals.get(&t).cloned();
            for _ in 0..count.unwrap_or_default() {
                num_packets_pushed += 1;
                self.router_queue.push(Packet::new(&t));
            }

            max_queue_size = std::cmp::max(max_queue_size, self.router_queue.len());

            if self.router_queue.pop(&t).is_some() {
                _num_packets_pulled += 1;
            }
        }

        assert_eq!(num_packets_pushed, self.arrival_rate * self.total_time);
        (self.router_queue.avg_qd(), max_queue_size)
    }
}