use rrplug::prelude::PluginData;

use self::{
    api_utils::{info_play_sound,info_play_sound_tracked},
    sink_api::{info_create_sink, info_set_speed, info_skip_sound, info_add_sound,info_destroy_sink},
};

pub mod api_utils;
pub mod sink_api;
pub mod sound_utils;

pub fn register_api_sq_functions(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(info_play_sound).unwrap();
    plugin_data.register_sq_functions(info_play_sound_tracked).unwrap();
    plugin_data.register_sq_functions(info_create_sink).unwrap();
    plugin_data.register_sq_functions(info_add_sound).unwrap();
    plugin_data.register_sq_functions(info_skip_sound).unwrap();
    plugin_data.register_sq_functions(info_set_speed).unwrap();
    plugin_data.register_sq_functions(info_destroy_sink).unwrap();
}
