use std::path::PathBuf;

pub fn presence_file(workdir: &str, device: &str) -> PathBuf {
    let mut file = PathBuf::from(workdir);
    file.push("home-control-api-device");
    file.set_extension(device);
    file
}

