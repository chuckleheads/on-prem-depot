table! {
    origin_channel_packages (channel_id, package_id) {
        channel_id -> Int8,
        package_id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_channels (id) {
        id -> Int8,
        origin_id -> Nullable<Int8>,
        owner_id -> Nullable<Int8>,
        name -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_integrations (id) {
        id -> Int8,
        origin -> Nullable<Text>,
        integration -> Nullable<Text>,
        name -> Nullable<Text>,
        body -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_invitations (id) {
        id -> Int8,
        origin_id -> Nullable<Int8>,
        origin_name -> Nullable<Text>,
        account_id -> Nullable<Int8>,
        account_name -> Nullable<Text>,
        owner_id -> Nullable<Int8>,
        ignored -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_members (origin_id, account_id) {
        origin_id -> Int8,
        origin_name -> Nullable<Text>,
        account_id -> Int8,
        account_name -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_packages (id) {
        id -> Int8,
        origin_id -> Nullable<Int8>,
        owner_id -> Nullable<Int8>,
        name -> Nullable<Text>,
        ident -> Nullable<Text>,
        checksum -> Nullable<Text>,
        manifest -> Nullable<Text>,
        config -> Nullable<Text>,
        target -> Nullable<Text>,
        deps -> Nullable<Text>,
        tdeps -> Nullable<Text>,
        exposes -> Nullable<Text>,
        visibility -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_project_integrations (id) {
        id -> Int8,
        origin -> Text,
        name -> Text,
        integration -> Text,
        integration_name -> Text,
        body -> Text,
        project_id -> Int8,
        integration_id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_projects (id) {
        id -> Int8,
        origin_id -> Nullable<Int8>,
        origin_name -> Nullable<Text>,
        package_name -> Nullable<Text>,
        name -> Nullable<Text>,
        plan_path -> Nullable<Text>,
        owner_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        visibility -> Text,
    }
}

table! {
    origin_public_keys (id) {
        id -> Int8,
        origin_id -> Nullable<Int8>,
        owner_id -> Nullable<Int8>,
        name -> Nullable<Text>,
        revision -> Nullable<Text>,
        full_name -> Nullable<Text>,
        body -> Nullable<Bytea>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_secret_keys (id) {
        id -> Int8,
        origin_id -> Nullable<Int8>,
        owner_id -> Nullable<Int8>,
        name -> Nullable<Text>,
        revision -> Nullable<Text>,
        full_name -> Nullable<Text>,
        body -> Nullable<Bytea>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origins (id) {
        id -> Int8,
        name -> Text,
        owner_id -> Int8,
        default_package_visibility -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

joinable!(origin_channel_packages -> origin_channels (channel_id));
joinable!(origin_channel_packages -> origin_packages (package_id));
joinable!(origin_channels -> origins (origin_id));
joinable!(origin_invitations -> origins (origin_id));
joinable!(origin_members -> origins (origin_id));
joinable!(origin_packages -> origins (origin_id));
joinable!(origin_project_integrations -> origin_integrations (integration_id));
joinable!(origin_project_integrations -> origin_projects (project_id));
joinable!(origin_projects -> origins (origin_id));
joinable!(origin_public_keys -> origins (origin_id));
joinable!(origin_secret_keys -> origins (origin_id));

allow_tables_to_appear_in_same_query!(
    origin_channel_packages,
    origin_channels,
    origin_integrations,
    origin_invitations,
    origin_members,
    origin_packages,
    origin_project_integrations,
    origin_projects,
    origin_public_keys,
    origin_secret_keys,
    origins,
);
