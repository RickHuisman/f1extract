use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimingDataEvents {
    #[serde(rename = "GapToLeader")]
    pub gap_to_leader: Option<String>,
    #[serde(rename = "IntervalToPositionAhead")]
    pub interval_to_pos_ahead: Option<IntervalToPositionAhead>,
    #[serde(rename = "NumberOfLaps")]
    pub number_of_laps: Option<i64>,
    #[serde(rename = "Sectors")]
    pub sectors: Option<Vec<Sector>>,
    #[serde(rename = "Speeds")]
    pub speeds: Option<HashMap<String, Speed>>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct IntervalToPositionAhead {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Catching")]
    pub catching: Option<bool>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Sector {
    #[serde(rename = "Stopped")]
    pub stopped: bool,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Status")]
    pub status: i64,
    #[serde(rename = "OverallFastest")]
    pub overall_fastest: bool,
    #[serde(rename = "PersonalFastest")]
    pub personal_fastest: bool,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Speed {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Status")]
    pub status: Option<i64>,
    #[serde(rename = "OverallFastest")]
    pub overall_fastest: bool,
    #[serde(rename = "PersonalFastest")]
    pub personal_fastest: bool,
}
