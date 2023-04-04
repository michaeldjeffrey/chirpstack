alter table device
    drop column join_eui;

drop table relay_device;

alter table device_profile
    drop column is_relay,
    drop column relay_ed_relay_only,
    drop column relay_enabled,
    drop column relay_cad_periodicity,
    drop column relay_default_channel_index,
    drop column relay_second_channel_freq,
    drop column relay_second_channel_dr,
    drop column relay_second_channel_ack_offset,
    drop column relay_ed_activation_mode,
    drop column relay_ed_smart_enable_level,
    drop column relay_ed_back_off;
