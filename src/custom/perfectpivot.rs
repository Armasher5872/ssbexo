#![allow(unused_macros)]
use {
    smash::{
        hash40,
        app::{
            lua_bind::{
                PostureModule,
                *
            }
        },
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
    },
    smashline::*,
};

#[fighter_frame_callback]
pub fn perfectpivot(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
        if MotionModule::motion_kind(boma) == hash40("dash") 
        && MotionModule::frame(boma) <= (8.0) {
            let mut stickx = ControlModule::get_stick_x(boma);
            let lr = PostureModule::lr(boma);
            stickx = stickx * lr;
            if stickx < -0.5 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
            };
        };
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        perfectpivot
    );
}