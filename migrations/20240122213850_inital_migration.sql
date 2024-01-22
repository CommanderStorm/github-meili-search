-- Add migration script here
CREATE TABLE issue_log
(
    id             bigint primary key not null unique,
    hash           bigint             not null unique,
    last_update_at datetime           not null default (DATETIME('now', 'utc'))
)