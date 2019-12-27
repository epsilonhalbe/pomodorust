use diesel::table;

table! {
    statistics {
        id -> Integer,
        created_at -> Timestamp,
        jira_id -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}
