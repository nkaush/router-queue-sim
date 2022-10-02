extern crate rand;

mod time;
mod packet;
mod simulator;
mod simulation;
mod router_queue;

use std::process::{Command, Output};
use std::fs::{File, OpenOptions};
use std::{fs, env, io, thread};
use std::io::{Read, Write};
use simulator::Simulator;
use std::time::Instant;

const NUM_SUMULATIONS: usize = 10000;
const GRAPH_CODE_PATH: &'static str = "python/graph_code.py";
const SCRIPT_PATH: &'static str = "graph.py";

fn main() -> Result<(), io::Error> {
    let num_simulations = env::args()
        .nth(1)
        .unwrap_or_else(|| NUM_SUMULATIONS.to_string())
        .parse()
        .unwrap();

    let mut xs: Vec<f64> = Vec::new();
    let mut avg_qds: Vec<f64> = Vec::new();
    let mut avg_qd_25pcts: Vec<f64> = Vec::new();
    let mut avg_qd_75pcts: Vec<f64> = Vec::new();
    let mut avg_qs: Vec<f64> = Vec::new();
    let mut handles = Vec::new();

    for arrival_rate in 1..11 {
        xs.push(arrival_rate as f64 / 10.0);
        let handle = thread::spawn(move || {
            let mut s = Simulator::new(arrival_rate, num_simulations);

            let start = Instant::now();
            let (avg_qd, avg_qd_25pct, avg_qd_75pct, mean_qs) = s.run();
            let duration = Instant::now() - start;

            (arrival_rate, avg_qd, avg_qd_25pct, avg_qd_75pct, mean_qs, duration.as_secs_f64())
        });

        handles.push(handle);
    }

    for handle in handles.into_iter() {
        let result = handle.join().unwrap();
        let (arrival_rate, avg_qd, avg_qd_25pct, avg_qd_75pct, mean_qs, duration) = result;
        avg_qs.push(mean_qs);
        avg_qds.push(avg_qd);
        avg_qd_25pcts.push(avg_qd_25pct);
        avg_qd_75pcts.push(avg_qd_75pct);
        println!("Running {} simulations for arrival rate={} took {:0.3}s", num_simulations, arrival_rate, duration);
    }

    let mut results = open_script_file()?;
    writeln!(results, "N={}", num_simulations)?;
    writeln!(results, "xs={:?}", xs)?;
    writeln!(results, "qs={:?}", avg_qs)?;
    writeln!(results, "mean={:?}", avg_qds)?;
    writeln!(results, "p25={:?}", avg_qd_25pcts)?;
    writeln!(results, "p75={:?}", avg_qd_75pcts)?;

    let cmd: Output = run_graphing_script(&mut results)?;
    eprintln!("status: {}", cmd.status);
    eprintln!("stdout: {}", String::from_utf8_lossy(&cmd.stdout));
    fs::remove_file(SCRIPT_PATH).unwrap();

    Ok(())
}

fn open_script_file() -> Result<File, io::Error> {
    fs::remove_file(SCRIPT_PATH).unwrap_or(());
    let results: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(SCRIPT_PATH)?;

    Ok(results)
}

fn run_graphing_script(results_file: &mut File) -> Result<Output, io::Error> {
    let mut graph_code = File::open(GRAPH_CODE_PATH)?;
    let mut graph_code_str = String::new();
    graph_code.read_to_string(&mut graph_code_str)?;
    writeln!(results_file, "{}", graph_code_str)?;

    let out = Command::new("python3")
        .arg(SCRIPT_PATH)
        .output()?;

    Ok(out)
}