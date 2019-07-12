table! {
    times (id) {
        id -> Integer,
        project -> Text,
        start -> Timestamp,
        end -> Nullable<Timestamp>,
    }
}
