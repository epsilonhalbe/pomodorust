use crate::schema;
use crate::schema::statistics;
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "statistics"]
pub struct Statistic {
    pub jira_id: Option<String>,
    pub note: Option<String>,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct Pomodoro {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub jira_id: Option<String>,
    pub note: Option<String>,
}

pub fn create_table(conn: &SqliteConnection) -> QueryResult<usize> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS statistics
        ( id INTEGER PRIMARY KEY AUTOINCREMENT
        , created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        , jira_id TEXT NULL
        , note TEXT NULL
        );",
    )
    .execute(conn)
}

impl Statistic {
    pub fn empty() -> Statistic {
        Statistic {
            jira_id: None,
            note: None,
        }
    }

    pub fn set_jira(
        conn: &SqliteConnection,
        pom_id: i32,
        jira_text: Option<String>,
    ) -> QueryResult<usize> {
        use schema::statistics::dsl::*;
        update(statistics.find(pom_id))
            .set(jira_id.eq(jira_text))
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
