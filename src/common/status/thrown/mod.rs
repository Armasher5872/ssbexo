use {
    crate::functions::ext::utility::misc::*,
    smash::{
        app::lua_bind::*,
        lib::L2CValue,
        lua2cpp::L2CFighterCommon,
        phx::Vector3f
    }
};

mod thrown;

pub fn install() {
    thrown::install();
}