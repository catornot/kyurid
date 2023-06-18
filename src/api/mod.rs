use rrplug::prelude::PluginData;

use self::{
    api_utils::{info_get_all_sounds_in_folder, info_play_sound, info_play_sound_tracked},
    sink_api::{
        info_add_sound, info_create_sink, info_destroy_sink, info_get_pause, info_get_speed,
        info_get_volume, info_set_pause, info_set_speed, info_set_volume, info_skip_sound,
    },
};

pub mod api_utils;
pub mod sink_api;
pub mod sound_utils;

pub fn register_api_sq_functions(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(info_play_sound);
    plugin_data.register_sq_functions(info_play_sound_tracked);
    plugin_data.register_sq_functions(info_get_all_sounds_in_folder);
    plugin_data.register_sq_functions(info_create_sink);
    plugin_data.register_sq_functions(info_add_sound);
    plugin_data.register_sq_functions(info_skip_sound);
    plugin_data.register_sq_functions(info_set_speed);
    plugin_data.register_sq_functions(info_destroy_sink);
    plugin_data.register_sq_functions(info_set_volume);
    plugin_data.register_sq_functions(info_get_volume);
    plugin_data.register_sq_functions(info_get_speed);
    plugin_data.register_sq_functions(info_get_pause);
    plugin_data.register_sq_functions(info_set_pause);
}
