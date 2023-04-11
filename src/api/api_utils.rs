use rand::Rng;
use rodio::Sink;
use rrplug::sq_return_null;
use rrplug::{sq_raise_error, wrappers::squirrel::SQFUNCTIONS};

use crate::api::sink_api::SINKS;
use crate::api::sound_utils::get_sound_file;
use crate::STREAM;

// test command
// script_ui try { KYPlaySoundFile( "cat_or_not.mp_sahel", "03_Klaxon_Beat.mp3" ) } catch (err) { printt(err) }

/// low overhead
#[rrplug::sqfunction(VM=UiClient,ExportName=KYPlaySoundFile)]
pub fn play_sound(mod_name: String, sound_name: String) {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(err, sqvm, sq_functions);
        }
    };

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

    sink.append(source);
    sink.detach();

    sq_return_null!()
}

/// higher overhead, can also fill up the memory since sinks will only be cleaned on client destruction
#[rrplug::sqfunction(VM=UiClient,ExportName=KYPlaySoundFileTracked)]
pub fn play_sound_tracked(mod_name: String, sound_name: String) {
    let source = match get_sound_file(mod_name, sound_name) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(err, sqvm, sq_functions);
        }
    };

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

    sink.append(source);

    let mut rng = rand::thread_rng();

    let random_key = rng.gen::<u64>().to_string();

    log::info!("the random key is {random_key}");

    if SINKS
        .lock()
        .expect("how")
        .insert(random_key, sink)
        .is_some()
    {
        log::warn!("lmao, you get to use the same sink now");
    }

    sq_return_null!()
}
