use crate::simulation::SingleSimulation;

pub struct Simulator {
    num_simulations: usize,
    arrival_rate: usize,

}

impl Simulator {
    pub fn new(arrival_rate: usize, num_simulations: usize) -> Self {
        Self {
            num_simulations,
            arrival_rate
        }
    }

    pub fn run(&mut self) -> (f64, f64, f64, f64) {
        let mut avg_qds: Vec<f64> = Vec::new();
        let mut max_qs: Vec<f64> = Vec::new();

        for _ in 0..self.num_simulations {
            let (avg_qd, max_q) = SingleSimulation::new(self.arrival_rate).simulate();
            avg_qds.push(avg_qd);
            max_qs.push(max_q as f64);
        }

        avg_qds.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let avg_qd = avg_qds.iter().sum::<f64>() / avg_qds.len() as f64;
        let avg_qd_25pct = avg_qds.iter().nth(avg_qds.len() / 4).unwrap();
        let avg_qd_75pct = avg_qds.iter().nth(avg_qds.len() * 3 / 4).unwrap();

        let mean_qs = max_qs.iter().sum::<f64>() / max_qs.len() as f64;

        return (avg_qd, *avg_qd_25pct, *avg_qd_75pct, mean_qs);
    }
}