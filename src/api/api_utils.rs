use rand::Rng;
use rodio::Sink;
use rrplug::prelude::*;

use crate::api::sink_api::SINKS;
use crate::api::sound_utils::{get_all_sounds, get_sound_file};
use crate::STREAM;

// test command
// script_ui try { KYPlaySoundFile( "cat_or_not.mp_sahel", "03_Klaxon_Beat.mp3" ) } catch (err) { printt(err) }

/// low overhead
///
/// just spawns a sound and plays it until it ends or when the game stops runnning
///
/// good for short sounds
///
/// `void function KYPlaySoundFile( string mod_name, string sound_name )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYPlaySoundFile")]
pub fn play_sound(mod_name: String, sound_name: String) -> Result<(), String> {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => Err(err)?,
    };

    let sink = match Sink::try_new(&STREAM.wait().stream_handle) {
        Ok(s) => s,
        Err(err) => Err(format!("couldn't create sink because of {err}"))?,
    };

    sink.append(source);
    sink.detach();

    Ok(())
}

/// higher overhead, can also fill up the memory since sinks will only be cleaned on client destruction
///
/// `void function KYPlaySoundFileTracked( string mod_name, string sound_name )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYPlaySoundFileTracked")]
pub fn play_sound_tracked(mod_name: String, sound_name: String) -> Result<(), String> {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => Err(err)?,
    };

    let sink = match Sink::try_new(&STREAM.wait().stream_handle) {
        Ok(s) => s,
        Err(err) => Err(format!("couldn't create sink because of {err}"))?,
    };

    sink.append(source);

    let mut rng = rand::thread_rng();

    let random_key = rng.gen::<u64>().to_string();

    log::info!("the random key is {random_key}");

    if SINKS.lock().insert(random_key, sink).is_some() {
        log::warn!("lmao, you get to use the same sink now");
    }

    Ok(())
}

/// gets all the sounds from a folder
///
/// `array<string> function KYGetSounds(string mod_name)`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYGetSounds")]
pub fn get_all_sounds_in_folder(mod_name: String) -> Vec<String> {
    get_all_sounds(mod_name)
        .map_err(|err| _ = log::error!("{err}"))
        .unwrap_or(Vec::new())
}
