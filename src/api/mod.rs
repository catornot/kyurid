use rrplug::prelude::PluginData;

use self::{
    api_utils::{get_all_sounds_in_folder, play_sound, play_sound_tracked},
    sink_api::{
        add_sound, create_sink, destroy_sink, get_pause, get_speed, get_volume, set_pause,
        set_speed, set_volume, skip_sound,
    },
};

pub mod api_utils;
pub mod sink_api;
pub mod sound_utils;

pub fn register_api_sq_functions(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(play_sound);
    plugin_data.register_sq_functions(play_sound_tracked);
    plugin_data.register_sq_functions(get_all_sounds_in_folder);
    plugin_data.register_sq_functions(create_sink);
    plugin_data.register_sq_functions(add_sound);
    plugin_data.register_sq_functions(skip_sound);
    plugin_data.register_sq_functions(set_speed);
    plugin_data.register_sq_functions(destroy_sink);
    plugin_data.register_sq_functions(set_volume);
    plugin_data.register_sq_functions(get_volume);
    plugin_data.register_sq_functions(get_speed);
    plugin_data.register_sq_functions(get_pause);
    plugin_data.register_sq_functions(set_pause);
}
