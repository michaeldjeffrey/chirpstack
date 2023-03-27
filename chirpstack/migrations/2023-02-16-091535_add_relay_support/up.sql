alter table device_profile
    add column is_relay boolean not null default false,
    add column ed_relay_only boolean not null default false;

alter table device_profile
    alter column is_relay drop default,
    alter column ed_relay_only drop default;

create table relay_device (
    relay_dev_eui bytea not null references device on delete cascade,
    dev_eui bytea not null references device on delete cascade,
    created_at timestamp with time zone not null,
    primary key (relay_dev_eui, dev_eui)
);

alter table device
    add column join_eui bytea not null default decode('0000000000000000', 'hex');

alter table device
    alter column join_eui drop default;
