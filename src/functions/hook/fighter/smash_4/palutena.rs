use super::*;

const PALUTENA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe5e6a0; //Palutena only
const PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe5f350; //Palutena only
const PALUTENA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xe5f3e0; //Palutena only

//Palutena Startup Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let u32_entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
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
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    UiManager::set_palutena_meter_info(u32_entry_id, 100.0, 100.0, 50.0);
    original!()(vtable, fighter)
}

//Palutena Reset Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let u32_entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
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
        WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
        WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
        UiManager::set_palutena_meter_info(u32_entry_id, 100.0, 100.0, 50.0);
    }
    original!()(vtable, fighter)
}

//Palutena Death Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let u32_entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
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
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    WorkModule::set_float(boma, 100.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    UiManager::set_palutena_meter_info(u32_entry_id, 100.0, 100.0, 50.0);
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
        UiManager::set_palutena_meter_info(entry_id, 100.0, 100.0, 50.0);
    }
    UiManager::set_palutena_meter_enable(entry_id, true);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        palutena_start_initialization,
        palutena_reset_initialization,
        palutena_death_initialization,
        palutena_opff
    );
}