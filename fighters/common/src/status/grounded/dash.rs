use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Dash)]
unsafe extern "C" fn status_pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.status_pre_DashCommon();
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_DASH, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, true, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, 0, (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, 0, 0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_DashCommon)]
unsafe extern "C" fn status_dashcommon(fighter: &mut L2CFighterCommon) {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let dash_enable_attack_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("dash_enable_attack_frame"));
    let retry_turn_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("retry_turn_dash_frame"));
    let run_brake_attack_escape_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("run_brake_attack_escape_frame"));
    let turn_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("turn_dash_frame"));
    let enable_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
    ]; //Transition terms ATTACK_DASH through ESCAPE_B are vanilla. Everything after is added
    let unable_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
    ];
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
    for x in enable_terms.iter() {
        WorkModule::enable_transition_term(boma, *x);
    }
    WorkModule::set_int(boma, turn_dash_frame, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    WorkModule::set_int(boma, retry_turn_dash_frame, *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    WorkModule::set_int(boma, dash_enable_attack_frame, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if prev_status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
        WorkModule::set_int(boma, run_brake_attack_escape_frame-fighter.global_table[UNK11].get_i32(), *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if 0 < WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME) {
            for y in unable_terms.iter() {
                WorkModule::unable_transition_term(boma, *y);
            }
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Dash_Main_common)]
unsafe extern "C" fn status_dash_main_common(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let is_have_item = ItemModule::is_have_item(boma, 0);
    let lr = PostureModule::lr(boma);
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); sv_battle_object::notify_event_msc_cmd(lua_state); fighter.pop_lua_stack(1).get_bool()};
    if fighter.global_table[DASH_COMMON_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_COMMON_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }
    let mut can_s4 = true;
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        can_s4 = stick_x*lr < WorkModule::get_param_float(boma, hash40("common"), 0x206138766c)
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && can_s4 {
        let mut throw = false;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(lua_state);
        if fighter.pop_lua_stack(1).get_bool() {
            throw = pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(lua_state);
        if fighter.pop_lua_stack(1).get_bool() {
            let item_bullet = ItemModule::get_shoot_item_bullet(boma, 0);
            if item_bullet <= 0 {
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
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(lua_state);
        fighter.pop_lua_stack(1).get_bool() 
    } && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && is_have_item
    && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(lua_state);
        !fighter.pop_lua_stack(1).get_bool()
     } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH)
    && is_have_item
    && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(lua_state);
        !fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) && {
        let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
        stick_x*lr <= turn_run_stick_x
    } && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && !is_have_item {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
    && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && !is_have_item {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), false.into());
                return 1.into();
            }
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if !WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_special_command().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(lua_state);
        fighter.pop_lua_stack(1).get_bool()
    } && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(lua_state);
        fighter.pop_lua_stack(1).get_bool()
    } && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(boma, 0) > 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
            return 1.into();
        }
    }
    if fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(lua_state);
        fighter.pop_lua_stack(1).get_bool()
    } && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) {
        if pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return 1.into();
    }
    if 0 < WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME)
    && (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || FighterUtil::is_valid_auto_catch_item(boma, false)) {
        if cmd_cat1 & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(lua_state);
            fighter.pop_lua_stack(1).get_bool()
        }
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
        && !is_have_item {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
        if ItemModule::get_pickable_item_size(boma) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
            sv_module_access::item(lua_state);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) 
    && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return 1.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 && {
        let frame = MotionModule::frame(boma);
        let re_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("re_dash_frame")) as f32;
        re_dash_frame <= frame
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        return 1.into();
    }
    if GroundModule::is_passable_ground(boma) && fighter.global_table[STICK_Y].get_f32() < pass_stick_y && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 0.into();
    }
    if (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0)) 
    && notify_taunt_hash {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 0.into();
    }
    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 1.into();
    }
    if param_2.get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_2.get_ptr());
        if callable(fighter).get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) && {
        let run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("run_stick_x"));
        run_stick_x <= stick_x * lr
    } {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN) {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), true.into());
        }
        return 1.into();
    }
    if GroundModule::get_down_friction(boma) < 1.0
    && FighterMotionModuleImpl::is_valid_cancel_frame(boma, -1, true) {
        fighter.change_status(FIGHTER_STATUS_KIND_WALK_BRAKE.into(), false.into());
        return 1.into();
    }
    if !MotionModule::is_end(boma) {
        if fighter.sub_ground_check_stop_wall().get_bool() {
            return 1.into();
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_dash,
            status_dashcommon,
            status_dash_main_common
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}