use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;

pub fn get_sound_file(
    mod_name: String,
    sound_name: String,
) -> Result<Decoder<BufReader<File>>, String> {
    let path = format!("R2Northstar/mods/{mod_name}/sounds/{sound_name}");

    log::info!("sound at {path}");

    let sound_file = match File::open(path) {
        Ok(s) => s,
        Err(err) => Err(format!(
            "couldn't open the file {sound_name} because of {err}"
        ))?,
    };

    let reader = BufReader::new(sound_file);

    match Decoder::new(reader) {
        Ok(s) => Ok(s),
        Err(err) => Err(format!(
            "couldn't decode the file {sound_name} because of {err}"
        )),
    }
}
