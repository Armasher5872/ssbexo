use super::*;

const NESS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xdefdf0; //Ness only
const NESS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const NESS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xdf0050; //Ness only
const NESS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xdf0360; //Ness only

//Ness Startup Initialization
#[skyline::hook(offset = NESS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    original!()(vtable, fighter)
}

//Ness Reset Initialization
#[skyline::hook(offset = NESS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_NESS as u32 {
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
    }
}

//Ness Death Initialization
#[skyline::hook(offset = NESS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_death_initialization(param_1: u64, param_2: u64, param_3: u64, param_4: u64, vtable: u64, fighter: &mut Fighter) -> u64 {
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
    original!()(param_1, param_2, param_3, param_4, vtable, fighter)
}

//Ness Once Per Fighter Frame
#[skyline::hook(offset = NESS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ness_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let sticky = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
    //Jump Cancel Down Throw
    if status_kind == *FIGHTER_STATUS_KIND_THROW && motion_kind == hash40("throw_lw") && (39.0..=45.0).contains(&frame) && fighter.battle_object.check_jump_cancel(false, false) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
    };
    //Shield Special
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END].contains(&status_kind)  && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
        if OFFENSE_UP_ACTIVE[entry_id] {
            MotionModule::change_motion(boma, Hash40::new("special_shield"), 107.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    if motion_kind == hash40("special_shield") {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        if frame > 120.0 && !OFFENSE_UP_ACTIVE[entry_id] {
            OFFENSE_UP_ACTIVE[entry_id] = true;
            OFFENSE_UP_TIMER[entry_id] = 336;
        }
        if frame > 157.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        };
    }
    if OFFENSE_UP_ACTIVE[entry_id] {
        //Graphics
        OFFENSE_UP_GFX_COUNTER[entry_id] += 1;
        if OFFENSE_UP_GFX_COUNTER[entry_id] >= 8 {
            ColorBlendModule::set_main_color(boma, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
            EffectModule::req_follow(boma, Hash40::new("ness_black_face"), Hash40::new("head"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::req_follow(boma, Hash40::new("ness_psi_atk"), Hash40::new("handr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::req_follow(boma, Hash40::new("ness_psi_atk"), Hash40::new("handl"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.2, false, 0, 0, 0, 0, 0, false, false);
            OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
        }
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_attack"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_blink"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_catch"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_cliff"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_down"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_downeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_escape"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_eye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_facen"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_final"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_furafura"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_furafuraeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_halfblink1"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_halfblink2"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_head"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_heavyattack"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_heavyattackeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_heavyouch2eye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_hot"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_ouch"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_oucheye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_steppose"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_stepposeeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_talk"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_patterna"), true);
        //Duration
        if OFFENSE_UP_TIMER[entry_id] > 0 {
            OFFENSE_UP_TIMER[entry_id] -= 1;
        }
        if OFFENSE_UP_TIMER[entry_id] <= 0 {
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_eye"), true);
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_facen"), true);
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_head"), true);
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_patterna"), false);
            EffectModule::kill_kind(boma, Hash40::new("ness_black_face"), false, false);
            OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
            OFFENSE_UP_ACTIVE[entry_id] = false;
        }
        //Mechanics
        AttackModule::set_power_up(boma, 1.15);
        DamageModule::set_damage_mul(boma, 1.15);
        DamageModule::set_reaction_mul(boma, 1.15);
    }
    else {
        AttackModule::set_power_up(boma, 1.0);
        DamageModule::set_damage_mul(boma, 1.0);
        DamageModule::set_reaction_mul(boma, 1.0);
        ColorBlendModule::cancel_main_color(boma, 0);
    }
    //Neutral Special
    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD {
        if PK_FLASH_TIMER[entry_id] >= 0 && PK_FLASH_TIMER[entry_id] < 120 {
            PK_FLASH_TIMER[entry_id] += 1;
        }
        if PK_FLASH_TIMER[entry_id] > 120 {
            PK_FLASH_TIMER[entry_id] -= 1;
        }
    }
    if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
        PK_FLASH_TIMER[entry_id] = 0;
        EffectModule::kill_kind(boma, Hash40::new("ness_pkfl_bomb_max"), false, false);
        EffectModule::kill_kind(boma, Hash40::new("ness_pkfl_hold"), false, false);
    }
    //Fast Fall PK Fire
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && situation_kind == *SITUATION_KIND_AIR && fighter.battle_object.is_cat_flag(Cat2::FallJump) && sticky < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
        WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
    };
    //Up Special Land Cancel
    if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN].contains(&status_kind) {
        //fighter.sub_transition_group_check_air_cliff();
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if StatusModule::is_situation_changed(boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ness_start_initialization,
        ness_reset_initialization,
        ness_death_initialization,
        ness_opff
    );
}