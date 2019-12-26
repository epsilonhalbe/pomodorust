#![allow(unused_imports)]
use diesel::{table};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::debug_query;
use diesel::sqlite::Sqlite;
use std::error::Error;

table! {
    statistics {
        id -> u32,
        created_at -> Timestamp,
        jira_id -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}
