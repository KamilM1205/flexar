use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Website {
    VK,
    Instagram,
    Twitter,
    Other(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum RegMethod {
    Phone,
    Email,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Proxy {
    None,
    File(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum PasswordType {
    Generate,
    FromFile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum PasswordFile {
    None,
    File(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum StatusFile {
    None,
    File(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SubscribeFile {
    None,
    File(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum PostsFile {
    None,
    File(String),
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub website: Website,
    pub proxy_use: bool,
    pub proxy_files: Vec<Proxy>,
    pub proxy_sel: Proxy,
    pub use_custom_pas: bool,
    pub default_pas: String,
    pub pas_type: PasswordType,
    pub pas_len: u16,
    pub pas_letters: bool,
    pub pas_nums: bool,
    pub pas_files: Vec<PasswordFile>,
    pub pas_file: PasswordFile,
    pub acc_photo: bool,
    pub acc_status_files: Vec<StatusFile>,
    pub acc_status_file: StatusFile,
    pub acc_sub_files: Vec<SubscribeFile>,
    pub acc_sub_file: SubscribeFile,
    pub acc_posts_files: Vec<PostsFile>,
    pub acc_posts_file: PostsFile,
    pub reg_method: RegMethod,
    pub reg_num: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            website: Website::VK,
            proxy_use: false,
            proxy_files: vec![Proxy::None],
            proxy_sel: Proxy::None,
            use_custom_pas: false,
            default_pas: String::from("Abcd5678"),
            pas_type: PasswordType::Generate,
            pas_len: 8,
            pas_letters: true,
            pas_nums: true,
            pas_files: vec![PasswordFile::None],
            pas_file: PasswordFile::None,
            acc_photo: false,
            acc_status_files: vec![StatusFile::None],
            acc_status_file: StatusFile::None,
            acc_sub_files: vec![SubscribeFile::None],
            acc_sub_file: SubscribeFile::None,
            acc_posts_files: vec![PostsFile::None],
            acc_posts_file: PostsFile::None,
            reg_method: RegMethod::Phone,
            reg_num: 10,
        }
    }
}

impl Config {
    fn load(filename: String, log: &mut String) -> Config {
        let mut path = dirs::config_dir().unwrap();
        path.push(std::path::PathBuf::from("flexar/configs/".to_owned() + &filename));
        path.set_extension(".toml");
        if !path.exists() {
            match std::fs::create_dir_all(path) {
                Err(e) => log.push_str(&format!("{:?}", e))
            }
            let data = match toml::to_string(&Config::default()) {
                Ok(d) => d,
                Err(e) => {log.push_str(&format!("{:?}", e)); "".to_owned()}
            };
            match std::fs::write(path, data) {
                Ok(_) => (),
                Err(e) => log.push_str(&format!("{:?}", e)),
            };
        }
        let data = match std::fs::read_to_string(path) {
            Ok(e) => e,
            Err(e) => {log.push_str(&format!("{:?}", e)); "".to_owned()},
        };
        let decode: Config = match toml::from_str(&data) {
            Ok(e) => e,
            Err(e) => { log.push_str(&format!("{:?}", e)); Config::default() }
        };

        decode
    }

    fn get_list(log: &mut String) -> Vec<String> {
        let mut path = dirs::config_dir().unwrap();
        path.push(std::path::PathBuf::from("flexar/configs/".to_owned()));
        let files = match path.read_dir() {
            Ok(e) => Some(e),
            Err(e) => {log.push_str(&format!("{:?}", e)); None}
        };

        if let Some(files) = files {
            files.map(|f| {
                let f = match f {
                    Ok(f) => Some(f),
                    Err(e) => {log.push_str(&format!("{:?}", e)); None}
                };
                if let Some(f) = f {
                    if f.file_type().unwrap().is_file() {
                        let mut path = std::path::PathBuf::from(f.file_name());
                        let ext = path.extension();
                        if let Some(e) = ext {
                            if e == "toml" {
                                path.set_extension("");
                                return path.to_str().unwrap()
                            }
                        }
                    }
                }
            }).collect();
        }

        return vec![];
    }
    fn save(&mut self, filename: String, log: &mut String) {}
}