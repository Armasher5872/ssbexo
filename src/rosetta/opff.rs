use super::*;

unsafe extern "C" fn rosetta_frame(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let obj_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
        let obj_boma = smash::app::sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let item_id = if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            WorkModule::get_int64(obj_boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else if obj_kind == *WEAPON_KIND_LINK_BOOMERANG {
            WorkModule::get_int64(obj_boma, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else {
            *BATTLE_OBJECT_ID_INVALID as u32
        };
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
    }
}

unsafe extern "C" fn tico_frame(fighter: &mut L2CFighterBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let owner_boma = &mut *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_kind = smash::app::utility::get_kind(boma) as i32;
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
    Agent::new("rosetta")
    .on_line(Main, rosetta_frame)
    .install()
    ;
    Agent::new("rosetta_tico")
    .on_line(Main, tico_frame)
    .install()
    ;
}