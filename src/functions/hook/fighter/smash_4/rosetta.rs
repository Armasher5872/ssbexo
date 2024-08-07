use super::*;

const ROSETTA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10a81a0; //Rosalina & Luma only
const ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10a88e0; //Rosalina & Luma only
const ROSETTA_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x10a8f60; //Rosalina & Luma only

//Rosalina & Luma Startup Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID);
    original!()(vtable, fighter)
}

//Rosalina & Luma Reset Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_ROSETTA as u32 {
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
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID);
    }
    original!()(vtable, fighter)
}

//Rosalina & Luma Death Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID);
    original!()(vtable, fighter)
}

//Rosalina & Luma Once Per Fighter Frame
#[skyline::hook(offset = ROSETTA_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn rosetta_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let obj_id = WorkModule::get_int(boma, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
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
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        rosetta_start_initialization,
        rosetta_reset_initialization,
        rosetta_death_initialization,
        rosetta_opff
    );
}