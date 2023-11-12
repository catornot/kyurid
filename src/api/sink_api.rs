use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rand::Rng;
use rodio::Sink;
use rrplug::prelude::*;
use std::collections::HashMap;

use crate::{api::sound_utils::get_sound_file, STREAM};

// note will be reset on each destroy of the client vm
pub static SINKS: Lazy<Mutex<HashMap<String, Sink>>> = Lazy::new(|| Mutex::new(HashMap::new()));

///! rodio sink api
///! any errors will produce error that you will have to catch with `try` and `catch`
///! real

// script_ui print(KYCreateSoundSink())

/// creates a rodio sink and returns a id for it
///
/// `string function KYCreateSoundSink()`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYCreateSoundSink")]
pub fn create_sink() -> Result<String, String> {
    let sink = match Sink::try_new(&STREAM.wait().stream_handle) {
        Ok(s) => s,
        Err(err) => Err(format!("couldn't create sink because of {err}"))?,
    };

    let mut rng = rand::thread_rng();

    let random_key = rng.gen::<u64>().to_string();

    log::info!("the random key is {random_key}");

    if SINKS.lock().insert(random_key.clone(), sink).is_some() {
        log::warn!("lmao, you get to use the same sink now");
    }

    Ok(random_key)
}

// script_ui try { KYAddSoundToSink( "id here", "cat_or_not.mp_sahel", "catvibe.mp3" ) } catch (err) { printt(err) }

/// appends a sound to the sink
///
/// `void function KYAddSoundToSink( string id, string mod_name, string sound_name )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYAddSoundToSink")]
pub fn add_sound(id: String, mod_name: String, sound_name: String) -> Result<(), String> {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => Err(err)?,
    };

    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    sink.append(source);

    Ok(())
}

/// skips the currently playing sound
///
/// `void function KYSkipSinkSound( string id )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSkipSinkSound")]
pub fn skip_sound(id: String) -> Result<(), String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    sink.skip_one();

    Ok(())
}

/// speeds up the sounds >:3
///
/// normal speed is 1.0
///
/// `void function KYSinkSetSpeed( string id, float speed )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSinkSetSpeed")]
pub fn set_speed(id: String, speed: f32) -> Result<(), String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    sink.set_speed(speed);

    Ok(())
}

/// changes the volume
///
/// normal volume is 1.0
///
/// `void function KYSinkSetVolume( string id, float volume )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSinkSetVolume")]
pub fn set_volume(id: String, volume: f32) -> Result<(), String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    sink.set_volume(volume);

    Ok(())
}

/// returns the current speed of the sink
///
/// `float function KYSinkGetVolume( string id )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSinkGetSpeed")]
pub fn get_speed(id: String) -> Result<f32, String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    Ok(sink.speed())
}

/// returns the current volume of the sink
///
/// `float function KYSinkGetVolume( string id )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSinkGetVolume")]
pub fn get_volume(id: String) -> Result<f32, String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    Ok(sink.volume())
}

/// attempts to pause or play the current sound in the sink
///
/// does nothing if it's already in that state
///
/// `void function KYSinkSetPause( string id, bool paused )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSinkSetPause")]
pub fn set_pause(id: String, paused: bool) -> Result<(), String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    if paused {
        sink.pause()
    } else {
        sink.play()
    }

    Ok(())
}

/// returns the pause state of the sink
///
/// `bool function KYSinkGetPause( string id )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYSinkGetPause")]
pub fn get_pause(id: String) -> Result<bool, String> {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => Err(format!("invalid sink id {id}"))?,
    };

    Ok(sink.is_paused())
}

/// destroys the sink which makes also makes the id invalid
///
/// `void function KYDestroySink( string id )`
#[rrplug::sqfunction(VM = "UiClient", ExportName = "KYDestroySink")]
pub fn destroy_sink(id: String) {
    let mut lock = SINKS.lock();
    if lock.remove(&id).is_none() {
        log::warn!("tried to to destroy invalid sink {id}")
    }
}
