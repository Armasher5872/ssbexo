#![allow(unused_macros)]
use {
    crate::functions::{
        USE_ONSLAUGHT
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
    },
    smashline::*,
};


#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn mii_brawler_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
        || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            USE_ONSLAUGHT[entry_id] = true;
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END].contains(&status_kind) {
            fighter.sub_transition_group_check_air_cliff();
            USE_ONSLAUGHT[entry_id] = false;
        }
        if [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK].contains(&status_kind) {
            fighter.sub_transition_group_check_air_cliff();
        }
    }
}

pub fn install() {
    install_agent_frames!(
        mii_brawler_frame
    );
}