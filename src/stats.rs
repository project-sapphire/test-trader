use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub enum Timeframe {
    Minute,
    Hour,
    Day,
    Week
}

pub struct RateData {
    data: VecDeque<f64>,
    span: Duration, // how much data to store back in time

}

pub fn moving_average(period: u32, frame: Timeframe, rd: RateData) -> f64 {
    1.0
}

pub fn e_moving_average(period: u32, frame: Timeframe, rd: RateData) -> f64 {
    1.0
}
