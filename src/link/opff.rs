use super::*;

unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    //Boomerang
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
            WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
            WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
        }
        else {
            let boomerang_fuse_item_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID) as u32;
            if sv_battle_object::is_active(boomerang_fuse_item_id) {
                let item_boma = smash::app::sv_battle_object::module_accessor(boomerang_fuse_item_id);
                if StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
                    if smash::app::utility::get_kind(&mut *item_boma) != *ITEM_KIND_LINKBOMB {
                        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
                        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, boomerang_fuse_item_id);
                    }
                    else {
                        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_THROW, false);
                        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                        WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
                    }
                }
            }
        }
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
        }
    }
    //Bomb
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
        let item_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        if !sv_battle_object::is_active(item_id) {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("link_ken"), true);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
        }
    }
}

unsafe extern "C" fn link_bowarrow_frame(weapon: &mut L2CFighterBase) {
    let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
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

unsafe extern "C" fn link_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let team_no = TeamModule::team_no(boma) as i32;
    //Universal
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    //Link
    WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
    WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
    WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_int(boma, team_no, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
    WorkModule::set_int(boma, 0, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    WorkModule::set_float(boma, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(boma, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
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
    Agent::new("link")
    .on_start(link_init)
    .on_line(Main, link_frame)
    .install()
    ;
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