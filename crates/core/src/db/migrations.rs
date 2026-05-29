//! database migrations

/// migration version
pub const SCHEMA_VERSION: u32 = 1;

/// list of all migrations
pub const MIGRATIONS: &[(&str, &str)] = &[
    ("001_initial_schema", include_str!("../../migrations/initial_schema.sql")),
];

