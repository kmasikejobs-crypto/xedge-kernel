#![no_std]
#![no_main]

use defmt::*;
use heapless::Vec;

mod tiny_infer;
mod scheduler;
mod xclaw_agent;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("xEdge v0.3 — OpenClaw-ready AI kernel online");

    let mut ai = tiny_infer::GrokEdgeModel::new();
    let mut sched = scheduler::GrokScheduler::new(ai);
    let mut agent = xclaw_agent::XClawAgent::new(sched);

    let mut history: Vec<WorkloadSnapshot, 32> = Vec::new();

    loop {
        let snap = read_sensors();
        agent.run_agent_task("monitor_vibration", snap);
    }
}

#[derive(Clone, Copy, Default)]
pub struct WorkloadSnapshot {
    pub temp: f32,
    pub vib: f32,
    pub battery: f32,
    pub time_of_day: f32,
}

fn read_sensors() -> WorkloadSnapshot {
    WorkloadSnapshot { temp: 45.0, vib: 65.0, battery: 80.0, time_of_day: 12.0 }
}