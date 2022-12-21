#![allow(unused_macros)]
use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::{
            L2CAgentBase,
            L2CFighterCommon
        },
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

#[fighter_frame( agent = FIGHTER_KIND_PZENIGAME )]
fn squirtle_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
            if MotionModule::frame(module_accessor) >= 10.0
            && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
                && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
                else if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
                && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        squirtle_frame
    );
}