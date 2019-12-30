use crate::schema;
use crate::schema::statistics;
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;
// use tui::widgets::{Row};

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "statistics"]
pub struct Statistic {
    pub duration: i64,
    pub ticket_id: Option<String>,
    pub note: Option<String>,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Pomodoro {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub duration: i64,
    pub ticket_id: Option<String>,
    pub note: Option<String>,
}

impl Pomodoro {
    pub fn as_row(self) -> [String; 5] {
        [
            format!("{}", self.id),
            format!("{}", self.created_at),
            format!("{}", self.duration),
            self.ticket_id.unwrap_or(String::new()),
            self.note.unwrap_or(String::new()),
        ]
    }
}

pub const HEADER: [&'static str; 5] = ["ID", "Created At", "Duration", "Ticket", "Note"];

pub fn create_table(conn: &SqliteConnection) -> QueryResult<usize> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS statistics
        ( id INTEGER PRIMARY KEY AUTOINCREMENT
        , created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        , duration INTEGER NOT NULL
        , ticket_id TEXT NULL
        , note TEXT NULL
        );",
    )
    .execute(conn)
}

impl Statistic {
    pub fn new(duration: i64) -> Statistic {
        Statistic {
            duration: duration,
            ticket_id: None,
            note: None,
        }
    }

    pub fn set_ticket(
        conn: &SqliteConnection,
        pom_id: i32,
        ticket_text: Option<String>,
    ) -> QueryResult<usize> {
        use schema::statistics::dsl::*;
        update(statistics.find(pom_id))
            .set(ticket_id.eq(ticket_text))
            .execute(conn)
    }

    pub fn set_note(
        conn: &SqliteConnection,
        pom_id: i32,
        note_text: Option<String>,
    ) -> QueryResult<usize> {
        use schema::statistics::dsl::*;
        update(statistics.find(pom_id))
            .set(note.eq(note_text))
            .execute(conn)
    }

    pub fn pomodoros_of(conn: &SqliteConnection, day: NaiveDateTime) -> QueryResult<Vec<Pomodoro>> {
        use schema::statistics::dsl::*;
        let sqltext = format!("date(created_at) = date('{}')", day);
        statistics.filter(sql(&sqltext)).load::<Pomodoro>(conn)
    }

    pub fn todays_no_pomodoros(conn: &SqliteConnection) -> QueryResult<i64> {
        use schema::statistics::dsl::*;
        statistics
            .select(count_star())
            .filter(sql("date(created_at) = date('now', 'start of day')"))
            .first::<i64>(conn)
    }

    pub fn insert(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use schema::statistics::dsl::*;
        insert_into(statistics).values(self).execute(conn)
    }
}
