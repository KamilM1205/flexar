use std::{cell::RefCell, rc::Rc};

use eframe::egui::{Align, CentralPanel, CtxRef, ScrollArea, Ui};
use mlua::{Function, Lua, RegistryKey, Table, UserData, UserDataMethods};

#[derive(Clone)]
pub struct PluginUI {
    pub ctx: CtxRef,
    pub widgets: Vec<Rc<RefCell<(Box<dyn FnMut(&PluginUI, &mut Ui, Table)>, RegistryKey)>>>,
    pub lua: &'static Lua,
    pub lua_log: Rc<RefCell<String>>,
}

impl PluginUI {
    pub fn new(ctx: CtxRef, lua: &'static Lua, lua_log: Rc<RefCell<String>>) -> Self {
        Self {
            ctx,
            widgets: Vec::new(),
            lua,
            lua_log,
        }
    }

    fn label(_pui: &PluginUI, ui: &mut Ui, table: Table) {
        ui.label(
            table
                .get::<&str, String>("text")
                .unwrap_or("label".to_owned()),
        );
    }

    fn collapsing(pui: &PluginUI, ui: &mut Ui, table: Table) {
        let label = table
            .get::<&str, String>("text")
            .unwrap_or("Collapsing".to_owned());
        let func = table.get::<&str, Option<Function>>("cb").unwrap_or(None);
        ui.collapsing(label, |ui| {
            if let Some(f) = func {
                let mut pui_arg = PluginUI::new(pui.ctx.clone(), pui.lua, pui.lua_log.clone());
                pui_arg = f.call::<PluginUI, PluginUI>(pui_arg).unwrap_or_else(|e| {
                    (*pui).lua_log.borrow_mut().push_str(&format!("{:?}", e));
                    PluginUI::new(pui.ctx.clone(), pui.lua, pui.lua_log.clone())
                });
                pui_arg.draw_ui(ui);
            }
        });
    }

    fn button(pui: &PluginUI, ui: &mut Ui, table: Table) {
        let label = table
            .get::<&str, String>("text")
            .unwrap_or("Button".to_owned());
        let func = table.get::<&str, Option<Function>>("cb").unwrap_or(None);
        if ui.button(label).clicked() {
            if let Some(f) = func {
                f.call(()).unwrap_or_else(|e| {
                    (*pui).lua_log.borrow_mut().push_str(&format!("{:?}", e));
                });
            }
        };
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {
        for i in self.widgets.clone() {
            let reg_val = self.lua.registry_value::<Table>(&i.borrow().1).unwrap();
            (*i).borrow_mut().0(self, ui, reg_val);
        }
    }
}

impl UserData for PluginUI {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // drawing ui
        methods.add_method_mut("show", |_, ctx, _: ()| {
            CentralPanel::default().show(&ctx.ctx.clone(), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(
                        eframe::egui::Layout::top_down_justified(Align::Center),
                        |ui| {
                            ctx.draw_ui(ui);
                        },
                    );
                });
            });
            Ok(())
        });

        // adding label for draw
        methods.add_method_mut("label", |lua, ctx, table: Table| {
            let reg_key = lua.create_registry_value(table).unwrap();
            ctx.widgets
                .push(Rc::new(RefCell::new((Box::new(PluginUI::label), reg_key))));
            Ok(())
        });

        // adding CollapsingHeader to draw
        methods.add_method_mut("collapsing", |lua, ctx, table: Table| {
            let reg_key = lua.create_registry_value(table).unwrap();
            ctx.widgets.push(Rc::new(RefCell::new((
                Box::new(PluginUI::collapsing),
                reg_key,
            ))));
            Ok(())
        });

        methods.add_method_mut("button", |lua, ctx, table: Table| {
            let reg_key = lua.create_registry_value(table).unwrap();
            ctx.widgets
                .push(Rc::new(RefCell::new((Box::new(PluginUI::button), reg_key))));
            Ok(())
        });
    }
}
