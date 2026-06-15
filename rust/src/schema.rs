use diesel::table;

table! {
    public.tickets {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
        priority -> Text,
        assignee -> Nullable<Text>,
        created_at -> Timestamptz
    }
}