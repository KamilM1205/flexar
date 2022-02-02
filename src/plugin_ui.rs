use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use eframe::egui::{Area, CentralPanel, CtxRef, Ui};
use mlua::{Function, UserData, UserDataMethods};

pub struct PluginUI {
    ctx: CtxRef,
    labels: Vec<(Box<dyn FnMut(&mut Ui, String)>, String)>,
    lua_log: Rc<RefCell<String>>,
}

impl PluginUI {
    pub fn new(ctx: CtxRef, lua_log: Rc<RefCell<String>>) -> Self {
        Self {
            ctx,
            labels: Vec::new(),
            lua_log,
        }
    }

    fn label(ui: &mut Ui, text: String) {
        ui.label(text);
    }
}

impl UserData for PluginUI {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("show", |_, ctx, _: ()| {
            CentralPanel::default().show(&ctx.ctx, |ui| {
                for i in &mut ctx.labels {
                    i.0.as_mut()(ui, i.1.clone());
                }
            });
            Ok(())
        });

        methods.add_method_mut("label", |_, ctx, text: String| {
            ctx.labels.push((Box::new(PluginUI::label), text));
            Ok(())
        });
    }
}
