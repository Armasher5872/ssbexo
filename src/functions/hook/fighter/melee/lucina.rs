use super::*;

const LUCINA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcd98a0; //Shared
const LUCINA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const LUCINA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcd99a0; //Shared
const LUCINA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d670; //Shared

//Lucina Startup Initialization
#[skyline::hook(offset = LUCINA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
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

//Lucina Reset Initialization
#[skyline::hook(offset = LUCINA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
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

//Lucina Death Initialization
#[skyline::hook(offset = LUCINA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
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
    }
    original!()(vtable, fighter)
}

//Lucina Once Per Fighter Frame
#[skyline::hook(offset = LUCINA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn lucina_opff(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let lr = PostureModule::lr(boma);
    let stick_x = ControlModule::get_stick_x(boma) * lr;
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let valuea = Vector3f{x: 0.0, y: 0.0, z: 2.0};
    let valueb = Vector3f{x: 0.0, y: 0.0, z: 5.0};
    let valuec = Vector3f{x: 0.0, y: 0.0, z: 8.0};
    let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
    let statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT].contains(&status_kind); 
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        LUCINA_GFX_COUNTER[entry_id] += 1;
        ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
        if LUCINA_GFX_COUNTER[entry_id] >= 6 && statuses {
            let effect_a = EffectModule::req_follow(boma, Hash40::new("sys_damage_elec"), Hash40::new("sword2"), &valuea, &valuea, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_b = EffectModule::req_follow(boma, Hash40::new("sys_damage_elec"), Hash40::new("sword2"), &valueb, &valueb, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            let effect_c = EffectModule::req_follow(boma, Hash40::new("sys_damage_elec"), Hash40::new("sword2"), &valuec, &valuec, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::set_rgb(boma, effect_a as u32, 0.0, 0.3, 1.0);
            EffectModule::set_rgb(boma, effect_b as u32, 0.0, 0.3, 1.0);
            EffectModule::set_rgb(boma, effect_c as u32, 0.0, 0.3, 1.0);
            LUCINA_GFX_COUNTER[entry_id] = 0;
        }
        else {
            EffectModule::kill_kind(boma, Hash40::new("sys_damage_elec"), false, true);
        }
        if motion_kind == hash40("attack_12") && frame >= 10.0 && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(boma, Hash40::new("attack_13"), 1.0, 1.0, false, 0.0, false, false);
        }
        if [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind) && MotionModule::end_frame(boma) - frame <= 2.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, true);
        }
        if situation_kind == *SITUATION_KIND_GROUND || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            USE_SWORDSMAN_DASH[entry_id] = true;
            USE_UP_SPECIAL[entry_id] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            //fighter.sub_transition_group_check_air_cliff();
            USE_SWORDSMAN_DASH[entry_id] = false;
            USE_UP_SPECIAL[entry_id] = false;
        }
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX && frame < 5.0 && stick_x < -0.2 {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
        if motion_kind == hash40("special_s1") && (12.0..=30.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                DID_ASTRA_2_S[entry_id] = true;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, true);
            }
        }
        if motion_kind == hash40("special_air_s1") && (12.0..=30.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_air_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_air_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                DID_ASTRA_2_S[entry_id] = true;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, true);
            }
        }
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2 && DID_ASTRA_2_S[entry_id] {
            DID_ASTRA_2_S[entry_id] = false;
            if situation_kind == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(boma, Hash40::new("special_s2_s"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_air_s2_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s4_s") && (10.0..=27.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s4_s") && (10.0..=27.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s4_hi") && (11.0..=28.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s4_hi") && (11.0..=28.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s4_lw") && (22.0..=39.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(boma, Hash40::new("special_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s4_lw") && (22.0..=39.0).contains(&frame) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.5 {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && DID_ASTRA_5_HI[entry_id] {
            DID_ASTRA_5_HI[entry_id] = false;
            if situation_kind == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(boma, Hash40::new("special_s5_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("special_air_s5_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s5_hi") && KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        };
        if motion_kind == hash40("special_hi_fall") {
            LANDING_HIT[entry_id] = true;
            if StatusModule::is_situation_changed(boma) && situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            }
            /*
            if stick_x >= 0.2 {
                macros::SET_SPEED_EX(fighter, 1.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else if stick_x <= -0.2 {
                macros::SET_SPEED_EX(fighter, -1.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else {
                macros::SET_SPEED_EX(fighter, 0.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            */
            if MotionModule::end_frame(boma) - frame <= 2.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        }
        if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) && frame > 28.0 {
            WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
}

pub fn install() {
	skyline::install_hooks!(
        lucina_start_initialization,
        lucina_reset_initialization,
        lucina_death_initialization,
        lucina_opff
    );
}