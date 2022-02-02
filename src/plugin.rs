use crate::{config::RegMethod, plugin_ui};

use eframe::egui::{self, CtxRef, Ui};
use include_dir::DirEntry::{Dir, File};
use mlua::{Function, Lua, LuaOptions, StdLib, Variadic};

use std::{
    cell::RefCell,
    io::{Read, Write},
    rc::Rc,
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

pub struct Plugin {
    name: String,
    lua: Option<Lua>,
    src: String,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            lua: None,
            src: String::new(),
        }
    }

    // Загрузка плагина и получение chunk'а для выполнения плагина
    pub fn load(&mut self, name: Option<String>, log: &mut String, lua_log: Rc<RefCell<String>>) {
        let sname;
        if let Some(n) = name {
            sname = n
        } else {
            self.lua = None;
            sname = "None".to_owned();
            return;
        }

        let lua = match Lua::new_with(
            StdLib::MATH | StdLib::STRING | StdLib::UTF8 | StdLib::TABLE | StdLib::PACKAGE,
            LuaOptions::default(),
        ) {
            Ok(lua) => lua,
            Err(e) => {
                log.push_str(&format!("{:?}\n", e));
                return;
            }
        };

        let src = Plugin::load_plugin_file(&sname, log);

        self.name = sname;
        self.lua = Some(lua);
        self.src = src;

        self.setup_base(lua_log, log);
        self.call_load(log);
    }

    fn setup_base(&mut self, lua_log: Rc<RefCell<String>>, log: &mut String) {
        let lua = self.lua.as_ref().unwrap();
        let globals = lua.globals();

        let lua_print = match lua.create_function(move |_, strings: Variadic<String>| {
            *lua_log.borrow_mut() = format!(
                "{}{}\n",
                *lua_log.borrow(),
                strings.iter().fold(String::new(), |a, b| a + b)
            );
            Ok(())
        }) {
            Ok(f) => f,
            Err(e) => {
                log.push_str(&format!("{:?}", e));
                return;
            }
        };

        match globals.set("print", lua_print) {
            Ok(_) => (),
            Err(e) => log.push_str(&format!("{:?}", e)),
        };
    }

    fn call_load(&mut self, log: &mut String) {
        let lua = self.lua.as_ref().unwrap();
        let chunk = lua.load(&self.src);

        match chunk.exec() {
            Ok(_) => (),
            Err(e) => log.push_str(&format!("{:?}\n", e)),
        };

        let globals = lua.globals();

        let load: Option<Function> = match globals.get("load") {
            Ok(v) => Some(v),
            Err(e) => {
                log.push_str(&format!("{:?}\n", e));
                None
            }
        };

        match load {
            Some(v) => v.call(()).unwrap(),
            None => return,
        }
    }

    pub fn call_draw(
        &mut self,
        ctx: CtxRef,
        lua_log: Rc<RefCell<String>>,
        log: &mut String,
    ) -> Result<(), ()> {
        if let None = self.lua {
            return Ok(());
        }
        let globals = self.lua.as_ref().unwrap().globals();

        let draw: Option<Function> = match globals.get("draw") {
            Ok(v) => Some(v),
            Err(e) => {
                log.push_str(&format!("{:?}\n", e));
                None
            }
        };

        let pui = plugin_ui::PluginUI::new(ctx, lua_log.clone());
        match draw {
            Some(v) => match v.call::<plugin_ui::PluginUI, ()>(pui) {
                Ok(_) => Ok(()),
                Err(e) => {
                    log.push_str(&format!("{:?}", e));
                    Err(())
                }
            },
            None => return Ok(()),
        }
    }

    // Загрузка lua плагина из файла
    fn load_plugin_file(name: &String, log: &mut String) -> String {
        let mut path = dirs::config_dir().unwrap();
        path.push(format!("{}/{}/{}", "flexar/plugins", name, "plugin.lua"));

        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                log.push_str(&format!("{:?}\n", e));
                return String::new();
            }
        };

        let mut src = String::new();

        match file.read_to_string(&mut src) {
            Ok(_) => (),
            Err(e) => {
                log.push_str(&format!("{:?}\n", e));
                return String::new();
            }
        };

        src
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
                            Err(e) => log.push_str(&format!("{:?}\n", e)),
                        },
                        Err(e) => log.push_str(&format!("{:?}\n", e)),
                    }
                } else {
                    match std::fs::write(path, file.contents_utf8().unwrap()) {
                        Ok(_) => (),
                        Err(e) => log.push_str(&format!("{:?}\n", e)),
                    };
                }
            }
            Dir(d) => {
                let mut path = apath.clone().to_path_buf();
                path.push(d.path());
                if !path.exists() {
                    match std::fs::create_dir(path) {
                        Ok(_) => (),
                        Err(e) => log.push_str(&format!("{:?}\n", e)),
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
