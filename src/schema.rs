use diesel::table;

table! {
    statistics {
        id -> Integer,
        created_at -> Timestamp,
        duration -> BigInt,
        ticket_id -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}
