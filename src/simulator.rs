use crate::simulation::SingleSimulation;

pub struct Simulator {
    num_simulations: usize,
    arrival_rate: usize,
}

fn mean(v: &Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

impl Simulator {
    pub fn new(arrival_rate: usize, num_simulations: usize) -> Self {
        Self {
            num_simulations,
            arrival_rate
        }
    }

    pub fn run(&mut self) -> (f64, f64, f64, f64) {
        let mut simulator = SingleSimulation::new(self.arrival_rate);
        let (mut avg_qds, max_qs): (Vec<f64>, Vec<f64>) = (0..self.num_simulations)
            .map(|_| simulator.simulate_and_reset())
            .map(|(a, b)| (a, b as f64))
            .unzip();

        avg_qds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let avg_qd_25pct = avg_qds.iter().nth(avg_qds.len() / 4).unwrap();
        let avg_qd_75pct = avg_qds.iter().nth(avg_qds.len() * 3 / 4).unwrap();

        return (mean(&avg_qds), *avg_qd_25pct, *avg_qd_75pct, mean(&max_qs));
    }
}