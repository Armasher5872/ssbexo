use super::*;

const LUCAS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc73040; //Lucas only
const LUCAS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const LUCAS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc732c0; //Lucas only
const LUCAS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc73570; //Lucas only
const LUCAS_VTABLE_LINK_EVENT_OFFSET: usize = 0xc73e00; //Lucas only
const LUCAS_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET: usize = 0x68d8c0; //Shared
const LUCAS_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET: usize = 0x68d8d0; //Shared
const LUCAS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET: usize = 0xc73640; //Lucas only

//Lucas Startup Initialization
#[skyline::hook(offset = LUCAS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let shield_data = ShieldDataResource::new(0.0, 6.5, 0.0, 0.0, 6.5, 11.5, 13.0, Hash40::new("hip"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
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
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN);
    WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
    add_shield_group(boma, resource, FIGHTER_LUCAS_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    original!()(vtable, fighter)
}

//Lucas Reset Initialization
#[skyline::hook(offset = LUCAS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
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
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
        WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
        WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
        WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
        WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN);
        WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
    }
    original!()(vtable, fighter)
}

//Lucas Death Initialization
#[skyline::hook(offset = LUCAS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
    WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN);
    WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
    original!()(vtable, fighter)
}

//Lucas Once Per Fighter Frame
#[skyline::hook(offset = LUCAS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn lucas_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let defense_up_timer = WorkModule::get_int(boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    let defense_up_effect_timer = WorkModule::get_int(boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
    if WorkModule::is_flag(boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        //Timer Mechanics
        if defense_up_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
            if defense_up_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("lucas_pkfr_hold"), false, false);
                EffectModule::kill_kind(boma, Hash40::new("sys_status_defense_up"), false, false);
            }
        }
        else {
            WorkModule::set_flag(boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
            WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
        }
        //Effect Mechanics
        WorkModule::inc_int(boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
        if defense_up_effect_timer > 25 {
            WorkModule::set_int(boma, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
        }
        if defense_up_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &NONE_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &NONE_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &NONE_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if defense_up_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("lucas_pkfr_hold"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_defense_up"), false, false);
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &NONE_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &NONE_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &NONE_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        //Game Mechanics
        DamageModule::set_damage_mul(boma, 0.9);
    }
    original!()(vtable, fighter)
}

//Lucas Link Event
#[skyline::hook(offset = LUCAS_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn lucas_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        let boma = fighter.battle_object.module_accessor;
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if StatusModule::status_kind(boma) == FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH && capture_event.status == *FIGHTER_STATUS_KIND_THROWN {
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            fighter.battle_object.change_status_req(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_THROW, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Lucas Shield Attack Detection Event
#[skyline::hook(offset = LUCAS_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET)]
unsafe extern "C" fn lucas_shield_attack_detection_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            WorkModule::set_flag(boma, true, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
        }
    }
    original!()(vtable, fighter, unk)
}

//Lucas Shield Attack Transition Event
#[skyline::hook(offset = LUCAS_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET)]
unsafe extern "C" fn lucas_shield_attack_transition_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
        if WorkModule::is_flag(boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED) {
            StatusModule::change_status_request_from_script(boma, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, false);
        }
    }
    original!()(vtable, fighter, unk)
}

//Lucas Reflect Attack Event
#[skyline::hook(offset = LUCAS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET)]
unsafe extern "C" fn lucas_reflect_attack_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
        WorkModule::set_flag(boma, true, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);   
    }
    original!()(vtable, fighter, unk)
}

pub fn install() {
	skyline::install_hooks!(
        lucas_start_initialization,
        lucas_reset_initialization,
        lucas_death_initialization,
        lucas_opff,
        lucas_link_event,
        lucas_shield_attack_detection_event,
        lucas_shield_attack_transition_event,
        lucas_reflect_attack_event
    );
}