use once_cell::sync::Lazy;
use rand::Rng;
use rodio::Sink;
use rrplug::{sq_raise_error, sq_return_null, sq_return_string, wrappers::squirrel::SQFUNCTIONS};
use std::{collections::HashMap, sync::Mutex};

use crate::{api::sound_utils::get_sound_file, STREAM};

// note will be reset on each destroy of the client vm
pub static SINKS: Lazy<Mutex<HashMap<String, Sink>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// script_ui print(KYCreateSoundSink())
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

    if SINKS
        .lock()
        .expect("how")
        .insert(random_key.clone(), sink)
        .is_some()
    {
        log::warn!("lmao, you get to use the same sink now");
    }

    sq_return_string!(random_key, sqvm, sq_functions);
}

// script_ui try { KYAddSoundToSink( "id here", "cat_or_not.mp_sahel", "catvibe.mp3" ) } catch (err) { printt(err) }
#[rrplug::sqfunction(VM=UiClient,ExportName=KYAddSoundToSink)]
pub fn add_sound(id: String, mod_name: String, sound_name: String) {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(err, sqvm, sq_functions);
        }
    };

    let lock = SINKS.lock().expect("how");
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.append(source);

    sq_return_null!()
}

#[rrplug::sqfunction(VM=UiClient,ExportName=KYSkipSinkSound)]
pub fn skip_sound(id: String) {
    let lock = SINKS.lock().expect("how");
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.skip_one();

    sq_return_null!()
}

/// normal speed is 1.0
#[rrplug::sqfunction(VM=UiClient,ExportName=KYSinkSetSpeed)]
pub fn set_speed(id: String, speed: f32) {
    let lock = SINKS.lock().expect("how");
    let sink = match lock.get(&id) {
        Some(s) => s,
        None => {
            sq_raise_error!(format!("invalid sink id"), sqvm, sq_functions);
        }
    };

    sink.set_speed(speed);

    sq_return_null!();
}

#[rrplug::sqfunction(VM=UiClient,ExportName=KYDestroySink)]
pub fn destroy_sink(id: String) {
    let mut lock = SINKS.lock().expect("how");
    if lock.remove(&id).is_none() {
        log::warn!("tried to to destroy invalid sink")
    }

    sq_return_null!();
}
