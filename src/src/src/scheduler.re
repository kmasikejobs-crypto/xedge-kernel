use crate::{tiny_infer, WorkloadSnapshot, Prediction};

pub struct GrokScheduler {
    model: tiny_infer::GrokEdgeModel,
}

impl GrokScheduler {
    pub fn new(model: tiny_infer::GrokEdgeModel) -> Self {
        Self { model }
    }

    pub fn ai_predict(&self, snapshot: WorkloadSnapshot, history: &mut heapless::Vec<WorkloadSnapshot, 32>) -> Prediction {
        if history.len() == 32 {
            history.remove(0);
        }
        let _ = history.push(snapshot);

        let input = [snapshot.temp, snapshot.vib, snapshot.battery, snapshot.time_of_day];
        let logits = self.model.infer(&input);

        Prediction {
            tasks: [logits[0] as u8, 1, 1, 1],
            anomaly_score: logits[1].clamp(0.0, 1.0),
            sleep_ms: logits[2] as u32,
        }
    }
}

pub struct Prediction {
    pub tasks: [u8; 4],
    pub anomaly_score: f32,
    pub sleep_ms: u32,
}