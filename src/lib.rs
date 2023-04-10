use once_cell::sync::OnceCell;
use rrplug::{
    bindings::convar::FCVAR_GAMEDLL,
    sq_return_null,
    wrappers::convars::{ConVarRegister, ConVarStruct},
    wrappers::northstar::{EngineLoadType, PluginData},
};
use rrplug::{prelude::*, sq_raise_error};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

struct Stream {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

static STREAM: OnceCell<Stream> = OnceCell::new();

pub struct NativeAudio;

impl std::fmt::Debug for NativeAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeAudio")
            .field("sink", &"sink")
            .finish()
    }
}

impl Plugin for NativeAudio {
    fn new() -> Self {
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(_) => {
                log::error!("couldn't get access to a midi device");
                unimplemented!()
            }
        };

        _ = STREAM.set(Stream {
            _stream,
            stream_handle,
        });

        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        _ = plugin_data.register_sq_functions(info_play_sound);
    }

    fn main(&self) {}

    fn on_engine_load(&self, engine: EngineLoadType) {
        let engine = match engine {
            EngineLoadType::Engine(engine) => engine,
            EngineLoadType::EngineFailed => return,
            EngineLoadType::Server => return,
            EngineLoadType::Client => return,
        };

        let convar = ConVarStruct::try_new().unwrap();
        let register_info = ConVarRegister {
            callback: Some(basic_convar_changed_callback),
            ..ConVarRegister::mandatory(
                "basic_convar",
                "48",
                FCVAR_GAMEDLL.try_into().unwrap(),
                "basic_convar",
            )
        };

        convar.register(register_info).unwrap();

        _ = engine.register_concommand(
            "basic_command",
            basic_command_callback,
            "basic_command",
            FCVAR_GAMEDLL.try_into().unwrap(),
        );
    }
}

#[rrplug::concommand]
fn basic_command_callback(command: CCommandResult) {
    log::info!("running basic_command");
    log::info!("args: {:?}", command.args)
}

#[rrplug::convar]
fn basic_convar_changed_callback(
    convar: Option<ConVarStruct>,
    old_value: String,
    float_old_value: f32,
) {
    log::info!("old value: {}", float_old_value)
}

// test command
// script_ui try { NAPlaySoundFile( "cat_or_not.mp_sahel", "03_Klaxon_Beat.mp3" ) } catch (err) { printt(err) }
#[rrplug::sqfunction(VM=UiClient,ExportName=NAPlaySoundFile)]
fn play_sound(mod_name: String, sound_name: String) {
    let path = format!("R2Northstar/mods/{mod_name}/sounds/{sound_name}");

    log::info!("playing sound at {path}");

    let sound_file = match File::open(path) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(
                format!("couldn't open the file {sound_name} because of {err}"),
                sqvm,
                sq_functions
            );
        }
    };

    let reader = BufReader::new(sound_file);

    let source = match Decoder::new(reader) {
        Ok(s) => s,
        Err(err) => {
            sq_raise_error!(
                format!("couldn't decode the file {sound_name} because of {err}"),
                sqvm,
                sq_functions
            );
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

entry!(NativeAudio);
