use {
    crate::functions::{
        hook::ui::{
            cloud_meter::*,
            ice_climber_meter::*,
            lucario_meter::*,
            mariod_meter::*,
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

pub mod cloud_meter;
pub mod ice_climber_meter;
pub mod lucario_meter;
pub mod mariod_meter;
pub mod palutena_meter;
pub mod robot_meter;
pub mod ui_hooks;
pub mod ui_manager;
pub mod ui_object;
pub mod utility;

pub fn install() {
    ui_hooks::install();
}