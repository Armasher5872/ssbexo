use super::*;

const KOOPA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const KOOPA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xbc1dd0; //Bowser only
const KOOPA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xbc1e00; //Bowser only
const KOOPA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xbc2290; //Bowser only

//Bowser Startup Initialization
#[skyline::hook(offset = KOOPA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_KOOPA as u32 {
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
    }
    original!()(vtable, fighter)
}

//Bowser Reset Initialization
#[skyline::hook(offset = KOOPA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    original!()(vtable, fighter)
}

//Bowser Death Initialization
#[skyline::hook(offset = KOOPA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    original!()(vtable, fighter)
}

//Bowser Once Per Fighter Frame
#[skyline::hook(offset = KOOPA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn koopa_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let lr = PostureModule::lr(boma);
    let special_zoom_gfx = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) {
        KOOPA_OK_SMASH[entry_id] = false;
        KOOPA_OK_SMASH_GFX[entry_id] = 0;
        KOOPA_GOOD_SMASH[entry_id] = false;
        KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
        KOOPA_GREAT_SMASH[entry_id] = false;
        KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
        KOOPA_EXCELLENT_SMASH[entry_id] = false;
        KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
        if (0.0..=19.0).contains(&frame) {
            KOOPA_OK_SMASH[entry_id] = true;
        }
        else if (19.0..=37.0).contains(&frame) {
            KOOPA_GOOD_SMASH[entry_id] = true;
            KOOPA_OK_SMASH[entry_id] = false;
        }
        else if (37.0..=54.0).contains(&frame) {
            KOOPA_GREAT_SMASH[entry_id] = true;
            KOOPA_GOOD_SMASH[entry_id] = false;
        }
        else if (54.0..=58.0).contains(&frame) {
            KOOPA_EXCELLENT_SMASH[entry_id] = true;
            KOOPA_GREAT_SMASH[entry_id] = false;
        }
        else {
            KOOPA_OK_SMASH[entry_id] = true;
            KOOPA_GOOD_SMASH[entry_id] = false;
            KOOPA_GREAT_SMASH[entry_id] = false;
            KOOPA_EXCELLENT_SMASH[entry_id] = false;
        }
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind)
    && KOOPA_EXCELLENT_SMASH[entry_id]
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        if special_zoom_gfx < 2 {
            SlowModule::set_whole(boma, 8, 80);
            CameraModule::zoom_in(boma, 2, 0, 1.8, &Vector2f{x: 0.0, y: 0.0}, false);
            EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            SoundModule::play_se(boma, Hash40::new("se_common_criticalhitr"), true, false, false, false, enSEType(0));
            CameraModule::req_quake(boma, *CAMERA_QUAKE_KIND_XL);
        }
        if special_zoom_gfx >= 4 {
            SlowModule::clear_whole(boma);
            CameraModule::reset_all(boma);
            EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
            CameraModule::zoom_out(boma, 0);
        }
    }
    if KOOPA_OK_SMASH[entry_id] == true {
        KOOPA_OK_SMASH_GFX[entry_id] += 1;
        if KOOPA_OK_SMASH_GFX[entry_id] == 6 {
            let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let hand_aura = EffectModule::req_follow(boma, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            let hand_bullet = EffectModule::req_follow(boma, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_rgb(boma, waist_aura as u32, 1.0, 0.0, 0.0);
            EffectModule::set_rgb(boma, bust_aura as u32, 1.0, 0.0, 0.0);
            EffectModule::set_rgb(boma, hand_aura as u32, 1.0, 0.0, 0.0);
            EffectModule::set_rgb(boma, hand_bullet as u32, 1.0, 0.0, 0.0);
            KOOPA_OK_SMASH_GFX[entry_id] = 0;
        }
    }
    else if KOOPA_GOOD_SMASH[entry_id] == true {
        KOOPA_GOOD_SMASH_GFX[entry_id] += 1;
        KOOPA_OK_SMASH_GFX[entry_id] = 0;
        if KOOPA_GOOD_SMASH_GFX[entry_id] == 6 {
            let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let hand_aura = EffectModule::req_follow(boma, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            let hand_bullet = EffectModule::req_follow(boma, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_rgb(boma, waist_aura as u32, 1.0, 1.0, 0.0);
            EffectModule::set_rgb(boma, bust_aura as u32, 1.0, 1.0, 0.0);
            EffectModule::set_rgb(boma, hand_aura as u32, 1.0, 1.0, 0.0);
            EffectModule::set_rgb(boma, hand_bullet as u32, 1.0, 1.0, 0.0);
            KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
        }
    }
    else if KOOPA_GREAT_SMASH[entry_id] == true {
        KOOPA_GREAT_SMASH_GFX[entry_id] += 1;
        KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
        if KOOPA_GREAT_SMASH_GFX[entry_id] == 6 {
            let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let hand_aura = EffectModule::req_follow(boma, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            let hand_bullet = EffectModule::req_follow(boma, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_rgb(boma, waist_aura as u32, 0.0, 1.0, 0.0);
            EffectModule::set_rgb(boma, bust_aura as u32, 0.0, 1.0, 0.0);
            EffectModule::set_rgb(boma, hand_aura as u32, 0.0, 1.0, 0.0);
            EffectModule::set_rgb(boma, hand_bullet as u32, 0.0, 1.0, 0.0);
            KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
        }
    }
    else if KOOPA_EXCELLENT_SMASH[entry_id] == true {
        KOOPA_EXCELLENT_SMASH_GFX[entry_id] += 1;
        KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
        if KOOPA_EXCELLENT_SMASH_GFX[entry_id] == 3 {
            let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let hand_aura = EffectModule::req_follow(boma, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            let hand_bullet = EffectModule::req_follow(boma, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_rgb(boma, waist_aura as u32, 0.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, bust_aura as u32, 0.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, hand_aura as u32, 0.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, hand_bullet as u32, 0.0, 0.0, 1.0);
            KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
        }
    }
    else {
        EffectModule::kill_kind(boma, Hash40::new("sys_aura_light"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("sys_revenge_aura"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("sys_revenge_bullet"), false, true);
        KOOPA_OK_SMASH[entry_id] = false;
        KOOPA_OK_SMASH_GFX[entry_id] = 0;
        KOOPA_GOOD_SMASH[entry_id] = false;
        KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
        KOOPA_GREAT_SMASH[entry_id] = false;
        KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
        KOOPA_EXCELLENT_SMASH[entry_id] = false;
        KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
    };
    if motion_kind == hash40("attack_air_lw")
    && lr <= 0.0 {
        PostureModule::set_lr(boma, 1.0);
        PostureModule::update_rot_y_lr(boma);
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
    && [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
        || FIREBALL_TIMER[entry_id] > 0 {
            CAN_FIREBALL[entry_id] = false;
        }
        else {
            CAN_FIREBALL[entry_id] = true;
        }
    }
    if motion_kind == hash40("special_n") {
        if CAN_FIREBALL[entry_id] == true {
            if end_frame - frame < 5.0 {
                FIREBALL_TIMER[entry_id] = 180;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            };
            if frame >= 19.0 {
                FIREBALL_TIMER[entry_id] = 180;
                CancelModule::enable_cancel(boma);
            };
            MotionModule::set_rate(boma, 0.775);
        }
        else {
            if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40::new("special_n_end"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if motion_kind == hash40("special_air_n") {
        if CAN_FIREBALL[entry_id] == true {
            if end_frame-frame < 5.0 {
                FIREBALL_TIMER[entry_id] = 180;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            };
            if frame >= 19.0 {
                FIREBALL_TIMER[entry_id] = 180;
                CancelModule::enable_cancel(boma);
            };
            MotionModule::set_rate(boma, 0.775);
        }
        else {
            if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40::new("special_air_n_end"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if motion_kind == hash40("special_n_end") {
        if CAN_FIREBALL[entry_id] == true {
            FIREBALL_TIMER[entry_id] = 180;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
        else {
            if end_frame - frame < 5.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            };
        }
    }
    if motion_kind == hash40("special_air_n_end") {
        if CAN_FIREBALL[entry_id] == true {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }
        else {
            if end_frame-frame < 5.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            };
        }
    }
    if FIREBALL_TIMER[entry_id] > 0 {
        FIREBALL_TIMER[entry_id] -= 1;
    }
    if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
        if CAN_FIREBALL[entry_id] == true {
            FIREBALL_GFX[entry_id] += 1;
        }
        else {
            FIREBALL_GFX[entry_id] = 0;
        };
    }
    if CAN_FIREBALL[entry_id] == true {
        EffectModule::kill_kind(boma, Hash40::new("koopa_breath_m_fire"), false, true);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A].contains(&status_kind) && (StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI | *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G) {
        //fighter.battle_object.sub_transition_group_check_air_cliff();
        //fighter.battle_object.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        fighter.battle_object.set_back_cliff_hangdata(20.0, 10.0);
        fighter.battle_object.set_front_cliff_hangdata(20.0, 10.0);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        koopa_start_initialization,
        koopa_reset_initialization,
        koopa_death_initialization,
        koopa_opff
    );
}