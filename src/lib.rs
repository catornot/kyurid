use once_cell::sync::OnceCell;
use rodio::{OutputStream, OutputStreamHandle};
use rrplug::prelude::*;
use rrplug::wrappers::northstar::{PluginData, ScriptVmType};

use api::register_api_sq_functions;

mod api;

pub struct Stream {
    pub _stream: OutputStream,
    #[allow(dead_code)] // for now
    pub stream_handle: OutputStreamHandle,
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

pub static STREAM: OnceCell<Stream> = OnceCell::new();

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
        register_api_sq_functions(plugin_data);

        let args: String = std::env::args().skip(1).collect();

        log::info!("args {args}"); // some stuff
    }

    fn main(&self) {}

    fn on_sqvm_destroyed(&self, context: ScriptVmType) {
        if let ScriptVmType::Client = context {
            crate::api::sink_api::SINKS.lock().expect("how").clear()
        }
    }
}

entry!(NativeAudio);
