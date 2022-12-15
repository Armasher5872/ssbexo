//Credit to Championship Edition
#![allow(unused_macros)]
use {
    smash::{
        app::lua_bind::*, 
        hash40,
        lib::{
            lua_const::*, 
        },
        lua2cpp::L2CFighterBase
    },
    smashline::*,
};

#[weapon_frame( agent = WEAPON_KIND_PIKMIN_PIKMIN )]
fn pikmin_functions(fighter: &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let owner_module_accessor = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
        let owner_motion_kind = MotionModule::motion_kind(owner_module_accessor);
        let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
        if weapon_kind == WEAPON_KIND_PIKMIN_PIKMIN {
            if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&owner_status_kind) {
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_OFF), 0);
            }
            if (owner_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF && owner_motion_kind != hash40("just_shield_off")) || owner_status_kind == *FIGHTER_STATUS_KIND_FURAFURA {
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);	
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        pikmin_functions
    );
}
