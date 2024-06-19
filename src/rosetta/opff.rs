use super::*;

unsafe extern "C" fn tico_frame(fighter: &mut L2CFighterBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let owner_boma = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let motion_kind = MotionModule::motion_kind(boma);
    let owner_motion_kind = MotionModule::motion_kind(owner_boma);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&owner_status_kind)
    || [*WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_GUARD, *WEAPON_ROSETTA_TICO_STATUS_KIND_FOLLOW_GUARD].contains(&status_kind)
    || motion_kind == hash40("follow_guard") {
        HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    if ([*FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_FURAFURA].contains(&owner_status_kind) && owner_motion_kind != hash40("just_shield_off")) {
        HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);	
    }
}

pub fn install() {
    Agent::new("rosetta_tico")
    .on_line(Main, tico_frame)
    .install()
    ;
}