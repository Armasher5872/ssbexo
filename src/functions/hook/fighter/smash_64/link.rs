use super::*;

const LINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const LINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const LINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared
const LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc289e0; //Shared

//Link Startup Initialization
#[skyline::hook(offset = LINK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let team_no = TeamModule::team_no(boma) as i32;
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
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
    original!()(vtable, fighter)
}

//Link Reset Initialization
#[skyline::hook(offset = LINK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let team_no = TeamModule::team_no(boma) as i32;
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
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
    original!()(vtable, fighter)
}

//Link Death Initialization
#[skyline::hook(offset = LINK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let team_no = TeamModule::team_no(boma) as i32;
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
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
    original!()(vtable, fighter)
}

//Link Once Per Fighter Frame
#[skyline::hook(offset = LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn link_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        if !ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
            if WorkModule::is_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
            else {
                let boomerang_fuse_item_id = WorkModule::get_int(boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID) as u32;
                if sv_battle_object::is_active(boomerang_fuse_item_id) {
                    let item_boma = smash::app::sv_battle_object::module_accessor(boomerang_fuse_item_id);
                    if StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
                        if smash::app::utility::get_kind(&mut *item_boma) != *ITEM_KIND_LINKBOMB {
                            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
                            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, boomerang_fuse_item_id);
                        }
                        else {
                            StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_THROW, false);
                            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                            WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
                        }
                    }
                }
            }
            if ItemModule::is_have_item(boma, 0) {
                WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
            }
        }
        if WorkModule::is_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
            let item_id = WorkModule::get_int(boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            if !sv_battle_object::is_active(item_id) {
                ModelModule::set_mesh_visibility(boma, Hash40::new("link_ken"), true);
                WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        link_start_initialization,
        link_reset_initialization,
        link_death_initialization,
        link_opff
    );
}