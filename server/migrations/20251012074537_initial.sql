-- Add migration script here
create table if not exists timeseries (
    id serial primary key,
    collector_id varchar(255),
    received timestamp,
    total_memory unsigned big int,
    used_memory unsigned big int,
    average_cpu float
)