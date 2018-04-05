table! {
    origin_channel_packages (channel_id, package_id) {
        channel_id -> BigInt,
        package_id -> BigInt,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_channels (id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_integrations (id) {
        id -> BigInt,
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
        id -> BigInt,
        origin_id -> Nullable<BigInt>,
        origin_name -> Nullable<Text>,
        account_id -> Nullable<BigInt>,
        account_name -> Nullable<Text>,
        owner_id -> Nullable<BigInt>,
        ignored -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_members (origin_id, account_id) {
        origin_id -> BigInt,
        origin_name -> Nullable<Text>,
        account_id -> BigInt,
        account_name -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_packages (id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        ident -> Text,
        checksum -> Text,
        manifest -> Text,
        config -> Text,
        target -> Text,
        deps -> Text,
        tdeps -> Text,
        exposes -> Text,
        visibility -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_project_integrations (id) {
        id -> BigInt,
        origin -> Text,
        name -> Text,
        integration -> Text,
        integration_name -> Text,
        body -> Text,
        project_id -> BigInt,
        integration_id -> BigInt,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_projects (id) {
        id -> BigInt,
        origin_id -> Nullable<BigInt>,
        origin_name -> Nullable<Text>,
        package_name -> Nullable<Text>,
        name -> Nullable<Text>,
        plan_path -> Nullable<Text>,
        owner_id -> Nullable<BigInt>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        visibility -> Text,
    }
}

table! {
    origin_public_keys (id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        revision -> Text,
        full_name -> Text,
        body -> Bytea,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_secret_keys (id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        revision -> Text,
        full_name -> Text,
        body -> Bytea,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origins (id) {
        id -> BigInt,
        name -> Text,
        owner_id -> BigInt,
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
