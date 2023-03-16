drop table relay_device;

alter table device_profile
    drop column is_relay,
    drop column ed_relay_only;
