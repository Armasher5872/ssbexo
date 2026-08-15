use {
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon
    },
    smashline::*,
};

mod special_s3_1_air;
mod special_s3_2_air;

pub fn install() {
    special_s3_1_air::install();
    special_s3_2_air::install();
}