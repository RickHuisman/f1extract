mod db;
mod event_types;

use serde::Deserialize;
use std::fs;
use crate::db::Formula1DB;
use serde_json::Value;
use crate::event_types::Root;

#[derive(Debug)]
struct Event {
    session_id: i64,
    timestamp: String,
    racing_number: u32,
}

#[derive(Deserialize, Debug)]
struct Session {
    id: Option<i64>,
    url: String,
    session: String,
}

#[derive(Deserialize, Debug)]
struct Meeting {
    name: String,
    start: String,
    end: String,
    sessions: Vec<Session>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Formula1DB::new();
    // add_meetings(&db);

    let sessions = db.get_sessions();
    extract_events(&db, &sessions[0]);
    Ok(())
}

fn add_meetings(db: &Formula1DB) {
    let data = fs::read_to_string("sessions.json").unwrap();
    let meetings: Vec<Meeting> = serde_json::from_str(&data).unwrap();

    for meeting in meetings {
        let meeting_id = db.create_meeting(&meeting);

        for session in meeting.sessions {
            db.create_session(meeting_id, &session);
        }
    }
}

fn extract_events(db: &Formula1DB, session: &Session) {
    let split = fs::read_to_string("test.jsonStream").unwrap();
    let lines: Vec<&str> = split.split('\n').collect();
    // TODO: Check for empty lines.

    for line in lines {
        let timestamp = &line[0..12];
        let line_data = &line[12..];

        let root: Root = serde_json::from_str(line_data).unwrap();

        for (r_number, events) in root.lines.racing_numbers {
            let event = Event {
                session_id: session.id.unwrap(), // TODO: Unwrap.
                timestamp: timestamp.to_string(),
                racing_number: r_number.parse::<u32>().unwrap(), // TODO: Unwrap.
            };
            db.add_event(&event);
        }
    }
}

async fn extract_lap(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    // let url = "https://livetiming.formula1.com/static/2020/2020-07-19_Hungarian_Grand_Prix/2020-07-19_Race/TimingData.jsonStream";
    // let resp = reqwest::get(url)
    //     .await?
    //     .text()
    //     .await?;
    // println!("{:#?}", resp);

    for line in lines {
        let timestamp = &line[0..12];
        let line_data = &line[12..];

        // for (r_number, events) in v.lines.racing_numbers {
        //     if let Some(lap) = events.number_of_laps {
        //         println!("{:?}", lap);
        //     }
        // }
    }

    Ok(())
}
