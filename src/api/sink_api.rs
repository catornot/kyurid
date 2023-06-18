use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rand::Rng;
use rodio::Sink;
use rrplug::{
    sq_raise_error, sq_return_bool, sq_return_float, sq_return_null, sq_return_string,
    wrappers::squirrel::SQFUNCTIONS,
};
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
#[rrplug::sqfunction(VM=UiClient,ExportName=KYCreateSoundSink)]
pub fn create_sink() -> String {
    let sink = match Sink::try_new(&STREAM.wait().stream_handle) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(
                format!("couldn't create sink because of {err}"),
                sqvm,
                sq_functions
            );
        }
    };

    let mut rng = rand::thread_rng();

    let random_key = rng.gen::<u64>().to_string();

    log::info!("the random key is {random_key}");

    if SINKS.lock().insert(random_key.clone(), sink).is_some() {
        log::warn!("lmao, you get to use the same sink now");
    }

    sq_return_string!(random_key, sqvm, sq_functions);
}

// script_ui try { KYAddSoundToSink( "id here", "cat_or_not.mp_sahel", "catvibe.mp3" ) } catch (err) { printt(err) }

/// appends a sound to the sink
///
/// `void function KYAddSoundToSink( string id, string mod_name, string sound_name )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYAddSoundToSink)]
pub fn add_sound(id: String, mod_name: String, sound_name: String) {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(err, sqvm, sq_functions);
        }
    };

    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.append(source);

    sq_return_null!()
}

/// skips the currently playing sound
///
/// `void function KYSkipSinkSound( string id )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSkipSinkSound)]
pub fn skip_sound(id: String) {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.skip_one();

    sq_return_null!()
}

/// speeds up the sounds >:3
///
/// normal speed is 1.0
///
/// `void function KYSinkSetSpeed( string id, float speed )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkSetSpeed)]
pub fn set_speed(id: String, speed: f32) {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.set_speed(speed);

    sq_return_null!();
}

/// changes the volume
///
/// normal volume is 1.0
///
/// `void function KYSinkSetVolume( string id, float volume )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkSetVolume)]
pub fn set_volume(id: String, volume: f32) {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.set_volume(volume);

    sq_return_null!();
}

/// returns the current speed of the sink
///
/// `float function KYSinkGetVolume( string id )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkGetSpeed)]
pub fn get_speed(id: String) -> f32 {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sq_return_float!(sink.speed(), sqvm, sq_functions);
}

/// returns the current volume of the sink
///
/// `float function KYSinkGetVolume( string id )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkGetVolume)]
pub fn get_volume(id: String) -> f32 {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sq_return_float!(sink.volume(), sqvm, sq_functions);
}

/// attempts to pause or play the current sound in the sink
///
/// does nothing if it's already in that state
///
/// `void function KYSinkSetPause( string id, bool paused )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkSetPause)]
pub fn set_pause(id: String, paused: bool) {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    if paused {
        sink.pause()
    } else {
        sink.play()
    }

    sq_return_null!()
}

/// returns the pause state of the sink
///
/// `bool function KYSinkGetPause( string id )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkGetPause)]
pub fn get_pause(id: String) -> bool {
    let lock = SINKS.lock();
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sq_return_bool!(sink.is_paused(), sqvm, sq_functions);
}

/// destroys the sink which makes also makes the id invalid
///
/// `void function KYDestroySink( string id )`
#[rrplug::sqfunction(VM=UiClient,ExportName=KYDestroySink)]
pub fn destroy_sink(id: String) {
    let mut lock = SINKS.lock();
    if lock.remove(&id).is_none() {
        log::warn!("tried to to destroy invalid sink")
    }

    sq_return_null!();
}
