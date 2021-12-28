use crate::config::RegMethod;

static PLUGINS: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets/plugins");

struct Config {
    name: String,
    use_photo: bool,
    use_status_files: bool,
    use_subscribes_files: bool,
    use_posts_files: bool,
    reg_methods: Vec<RegMethod>,
}

pub struct Plugin {}

impl Plugin {}

fn create_default_plugins(path: &std::path::PathBuf, log: &mut String) {
    match std::fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(e) => log.push_str(&format!("{:?}\n", e)),
    }

}

pub fn get_list(log: &mut String) -> Vec<String> {
    let mut path = dirs::config_dir().unwrap();
    path.push(std::path::PathBuf::from("flexar/plugins/".to_owned()));
    if !path.exists() {
        create_default_plugins(&path, log);
    }

    let files = match path.read_dir() {
        Ok(e) => Some(e),
        Err(e) => {
            log.push_str(&format!("{:?}\n", e));
            None
        }
    };

    let mut file: Vec<String> = Vec::new();
    if let Some(entries) = files {
        for e in entries {
            if let Ok(entry) = e {
                let mut path = entry.path();
                if let Ok(ftype) = entry.file_type() {
                    if ftype.is_dir() == true {
                        path.set_extension("");
                        file.push(String::from(path.file_name().unwrap().to_str().unwrap()));
                    }
                }
            }
        }
    }
    file
}
