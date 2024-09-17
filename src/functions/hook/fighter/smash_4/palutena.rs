use super::*;

const PALUTENA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe5e6a0; //Palutena only
const PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe5f350; //Palutena only
const PALUTENA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xe5f3e0; //Palutena only
const PALUTENA_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0xe61dc0; //Palutena only

//Palutena Startup Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
    WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
    WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_ID);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_TIMER);
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    UiManager::change_palutena_meter_color_green(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32);
    UiManager::set_palutena_meter_info(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, 100.0, 100.0, 50.0);
    UiManager::reset_palutena_meter(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32);
    original!()(vtable, fighter)
}

//Palutena Reset Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_PALUTENA as u32 {
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
        WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
        WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
        WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
        WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_ID);
        WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_TIMER);
        WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
        WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
        UiManager::change_palutena_meter_color_green(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32);
        UiManager::set_palutena_meter_info(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, 100.0, 100.0, 50.0);
        UiManager::reset_palutena_meter(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32);
    }
    original!()(vtable, fighter)
}

//Palutena Death Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
    WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
    WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_ID);
    WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_TIMER);
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    UiManager::change_palutena_meter_color_green(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32);
    UiManager::set_palutena_meter_info(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, 100.0, 100.0, 50.0);
    UiManager::reset_palutena_meter(WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32);
    original!()(vtable, fighter)
}

//Palutena Once Per Fighter Frame
#[skyline::hook(offset = PALUTENA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn palutena_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let lightweight_timer = WorkModule::get_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
    let lightweight_effect_timer = WorkModule::get_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
    let lightweight_burnout_timer = WorkModule::get_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
    let lightweight_burnout_effect_timer = WorkModule::get_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
    let lightweight_meter_timer = WorkModule::get_float(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    let lightweight_burnout_meter_timer = WorkModule::get_float(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    let angelic_missile_id = WorkModule::get_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_ID);
    let angelic_missile_timer = WorkModule::get_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_TIMER);
    if angelic_missile_id != *BATTLE_OBJECT_ID_INVALID {
        let opponent_boma = sv_battle_object::module_accessor(angelic_missile_id as u32);
        let opponent_lua_module_fighter = get_fighter_common_from_accessor(&mut *opponent_boma);
        let opponent_status_kind = StatusModule::status_kind(opponent_boma);
        let opponent_situation_kind = StatusModule::situation_kind(opponent_boma);
        let opponent_motion_kind = MotionModule::motion_kind(opponent_boma);
        sv_kinetic_energy!(set_speed, opponent_lua_module_fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE, 2.5*PostureModule::lr(boma), 0.0);
        sv_kinetic_energy!(set_speed, opponent_lua_module_fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        ModelModule::set_joint_rotate(opponent_boma, Hash40::new("rot"), &Vector3f{x: 270.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: 0 as u8}, MotionNodeRotateOrder{_address: 0 as u8});
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
        WorkModule::unable_transition_term_group(opponent_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_WALL_JUMP);
        if opponent_motion_kind != hash40("damage_fly_roll") {
            MotionModule::change_motion(opponent_boma, Hash40::new("damage_fly_roll"), 0.0, 1.0, false, 0.0, false, false);
        }
        if angelic_missile_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_TIMER);
        }
        else {
            if opponent_situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
            else {
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_WAIT, false);
            }
            AttackModule::clear(opponent_boma, 7, false);
            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_ID);
        }
    }
    if WorkModule::is_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
        WorkModule::set_int(boma, 720, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
        //Timer
        if lightweight_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
            WorkModule::sub_float(boma, 0.138, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
            if lightweight_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("sys_status_speed_up"), false, false);
            }
        }
        else {
            WorkModule::on_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
        }
        //Effects
        WorkModule::inc_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
        if lightweight_effect_timer > 25 {
            WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
        }
        if lightweight_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if lightweight_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("sys_status_speed_up"), false, false);
            EffectModule::req_follow(boma, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        UiManager::change_palutena_meter_color_green(entry_id);
        UiManager::set_palutena_meter_info(entry_id, lightweight_meter_timer, 100.0, 50.0);
    }
    if WorkModule::is_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
        WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
        //Timer
        if lightweight_burnout_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
            WorkModule::sub_float(boma, 0.138, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
            if lightweight_burnout_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("sys_status_down"), false, false);
            }
        }
        else {
            WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
            WorkModule::off_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
        }
        //Effects
        WorkModule::inc_int(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
        if lightweight_burnout_effect_timer > 25 {
            WorkModule::set_int(boma, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
        }
        if lightweight_burnout_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("sys_status_down"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if lightweight_burnout_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("sys_status_down"), false, false);
            EffectModule::req_follow(boma, Hash40::new("sys_status_down"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        UiManager::change_palutena_meter_color_purple(entry_id);
        UiManager::set_palutena_meter_info(entry_id, lightweight_burnout_meter_timer, 100.0, 50.0);
    }
    if !WorkModule::is_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) && !WorkModule::is_flag(boma, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        UiManager::change_palutena_meter_color_green(entry_id);
    }
    UiManager::set_palutena_meter_enable(entry_id, true);
    original!()(vtable, fighter)
}

//Palutena On Search
#[skyline::hook(offset = PALUTENA_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn palutena_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        let opponent_id = (*collision_log).opponent_battle_object_id;
        if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                let lua_module_fighter = get_fighter_common_from_accessor(&mut *boma);
                let opponent_boma = sv_battle_object::module_accessor(opponent_id as u32);
                let opponent_fighter_kind = smash::app::utility::get_kind(&mut *opponent_boma);
                let opponent_lua_module_fighter = get_fighter_common_from_accessor(&mut *opponent_boma);
                let opponent_get_whole = HitModule::get_whole(opponent_boma, 0);
                let opponent_status_kind = StatusModule::status_kind(opponent_boma);
                if ![*HIT_STATUS_INVINCIBLE, *HIT_STATUS_XLU, *HIT_STATUS_OFF].contains(&opponent_get_whole) || ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&opponent_status_kind) {
                    if PostureModule::lr(boma) != PostureModule::lr(opponent_boma) {
                        PostureModule::reverse_lr(opponent_boma);
                    }
                    match opponent_fighter_kind {
                        _ if opponent_fighter_kind == *FIGHTER_KIND_FOX => {
                            WorkModule::set_float(opponent_boma, 360.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_FIRE_DECIDE_STICK_X);
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_LUIGI => {
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_NESS => {
                            WorkModule::set_float(opponent_boma, 0.0, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_FLOAT_ANGLE);
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_FALCO => {
                            WorkModule::set_float(opponent_boma, 360.0, *FIGHTER_FALCO_INSTANCE_WORK_ID_FLOAT_FIRE_DECIDE_STICK_X);
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_METAKNIGHT => {
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_PLIZARDON => {
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_LUCAS => {
                            WorkModule::set_float(opponent_boma, 0.0, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_FLOAT_ANGLE);
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, false);
                        }
                        _ if opponent_fighter_kind == *FIGHTER_KIND_WOLF => {
                            WorkModule::set_float(opponent_boma, 360.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_FIRE_DECIDE_STICK_X);
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH, false);
                        }
                        _ => {
                            WorkModule::set_int(boma, opponent_id as i32, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_ID);
                            WorkModule::set_int(boma, 20, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_ANGELIC_MISSILE_TIMER);
                            DamageModule::add_damage(opponent_boma, 10.0, 0);
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DAMAGE_FLY, false);
                            MotionModule::change_motion(opponent_boma, Hash40::new("damage_fly_roll"), 0.0, 1.0, false, 0.0, false, false);
                            ModelModule::set_joint_rotate(opponent_boma, Hash40::new("rot"), &Vector3f{x: 270.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: 0 as u8}, MotionNodeRotateOrder{_address: 0 as u8});
                            macros::ATTACK(opponent_lua_module_fighter, 7, 0, Hash40::new("top"), WorkModule::get_param_float(opponent_boma, hash40("weight"), 0)/8.0, 0, 80, 0, 65, 9.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                            sv_kinetic_energy!(set_speed, opponent_lua_module_fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE, 1.0, 0.0);
                        }
                    }
                }
                HitModule::set_whole(boma, HitStatus(*HIT_STATUS_OFF), 0);
                search!(lua_module_fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
    skyline::install_hooks!(
        palutena_start_initialization,
        palutena_reset_initialization,
        palutena_death_initialization,
        palutena_opff,
        palutena_on_search
    );
}