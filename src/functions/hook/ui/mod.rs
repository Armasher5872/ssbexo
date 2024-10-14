use {
    crate::functions::{
        hook::ui::{
            palutena_meter::*,
            robot_meter::*,
            ui_manager::*,
            ui_object::*,
            utility::*,
        },
        util::*,
    },
    once_cell::sync::Lazy,
    parking_lot::RwLock
};

pub mod palutena_meter;
pub mod robot_meter;
pub mod ui_hooks;
pub mod ui_manager;
pub mod ui_object;
pub mod utility;

pub fn install() {
    ui_hooks::install();
}