use crate::config::RegMethod;

use eframe::egui::{self, Ui};
use include_dir::DirEntry::{Dir, File};
use rlua::{Function, Lua, StdLib};

use std::{
    io::{Read, Write},
    thread::JoinHandle,
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
    QUIT,
}

pub struct Plugin {
    src: Option<String>,
    rx: std::sync::mpsc::Receiver<PluginCommands>,
    tx: std::sync::mpsc::Sender<PluginCommands>,
    thread: Option<JoinHandle<()>>,
    error_message: String,
}

impl Default for Plugin {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            src: None,
            rx,
            tx,
            thread: None,
            error_message: String::new(),
        }
    }
}

impl Plugin {
    pub fn load_plugin(
        tx: std::sync::mpsc::Sender<PluginCommands>,
        plugin_name: String,
        log: &mut String,
    ) -> String {
        let mut path = dirs::config_dir().unwrap();
        path.push("flexar/plugins/".to_owned());
        path.push(plugin_name);
        path.push("plugin.lua");

        let mut src = String::new();
        match std::fs::File::open(path) {
            Ok(mut f) => match f.read_to_string(&mut src) {
                Ok(_) => (),
                Err(e) => {
                    log.push_str(&format!("{:?}", e));
                    return "".to_owned();
                }
            },
            Err(e) => {
                log.push_str(&format!("{:?}", e));
                return "".to_owned();
            }
        }

        src
    }

    pub fn start_thread(&mut self) {
        let plugin = Lua::new_with(
            StdLib::BASE & StdLib::UTF8 & StdLib::TABLE & StdLib::STRING & StdLib::MATH,
        );
        plugin.context(|mut ctx| loop {
            let cmd = self.rx.recv().unwrap();

            match cmd {
                PluginCommands::LOAD(src) => {
                    self.src = Some(src);
                    self.load_lua(&mut ctx)
                }
                PluginCommands::UI => (),
                PluginCommands::QUIT => break,
            }
        });
    }

    fn load_lua(&mut self, ctx: &mut rlua::Context) {
        let globals = ctx.globals();
        match &mut self.src {
            Some(src) => {
                let chunk = ctx.load(&src);
                let f: Result<Function, rlua::Error> = globals.get("load");
                match f {
                    Ok(f) => match f.call::<_, ()>(()) {
                        Ok(_) => (),
                        Err(e) => {
                            &self.error(e);
                            ()
                        }
                    },
                    Err(e) => {
                        &self.error(e);
                        ()
                    }
                };
                chunk.exec();
            }
            None => (),
        }
    }

    fn error<T>(&mut self, err: T)
    where
        T: std::fmt::Debug,
    {
        self.error_message.push_str(&format!("{:?}", err));
    }

    /*pub fn load(&mut self, tx: std::sync::mpsc::Sender<PluginCommands>, log: &mut String) {
        match tx.send(PluginCommands::LOAD) {
            Ok(_) => (),
            Err(e) => log.push_str(&format!("{:?}", e)),
        }
    }*/

    pub fn draw_ui(
        &mut self,
        tx: std::sync::mpsc::Sender<PluginCommands>,
        ui: &mut Ui,
        log: &mut String,
    ) {
        match tx.send(PluginCommands::UI) {
            Ok(_) => (),
            Err(e) => {
                log.push_str(&format!("{:?}", e));
                return;
            }
        };
    }

    pub fn get_tx(&self) -> std::sync::mpsc::Sender<PluginCommands> {
        self.tx.clone()
    }

    pub fn stop_thread(&mut self, tx: std::sync::mpsc::Sender<PluginCommands>) {
        tx.send(PluginCommands::QUIT).unwrap();
        /*match self.thread {
            Some(t) => {
                t.join();
            }
            None => (),
        }*/
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
