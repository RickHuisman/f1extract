use rusqlite::{params, Connection};
use crate::{Session, Meeting, Event};

pub(crate) struct Formula1DB {
    conn: Connection,
}

impl Formula1DB {
    pub fn new() -> Self {
        let db = Formula1DB {
            conn: Connection::open("f1.db").unwrap()
        };
        // db.setup_db();
        db
    }

    fn setup_db(&self) {
        // Create necessary tables.
        self.conn.execute(
            "CREATE TABLE meeting (
                meeting_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name VARCHAR(255),
                start NUMERIC,
                end NUMERIC
            );", []).unwrap();

        self.conn.execute(
            "CREATE TABLE session (
                session_id INTEGER PRIMARY KEY AUTOINCREMENT,
                meeting_id INTEGER,
                session VARCHAR(255),
                url VARCHAR(255),
                FOREIGN KEY(meeting_id) REFERENCES meeting(meeting_id)
            );", []).unwrap();

        self.conn.execute(
            "CREATE TABLE event (
                event_id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER,
                timestamp NUMERIC,
                racing_number INTEGER,
                FOREIGN KEY(session_id) REFERENCES sessions(session_id)
            );", []).unwrap();

        self.conn.execute(
            "CREATE TABLE lap (
                lap_id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_id INTEGER,
                laps INTEGER,
                FOREIGN KEY(event_id) REFERENCES event(event_id)
            );", []).unwrap();
    }

    pub fn create_meeting(&self, meeting: &Meeting) -> i64 {
        self.conn.execute(
            "INSERT INTO meeting (name, start, end) VALUES (?1, ?2, ?3)",
            params![meeting.name, meeting.start, meeting.end],
        ).unwrap();

        self.conn.last_insert_rowid()
    }

    pub fn create_session(&self, meeting_id: i64, session: &Session) {
        self.conn.execute(
            "INSERT INTO session (meeting_id, session, url) VALUES (?1, ?2, ?3)",
            params![meeting_id, session.session, session.url],
        ).unwrap();
    }

    pub fn add_event(&self, event: &Event) -> i64 {
        self.conn.execute(
            "INSERT INTO event (session_id, timestamp, racing_number) VALUES (?1, ?2, ?3)",
            params![event.session_id, event.timestamp, event.racing_number],
        ).unwrap();

        self.conn.last_insert_rowid()
    }

    fn add_lap(&self, event_id: i64, laps: i64) {
        self.conn.execute(
            "INSERT INTO lap (event_id, laps) VALUES (?1, ?2)",
            params![event_id, laps],
        ).unwrap();
    }

    pub fn get_sessions(&self) -> Vec<Session> {
        let mut stmt = self.conn.prepare("SELECT session_id, url, session FROM session").unwrap();
        let session_iter = stmt.query_map([], |row| {
            Ok(Session {
                id: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
                session: row.get(2).unwrap(),
            })
        }).unwrap();

        // TODO: Cleanup.
        let mut sessions = vec![];
        for s in session_iter {
            sessions.push(s.unwrap());
        }
        sessions
    }
}

