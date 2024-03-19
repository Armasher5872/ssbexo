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
    let kirby_falcon_punch_turn_count = WorkModule::get_int(boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
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
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP {
        let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_INHALE_OBJECT_ID) as u32;
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
        if [23, 45, 67, 89, 114, 133, 152, 171, 194, 217, 240, 263, 286].contains(&fighter.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, true);
        };
        if [*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                WorkModule::set_flag(boma, true, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
            };
            if WorkModule::is_flag(boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
            && (54.0..57.0).contains(&frame) {
                macros::PLAY_SE(fighter, Hash40::new("vc_kirby_cheer"));
            }
        };
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N
        && WorkModule::is_flag(boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
        && frame > 70.0 {
            CancelModule::enable_cancel(boma);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN
        && WorkModule::is_flag(boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
        && frame > 104.0 {
            CancelModule::enable_cancel(boma);
        }
        if ![*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN 
        && (25.0..40.0).contains(&frame)
        && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.5
        && kirby_falcon_punch_turn_count <= 15 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN {
            WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
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

unsafe extern "C" fn kirby_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let team_no = TeamModule::team_no(boma) as i32;
    //Universal
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
    //Kirby
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
    WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
    WorkModule::set_float(boma, 0.0, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    WHEEL_SPEED_UP[entry_id] = 0.0;
    WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
    WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
    WorkModule::set_int(boma, team_no, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO);
}

pub fn install() {
    Agent::new("kirby")
    .on_start(kirby_init)
    .on_line(Main, kirby_frame)
    .install()
    ;
}