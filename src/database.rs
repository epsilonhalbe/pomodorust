use crate::schema;
use crate::schema::statistics;
use diesel::sql_query;
use diesel::insert_into;
use diesel::prelude::*;

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "statistics"]
pub struct Statistic {
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
        );"
        ).execute(conn)
}

impl Statistic {
    pub fn empty() -> Statistic {
        Statistic {
            jira_id: None,
            note: None,
        }
    }

    pub fn insert(&self, conn: &SqliteConnection) -> QueryResult<usize> {
        use schema::statistics::dsl::*;
        insert_into(statistics)
            .values(self)
            .execute(conn)
    }
}
