extern crate rand;

mod time;
mod packet;
mod simulator;
mod simulation;
mod router_queue;

use std::os::unix::prelude::PermissionsExt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;
use simulator::Simulator;
use std::time::Instant;

const NUM_SUMULATIONS: usize = 2000;

fn main() -> Result<(), std::io::Error> {
    let num_simulations = std::env::args()
        .nth(1)
        .unwrap_or_else(|| NUM_SUMULATIONS.to_string())
        .parse()
        .unwrap();

    let mut xs: Vec<f64> = Vec::new();
    let mut avg_qds: Vec<f64> = Vec::new();
    let mut avg_qd_25pcts: Vec<f64> = Vec::new();
    let mut avg_qd_75pcts: Vec<f64> = Vec::new();
    let mut avg_qs: Vec<f64> = Vec::new();

    for arrival_rate in 1..11 {
        print!("Running {} simulations for arrival rate={} ...", num_simulations, arrival_rate);
        std::io::stdout().flush()?;
        let start = Instant::now();
        let mut s = Simulator::new(arrival_rate, num_simulations);

        let (avg_qd, avg_qd_25pct, avg_qd_75pct, mean_qs) = s.run();
        avg_qds.push(avg_qd);
        avg_qd_25pcts.push(avg_qd_25pct);
        avg_qd_75pcts.push(avg_qd_75pct);
        avg_qs.push(mean_qs);
        xs.push(arrival_rate as f64 / 10.0);

        let duration = Instant::now() - start;
        println!(" {}s", duration.as_secs_f64());
    }

    std::fs::remove_file("graph.py").unwrap_or(());
    let mut results: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("graph.py")?;
    let metadata = results.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);

    writeln!(results, "xs={:?}", xs)?;
    writeln!(results, "qs={:?}", avg_qs)?;
    writeln!(results, "mean={:?}", avg_qds)?;
    writeln!(results, "p25={:?}", avg_qd_25pcts)?;
    writeln!(results, "p75={:?}", avg_qd_75pcts)?;

    let mut graph_code = File::open("python/graph_code.py")?;
    let mut graph_code_str = String::new();
    graph_code.read_to_string(&mut graph_code_str)?;
    writeln!(results, "{}", graph_code_str)?;

    let cmd = Command::new("python3")
        .arg("graph.py")
        .output()?;

    eprintln!("status: {}", cmd.status);
    eprintln!("stdout: {}", String::from_utf8_lossy(&cmd.stdout));
    // std::fs::remove_file("graph.py").unwrap();

    Ok(())
}
