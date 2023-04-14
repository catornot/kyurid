use rodio::Decoder;
use std::fs::{self, File};
use std::io::BufReader;

pub fn get_sound_file(
    mod_name: String,
    sound_name: String,
) -> Result<Decoder<BufReader<File>>, String> {
    let path = format!("R2Northstar/mods/{mod_name}/sounds/{sound_name}");

    log::info!("sound at {path}");

    validate_user_path(&path)?;

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

// wtf this doesn't work?
fn validate_user_path(path: &str) -> Result<(), String> {
    if path.contains("..") {
        Err("BRUH, stop trying to break the path")?
    }
    Ok(())
}

pub fn get_all_sounds(mod_name: String) -> Result<Vec<String>, String> {
    let path = format!("R2Northstar/mods/{mod_name}/sounds/");

    validate_user_path(&path)?;

    let folder = fs::read_dir(path.clone())
        .map_err(|err| format!("couldn't open the folder at {path} because of {err}"))?;

    Ok(folder
        .filter_map(|file| match file {
            Ok(file) if file.path().is_file() => {
                Some(file.file_name().to_string_lossy().to_string())
            }
            Ok(_) => None,
            Err(err) => {
                log::warn!("faulty file : {err}");
                None
            }
        })
        .collect())
}
