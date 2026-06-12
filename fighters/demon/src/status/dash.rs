use super::*;

unsafe extern "C" fn demon_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Dash_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_dash_main_loop as *const () as _))
}

unsafe extern "C" fn demon_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let boma = fighter.module_accessor;
    let dash_to_attack_stand1_frame = WorkModule::get_param_int(boma, hash40("param_attack_step"), hash40("dash_to_attack_stand1_frame"));
    if !demon_dash_main_common(fighter, 0.into()).get_bool() {
        if situation_kind == *SITUATION_KIND_GROUND {
            if prev_status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                    if current_frame <= dash_to_attack_stand1_frame {
                        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1.into(), true.into());
                    }
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn demon_dash_main_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let boma = fighter.module_accessor;
    let is_have_item = ItemModule::is_have_item(boma, 0);
    let lr = PostureModule::lr(boma);
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
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            throw = pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
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
        sv_module_access::item(fighter.lua_state_agent);
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
        sv_module_access::item(fighter.lua_state_agent);
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
        sv_module_access::item(fighter.lua_state_agent);
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
    if fighter.sub_transition_group_check_special_command().get_bool() {
        return true.into();
    }
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return true.into();
    }
    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return true.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(boma, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
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
    if fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_bool() && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_ptr());
        callable(fighter).get_bool()
    } {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
        return 1.into();
    }
    if pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        if stick_x*lr < 0.3 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_DASH_2.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_DASH_3.into(), true.into());
        }
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return true.into();
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
            sv_module_access::item(fighter.lua_state_agent);
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
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) 
    && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
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
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
        && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
        && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
        && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE != 0 {
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_MIST_STEP_ACTIVE) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 1.into();
    }
    if param_1.get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_1.get_ptr());
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

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_DASH, demon_dash_main_status)
    .install()
    ;
}