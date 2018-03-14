table! {
    origins (id) {
        id -> Integer,
        name -> Text,
        session_sync -> Bool,
        default_package_visibility -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}