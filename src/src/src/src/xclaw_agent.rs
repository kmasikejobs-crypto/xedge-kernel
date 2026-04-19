use crate::{GrokScheduler, WorkloadSnapshot, Prediction};

pub struct XClawAgent {
    scheduler: GrokScheduler,
}

impl XClawAgent {
    pub fn new(scheduler: GrokScheduler) -> Self {
        Self { scheduler }
    }

    pub fn run_agent_task(&mut self, task: &str, snapshot: WorkloadSnapshot) {
        let prediction = self.scheduler.ai_predict(snapshot, &mut heapless::Vec::new());

        if task == "monitor_vibration" && prediction.anomaly_score > 0.8 {
            defmt::info!("🦾 xClaw agent triggered — anomaly detected!");
        }
    }
}