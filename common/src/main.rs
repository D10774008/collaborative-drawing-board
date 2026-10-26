use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum DrawMessage {
    DrawLine {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: String,
        width: f32,
    },
    Clear,
}
