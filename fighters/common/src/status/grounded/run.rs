/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
#![allow(unused_assignments)] //Addresses warning: value assigned to `sticky_run` is never read
use super::*;

//Run Sub
#[skyline::hook(replace = L2CFighterCommon_status_Run_Sub)]
unsafe extern "C" fn status_run_sub(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let start_frame = if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {WorkModule::get_float(boma, *FIGHTER_STATUS_RUN_WORK_FLOAT_START_FRAME)} else {0.0};
    MotionModule::change_motion(boma, Hash40::new("run"), start_frame, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    /* The transition terms from ITEM_THROW_DASH to TURN_RUN are vanilla, and anything past that is modded */
    let transition_terms = [ 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
    ];
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(boma, *term);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE);
    }
}

//Run Main
#[skyline::hook(replace = L2CFighterCommon_status_Run_Main)]
unsafe extern "C" fn status_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let const_stick_x = fighter.global_table[STICK_X].get_f32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32(); //New
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let is_have_item = ItemModule::is_have_item(boma, 0);
    let get_correct = GroundModule::get_correct(boma);
    let run_frame = WorkModule::get_float(boma, *FIGHTER_STATUS_RUN_WORK_FLOAT_RUN_FRAME);
    let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
    let stick_x = const_stick_x*lr;
    let pad_attack = pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    let throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let stick_direction = get_command_stick_direction(&mut *boma, 0.5);
    let valid_sticky_run_bounds = stick_direction == 1 || stick_direction == 3 || stick_direction == 7 || stick_direction == 9; //New
    if fighter.global_table[RUN_MAIN_UNIQ].get_bool() 
    && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[RUN_MAIN_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
        fighter.change_status(FIGHTER_STATUS_KIND_HAMMER_WALK.into(), false.into());
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 0.into();
    }
    let item_throw_threshold = WorkModule::get_param_float(boma, hash40("common"), 0x206138766c);
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        if pad_attack {
            if stick_x < item_throw_threshold {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
                sv_module_access::item(fighter.lua_state_agent);
                let mut item_throw = fighter.pop_lua_stack(1).get_bool();
                if !item_throw {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
                    sv_module_access::item(fighter.lua_state_agent);
                    if fighter.pop_lua_stack(1).get_bool() {
                        item_throw = ItemModule::get_shoot_item_bullet(boma, 0) <= 0;
                    }
                }
                if item_throw {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) {
        if throw {
            if pad_attack {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), true.into());
                return 0.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH) {
        if is_have_item {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
                sv_module_access::item(fighter.lua_state_agent); 
                let no_throw = fighter.pop_lua_stack(1).get_bool();
                if !no_throw {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), true.into());
                    return 0.into();
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        if run_frame <= 0.0 {
            if stick_x <= turn_run_stick_x {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
                    if !is_have_item {
                        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
                        return 0.into();
                    }
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
            if !is_have_item {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_ground_guard().get_bool() {
        return 0.into();
    }
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) {
        fighter.clear_lua_stack(); 
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING); 
        sv_module_access::item(fighter.lua_state_agent); 
        let swing = fighter.pop_lua_stack(1).get_bool();
        if swing {
            if pad_attack {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return 1.into();
    }
    if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() 
    && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 0.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
            return 0.into();
        }
    }
    /*Start of New*/
    if fighter.global_table[CHECK_ATTACK_LW4_UNIQ].get_bool() 
    && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_LW4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 0.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
            return 0.into();
        }
    }
    if fighter.global_table[CHECK_ATTACK_3_UNIQ].get_bool() 
    && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_3_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 0.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) {
            if !throw {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
                return 0.into();
            }
        }
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) {
            if !throw {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
                return 0.into();
            }
        }
    }
    /*End of New*/
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) {
        if pad_attack {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
            return 0.into();
        }
    }
    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN) {
        if run_frame <= 0.0 {
            if stick_x <= turn_run_stick_x {
                fighter.change_status(FIGHTER_STATUS_KIND_TURN_RUN.into(), true.into());
                return 0.into();
            }
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE) {
            let walk_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("walk_stick_x"));
            if const_stick_x.abs() < walk_stick_x {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), true.into());
                return 0.into();
            }
        }
    }
    else {
        let run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("run_stick_x"));
        if const_stick_x.abs() < run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), true.into());
            return 0.into();
        }
    }
    /*Start of New*/
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    if GroundModule::is_passable_ground(boma) && fighter.global_table[STICK_Y].get_f32()< pass_stick_y && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 0.into();
    }
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0)) 
    && notify_taunt_hash {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 0.into();
    }
    if GroundModule::is_ottotto(boma, 1.5) {
        if valid_sticky_run_bounds {
            if get_correct != *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP {
                GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
        else {
            if get_correct != *GROUND_CORRECT_KIND_GROUND_OTTOTTO {
                GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_OTTOTTO));
            }
        }
    }
    /*End of New*/
    fighter.sub_ground_check_stop_wall();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_run_sub,
            status_run_main
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}