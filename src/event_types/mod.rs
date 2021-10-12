use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::event_types::timing_data::TimingDataEvents;

pub mod timing_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "Lines")]
    pub lines: Lines,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lines {
    #[serde(flatten)]
    pub racing_numbers: HashMap<String, TimingDataEvents>, // TODO: Make generic.
}
