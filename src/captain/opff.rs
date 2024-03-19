use super::*;

unsafe extern "C" fn captain_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let parry_timer = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    let parried = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    let falcon_punch_turn_count = WorkModule::get_int(boma, FIGHTER_FALCON_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
    //Parry Voice
    if [hash40("just_shield_off"), hash40("just_shield")].contains(&motion_kind) {
        if (0.0..5.0).contains(&frame) {
            WorkModule::set_int(boma, 1, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
            WorkModule::set_int(boma, 60, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        }
    }
    if parry_timer > 0 {
        WorkModule::dec_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    }
    if parry_timer == 30 {
        macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal02"));
    }
    if parry_timer <= 0
    && parried == 1 {
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    }
    //Dash Grab
    if status_kind == *FIGHTER_STATUS_KIND_DASH {
        WorkModule::set_float(boma, 1.933, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    }
    if status_kind == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::set_float(boma, 2.338, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    }
    if motion_kind == hash40("catch_pull")
    && status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH_PULL {
        macros::SET_SPEED_EX(fighter, WorkModule::get_float(boma, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED)/1.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    //Neutral Special
    if [23, 45, 67, 89, 114, 133, 152, 171, 194, 217, 240, 263, 286].contains(&fighter.magic_series())
    || ([*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) == true) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::set_flag(boma, true, FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
        };
        if WorkModule::is_flag(boma, FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
        && (54.0..57.0).contains(&frame) {
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_cheer"));
        }
    };
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
    && WorkModule::is_flag(boma, FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
    && frame > 70.0 {
        CancelModule::enable_cancel(boma);
    }
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN
    && WorkModule::is_flag(boma, FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
    && frame > 104.0 {
        CancelModule::enable_cancel(boma);
    }
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN 
    && (25.0..40.0).contains(&frame)
    && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.5
    && falcon_punch_turn_count <= 15 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
    };
    if status_kind != *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN {
        WorkModule::set_int(boma, 0, FIGHTER_FALCON_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
    };
    if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        WorkModule::set_flag(boma, false, FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
    }
    //Down Special
    if motion_kind == hash40("special_air_lw") {
        let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
        if ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_LEFT as u64) || ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_RIGHT as u64) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }
}

unsafe extern "C" fn captain_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(if_shield_special as *const () as _));
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

pub fn install() {
    Agent::new("captain")
    .on_start(captain_init)
    .on_line(Main, captain_frame)
    .install()
    ;
}