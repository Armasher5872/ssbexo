use super::*;

unsafe extern "C" fn link_bowarrow_frame(weapon: &mut L2CFighterBase) {
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR)
    && StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_FLY
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        WorkModule::set_flag(weapon.module_accessor, true, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
        let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
        TeamModule::set_team(item_boma, team_no, true);
        TeamModule::set_team_owner_id(item_boma, team_owner_id);
    }
}

unsafe extern "C" fn link_boomerang_frame(weapon: &mut L2CFighterBase) {
    let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR)
    && status_kind == *WN_LINK_BOOMERANG_STATUS_KIND_FLY
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        WorkModule::set_flag(weapon.module_accessor, true, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
        let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
        TeamModule::set_team(item_boma, team_no, true);
        TeamModule::set_team_owner_id(item_boma, team_owner_id);
    }
    if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_SHIELD))
    && [*WN_LINK_BOOMERANG_STATUS_KIND_TURN, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED].contains(&status_kind)
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
            let team_no = WorkModule::get_int(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
            TeamModule::set_team(weapon.module_accessor, team_no, true);
            TeamModule::set_team_owner_id(weapon.module_accessor, (*(owner_boma)).battle_object_id);
            WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        }
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_boma, status, false);
        }
    }
}

unsafe extern "C" fn link_bowarrow_init(weapon: &mut L2CFighterBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(boma, 0, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
    WorkModule::set_int(boma, 0, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_flag(boma, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    WorkModule::set_flag(boma, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
    WorkModule::set_flag(boma, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW);
}

unsafe extern "C" fn link_boomerang_init(weapon: &mut L2CFighterBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
    WorkModule::set_int(boma, 0, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_flag(boma, false, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    WorkModule::set_flag(boma, false, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
}

pub fn install() {
    Agent::new("link_bowarrow")
    .on_start(link_bowarrow_init)
    .on_line(Main, link_bowarrow_frame)
    .install()
    ;
    Agent::new("link_boomerang")
    .on_start(link_boomerang_init)
    .on_line(Main, link_boomerang_frame)
    .install()
    ;
}