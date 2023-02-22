alter table device_profile
    add column is_relay boolean not null default false;

alter table device_profile
    alter column is_relay drop default;

create table relay_device (
    relay_dev_eui bytea not null references device on delete cascade,
    dev_eui bytea not null references device on delete cascade,
    created_at timestamp with time zone not null,
    primary key (relay_dev_eui, dev_eui)
);
