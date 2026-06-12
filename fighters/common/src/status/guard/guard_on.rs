use super::*;

//Sub Guard Cont Pre
#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont_pre)]
unsafe extern "C" fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS
    ];
    let boma = fighter.module_accessor;
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON {
        if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            let catch_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("catch_dash_frame"));
            WorkModule::set_int(boma, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        }
    }
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(boma, *term);
    }
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

//Sub Guard Cont, handles Shield Dropping
#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont)]
unsafe extern "C" fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let cmd_cat3 = fighter.global_table[CMD_CAT3].get_i32();
    let boma = fighter.module_accessor;
    let attack_on = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
    let have_item = ItemModule::is_have_item(boma, 0);
    let lr = PostureModule::lr(boma);
    let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
    let squat_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"));
    let invalid_catch_frame = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME);
    let guard_hold = fighter.check_guard_hold();
    let check_guard_special_hi = fighter.check_guard_attack_special_hi(guard_hold).get_bool();
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    if fighter.global_table[GUARD_CONT_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return true.into();
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return true.into();
    }
    if !fighter.check_guard_hold().get_bool() {
        if situation_kind == *SITUATION_KIND_GROUND {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) && have_item && !item_lua_stack_no_throw && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0 && cmd_cat3 & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4) != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return true.into();
            }
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
                return true.into();
            }
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return true.into();
            }
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
                return true.into();
            }
        }
        if !check_guard_special_hi && fighter.sub_transition_group_check_ground_jump().get_bool() {
            return true.into();
        }
    }
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_GUARD_ON && status_kind == *FIGHTER_STATUS_KIND_RUN && attack_on && situation_kind == *SITUATION_KIND_GROUND && !have_item {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
            if stick_x*lr <= turn_run_stick_x {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
                return true.into();
            }
        }
    }
    if GroundModule::is_passable_ground(boma) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS) && stick_y <= squat_stick_y && situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return true.into();
    }
    if check_guard_special_hi {
        return true.into();
    }
    if invalid_catch_frame == 0 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && situation_kind == *SITUATION_KIND_GROUND && !have_item {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
        return true.into();
    }
    false.into()
}

//Sub Ft Status Uniq Process Guard On Init Status Common, adds the shield grabbox
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
unsafe extern "C" fn sub_ftstatusuniqprocessguardon_initstatus_common(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let shield_setoff_mul = WorkModule::get_param_float(boma, hash40("common"), 0x20d241cd64);
    let guard_off_disable_shield_recovery = WorkModule::get_param_int(boma, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    ShieldModule::set_status(boma, *FIGHTER_STATUS_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    ShieldModule::set_hit_stop_mul(boma, shield_setoff_mul);
    WorkModule::set_int(boma, guard_off_disable_shield_recovery, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
}

//Status Guard On Main, makes shield effects show up frame 1 instead of 2
#[skyline::hook(replace = L2CFighterCommon_status_GuardOn_Main)]
unsafe extern "C" fn status_guardon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        WorkModule::on_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    }
    if !fighter.sub_status_guard_on_main_air_common().get_bool()
    && !fighter.sub_guard_cont().get_bool()
    && !fighter.status_guard_main_common().get_bool()  {
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
        }
    }
    0.into()
}

//Effect Guard On Common, deals with Shield Effects
#[skyline::hook(replace = L2CFighterAnimcmdEffectCommon_effect_GuardOnCommon)]
unsafe extern "C" fn effect_guardoncommon(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    agent.clear_lua_stack();
    is_excute(lua_state);
    let excute = agent.pop_lua_stack(1).get_bool();
    if excute {
        //Shield Smoke
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FLW_POS(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 0.5);
        LAST_EFFECT_SET_RATE(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 0.5);
        LAST_EFFECT_SET_ALPHA(lua_state);
        //Base Color
        let color = {agent.clear_lua_stack(); lua_args!(agent, FT_VAR_INT_TEAM_COLOR); get_value_int(lua_state, *FT_VAR_INT_TEAM_COLOR)};
        //External Shield, prevents shield poking
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1, false, 0, color);
        EFFECT_FOLLOW_arg12(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 0.22);
        LAST_EFFECT_SET_ALPHA(lua_state);
        //Internal Shield, demonstrates shield health
        let shield_hp = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_max = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1*ratio, false, 0, color);
        EFFECT_FOLLOW_arg12(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 0.6);
        LAST_EFFECT_SET_ALPHA(lua_state);
        let effect_id = EffectModule::get_last_handle(boma) as u32;
        WorkModule::set_int(boma, effect_id as i32, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_guard_cont_pre,
            sub_guard_cont,
            status_guardon_main,
            effect_guardoncommon
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}