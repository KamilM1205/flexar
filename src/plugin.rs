use crate::config::RegMethod;

use eframe::egui::{self, Ui};
use include_dir::DirEntry::{Dir, File};
use mlua::{Lua, LuaOptions, StdLib};

use std::{
    io::{Read, Write},
    sync::mpsc::{channel, Receiver, Sender},
};

pub static PLUGINS: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets/plugins");

struct Config {
    name: String,
    use_photo: bool,
    use_status_files: bool,
    use_subscribes_files: bool,
    use_posts_files: bool,
    reg_methods: Vec<RegMethod>,
}

pub enum PluginCommands {
    LOAD(String),
    UI,
    EXEC,
    ERROR,
    QUIT,
}

pub struct Plugin {
    tx: Option<Sender<PluginCommands>>,
    erx: Option<Receiver<String>>,
    etx: Option<Sender<PluginCommands>>,
}

struct PluginThread {
    tx: Option<Sender<PluginCommands>>,
    etx: Option<Sender<String>>,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            tx: None,
            erx: None,
            etx: None,
        }
    }
}

impl Plugin {
    pub fn run_thread(&mut self) {
        let (tx, rx) = channel::<String>();
        self.erx = Some(rx);
        let mut thread = PluginThread::new();
        
        let (tx, rx) = channel::<PluginCommands>();
        self.tx = Some(tx);
        
        std::thread::spawn(move || {
            thread.run_thread(rx);
        });
    }

    pub fn get_error(&self) -> String {
        self.etx.unwrap().send(PluginCommands::ERROR).unwrap();
        let error = self.erx.clone().unwrap().recv().unwrap();
        error
    }
}

impl PluginThread {
    pub fn new() -> Self {
        Self {
            tx: None,
            etx: None,
        }
    }

    fn load_plugin(&mut self, name: String) -> Result<String, std::io::Error> {
        let mut path = dirs::config_dir().unwrap();
        path.push(name);
        path.push("plugin.lua");
        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut src = String::new();

        match file.read_to_string(&mut src) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(src.clone())
    }

    pub fn run_thread(&mut self, rx: Receiver<PluginCommands>) {
        let lua = Lua::new_with(
            StdLib::MATH | StdLib::STRING | StdLib::TABLE | StdLib::UTF8 | StdLib::PACKAGE,
            LuaOptions::default(),
        ).unwrap();

        match rx.recv() {
            Ok(pc) => match pc {
                PluginCommands::LOAD(name) => {
                    let src = match self.load_plugin(name) {
                        Ok(s) => s,
                        Err(e) => String::new(),
                    };
                    lua.load(&src).exec().unwrap();
                }
                PluginCommands::UI => {}
                PluginCommands::EXEC => {}
                PluginCommands::QUIT => {}
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn get_tx(&self) -> Sender<PluginCommands> {
        self.tx.clone().unwrap()
    }
}

pub fn unpack_plugins(apath: &std::path::Path, ppath: &std::path::Path, log: &mut String) {
    let dir: &include_dir::Dir;
    if ppath.to_string_lossy() == "" {
        dir = &PLUGINS;
    } else {
        dir = match PLUGINS.get_dir(ppath) {
            Some(d) => d,
            None => {
                log.push_str(&format!("Dir: {}, not found.\n", ppath.display()));
                return;
            }
        };
    }

    for entry in dir.entries().iter() {
        match entry {
            File(file) => {
                let mut path = apath.clone().to_path_buf();
                path.push(file.path());
                if !path.exists() {
                    match std::fs::File::create(path) {
                        Ok(mut f) => match write!(f, "{}", file.contents_utf8().unwrap()) {
                            Ok(_) => (),
                            Err(e) => log.push_str(&format!("{:?}", e)),
                        },
                        Err(e) => log.push_str(&format!("{:?}", e)),
                    }
                } else {
                    match std::fs::write(path, file.contents_utf8().unwrap()) {
                        Ok(_) => (),
                        Err(e) => log.push_str(&format!("{:?}", e)),
                    };
                }
            }
            Dir(d) => {
                let mut path = apath.clone().to_path_buf();
                path.push(d.path());
                if !path.exists() {
                    match std::fs::create_dir(path) {
                        Ok(_) => (),
                        Err(e) => log.push_str(&format!("{:?}", e)),
                    }
                }
                unpack_plugins(&apath, d.path(), log);
            }
        }
    }
}

pub fn get_list(log: &mut String) -> Vec<String> {
    let mut path = dirs::config_dir().unwrap();
    path.push("flexar/plugins/".to_owned());
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(e) => log.push_str(&format!("{:?}\n", e)),
        }
        unpack_plugins(&path, PLUGINS.path(), log);
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
