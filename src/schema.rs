table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_encrypted -> Text,
        created_at -> Timestamptz,
    }
}
