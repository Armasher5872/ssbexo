/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Dash Back Main
pub unsafe extern "C" fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Due to the fighting game characters having a different status for their dashes, this needs to be be implemented alongside the normal dash status script to allow for the new features to apply for them
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash_b"), 0.0, 1.0, false, 0.0, false, false);
    fighter.status_DashCommon();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    GroundModule::set_reverse_direction(fighter.module_accessor, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_dashback_main_loop as *const () as _))
}

//Dash Back Loop
unsafe extern "C" fn fgc_dashback_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let const_stick_x = fighter.global_table[STICK_X].get_f32(); 
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let flick_y = fighter.global_table[FLICK_Y].get_i32();
    let get_down_friction = GroundModule::get_down_friction(boma);
    let get_pickable_item_size = ItemModule::get_pickable_item_size(boma);
    let get_shoot_item_bullet = ItemModule::get_shoot_item_bullet(boma, 0);
    let is_have_item = ItemModule::is_have_item(boma, 0);
    let frame = MotionModule::frame(boma);
    let heavy_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let light_item = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let item_lua_stack_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    let item_lua_stack_swing = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let item_lua_stack_shoot = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT); sv_module_access::item(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    let lr = PostureModule::lr(boma);
    let dash_flag_no_s4 = WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_S4);
    let run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("run_stick_x"));
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_flick_y")) as i32;
    let re_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("re_dash_frame")) as f32;
    let dash_enable_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    let stick_x = const_stick_x*lr;
    let foxtrot_check = re_dash_frame <= frame;
    let run_stick_x_check = run_stick_x <= (stick_x*-1.0);
    let mut throw = false;
    let kind;
    if CancelModule::is_enable_cancel(boma) && fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
    || fighter.sub_transition_group_check_special_command().get_bool()
    || fighter.sub_transition_group_check_ground_special().get_bool()
    || fighter.sub_transition_specialflag_hoist().get_bool()
    || fighter.sub_transition_group_check_ground_attack().get_bool()
    || fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 && foxtrot_check {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) 
    && run_stick_x_check 
    && cmd_cat1 & (
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | 
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
    ) == 0 {
        if fighter_kind == *FIGHTER_KIND_DOLLY {
            kind = FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK;
        }
        else if fighter_kind == *FIGHTER_KIND_DEMON {
            kind = FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK;
        }
        else  {
            kind = FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK;
        }
        fighter.change_status(kind.into(), false.into());
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), 1.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return 1.into();
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if !WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return 1.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) {
        if fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
            return 1.into();
        }
    }
    if GroundModule::is_passable_ground(boma) && stick_y < pass_stick_y && flick_y < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 1.into();
    }
    if (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0)) 
    && notify_taunt_hash {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0 && !dash_flag_no_s4 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return 1.into();
    }
    if 0 < dash_enable_attack_frame && (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 || FighterUtil::is_valid_auto_catch_item(boma, false)) {
        if cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 
        && heavy_item && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH) && !is_have_item {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return 1.into();
        }
        if get_pickable_item_size == *ITEM_SIZE_LIGHT as u64 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH) && light_item {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        if item_lua_stack_throw {
            throw = pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        if item_lua_stack_shoot {
            if get_shoot_item_bullet <= 0 {
                throw = true;
            }
            else {
                throw = false;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE) && is_have_item && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0  && item_lua_stack_no_throw {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) && item_lua_stack_swing && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0 && !dash_flag_no_s4 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) && item_lua_stack_shoot && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 && !dash_flag_no_s4 {
        if get_shoot_item_bullet <= 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(boma) {
        if get_down_friction < 1.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}