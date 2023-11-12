use once_cell::sync::OnceCell;
use rodio::{OutputStream, OutputStreamHandle};
use rrplug::prelude::*;

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

impl Plugin for NativeAudio {
    fn new(plugin_data: &PluginData) -> Self {
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

        register_api_sq_functions(plugin_data);

        // let args: String = std::env::args().skip(1).collect();

        // log::info!("args {args}"); // some stuff

        Self {}
    }

    fn on_sqvm_destroyed(&self, context: ScriptVmType) {
        if let ScriptVmType::Client = context {
            crate::api::sink_api::SINKS.lock().clear()
        }
    }
}

entry!(NativeAudio);
