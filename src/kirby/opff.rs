use super::*;

unsafe extern "C" fn kirby_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let copy_chara = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    //Kirby Stuff
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_CATCH].contains(&status_kind) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
    }
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        ModelModule::set_mesh_visibility(boma, Hash40::new("kirby_armfoot"), true);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_STATUS_KIND_SPECIAL_HI, 
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4
    ].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) {
        macros::STOP_SE(fighter, Hash40::new("se_kirby_attack100"));
    }
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) {
        WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
        WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
    }
    //Dark Samus
    if copy_chara == *FIGHTER_KIND_SAMUSD
    && status_kind == *FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N
    && motion_kind == hash40("damage_n_2") {
        if StatusModule::is_situation_changed(boma) {
            if situation_kind != SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }
        if end_frame - frame <= 2.0 {
            if situation_kind != *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        if CHARGE_SHOT_TIMER[entry_id] > 0 {
            CHARGE_SHOT_TIMER[entry_id] -= 1;
        }
        if CHARGE_SHOT_TIMER[entry_id] <= 0
        && HAS_FIRE_CHARGE_SHOT[entry_id] == true {
            HAS_FIRE_CHARGE_SHOT[entry_id] = false;
            fighter.gimmick_flash();
        }
    }
    //Captain Falcon
    if copy_chara == *FIGHTER_KIND_CAPTAIN {
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)
        && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) == true {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, true);
        };
        if [*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) == true {
                FALCON_PUNCH_HIT[entry_id] = true;
            };
            if FALCON_PUNCH_HIT[entry_id] == true
            && (54.0..57.0).contains(&frame) {
                macros::PLAY_SE(fighter, Hash40::new("vc_kirby_cheer"));
            }
        };
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N
        && FALCON_PUNCH_HIT[entry_id] == true
        && frame > 70.0 {
            CancelModule::enable_cancel(boma);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN
        && FALCON_PUNCH_HIT[entry_id] == true
        && frame > 104.0 {
            CancelModule::enable_cancel(boma);
        }
        if ![*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            FALCON_PUNCH_HIT[entry_id] = false;
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN 
        && (25.0..40.0).contains(&frame)
        && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.5
        && KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] <= 15.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN {
            KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
        }
        if KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] == 0.0 {
            AttackModule::set_power_up(boma, 1.0);
        }
    }
    //Bowser
    if copy_chara == *FIGHTER_KIND_KOOPA {
        if motion_kind == hash40("koopa_special_n") {
            if CAN_FIREBALL[entry_id] == true {
                if end_frame - frame < 5.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                };
                if frame >= 19.0 {
                    CancelModule::enable_cancel(boma);
                };
                MotionModule::set_rate(boma, 0.775);
            }
            else {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("koopa_special_n_end"), 1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("koopa_special_air_n") {
            if CAN_FIREBALL[entry_id] == true {
                if end_frame-frame < 5.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                };
                if frame >= 19.0 {
                    CancelModule::enable_cancel(boma);
                };
                MotionModule::set_rate(boma, 0.775);
            }
            else {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("koopa_special_air_n_end"), 1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("koopa_special_n_end") {
            if CAN_FIREBALL[entry_id] == true {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
            else {
                if end_frame - frame < 5.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                };
            }
        }
        if motion_kind == hash40("koopa_special_air_n_end") {
            if CAN_FIREBALL[entry_id] == true {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
            else {
                if end_frame-frame < 5.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                };
            }
        }
        if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
            if CAN_FIREBALL[entry_id] == true {
                AttackModule::set_power_up(boma, 0.2);
                FIREBALL_GFX[entry_id] += 1;
            }
            else {
                AttackModule::set_power_up(boma, 1.0);
                FIREBALL_GFX[entry_id] = 0;
            };
        }
        if CAN_FIREBALL[entry_id] == true {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);
        }
    }
    //Sonic
    if copy_chara == *FIGHTER_KIND_SONIC {
        let rand_num_8 = smash::app::sv_math::rand(hash40("fighter"), 8);
        let kirby_new_animation_hash = Hash40::new(match rand_num_8 {1|2 => "sonic_special_n_hit", 3..=4 => "sonic_special_n_hit_1", 5..=6 => "sonic_special_n_hit_2", _ => "sonic_special_n_hit_3"});
        if motion_kind == hash40("sonic_special_n_homing") 
        && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) {
            HOMING_ATTACK_HIT[entry_id] = true;
        }
        if HOMING_ATTACK_HIT[entry_id] {
            if [hash40("sonic_special_n_hit"), hash40("sonic_special_n_hit_1"), hash40("sonic_special_n_hit_2"), hash40("sonic_special_n_hit_3")].contains(&motion_kind) {
                HOMING_ATTACK_HIT[entry_id] = false;
            };
            MotionModule::change_motion(boma, kirby_new_animation_hash, 1.0, 1.0, false, 0.0, false, false);
        };
    }
    //Lucario
    if copy_chara == *FIGHTER_KIND_LUCARIO {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        };
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW) {
            MotionModule::set_rate(boma, 1.65);
        }
        if MotionModule::end_frame(boma) - frame <= 2.0
        || CancelModule::is_enable_cancel(boma) {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
            MotionModule::set_rate(boma, 1.0);
        };
    }
    //Ryu
    if copy_chara == *FIGHTER_KIND_RYU {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        if [8, 30, 52, 74, 156, 179, 202, 225, 248, 271].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
        }
        if [9, 31, 53, 75, 157, 180, 203, 226, 249, 272].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
        }
        if [23, 45, 67, 89, 171, 194, 217, 240, 263, 286].contains(&fighter.magic_series()) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if [24, 46, 68, 90, 172, 195, 218, 241, 264, 287].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if [25, 47, 69, 91, 173, 196, 219, 242, 265, 288].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if [26, 48, 70, 92, 174, 197, 220, 243, 266, 289].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
    //Ken
    if copy_chara == *FIGHTER_KIND_KEN {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
        if [9, 31, 53, 75, 157, 180, 203, 226, 249, 272].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
        }
        if [23, 45, 67, 89, 171, 194, 217, 240, 263, 286].contains(&fighter.magic_series()) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if [24, 46, 68, 90, 172, 195, 218, 241, 264, 287].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if [25, 47, 69, 91, 173, 196, 219, 242, 265, 288].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if [26, 48, 70, 92, 174, 197, 220, 243, 266, 289].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
    //Terry
    if copy_chara == *FIGHTER_KIND_DOLLY {
        if [23, 45, 67, 89, 194, 263, 286].contains(&fighter.magic_series()) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if [24, 46, 68, 90, 195, 264, 287].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if [25, 47, 69, 91, 196, 265, 288].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if [26, 48, 70, 92, 197, 266, 289].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
}

pub fn install() {
    Agent::new("kirby")
    .on_line(Main, kirby_frame)
    .install()
    ;
}