/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*SHIELD STATUSES*/

//Sub Guard Cont Pre
#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont_pre)]
unsafe fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS
    ];
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON {
        if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
            WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        }
    }
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

//Sub Guard Cont
#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont)]
unsafe fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let cmd_cat3 = fighter.global_table[CMD_CAT3].get_i32();
    let global_stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let stick_x = global_stick_x*PostureModule::lr(fighter.module_accessor);
    let is_have_item = ItemModule::is_have_item(fighter.module_accessor, 0);
    let check_button_attack = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let turn_run_stick_x_threshold = stick_x <= turn_run_stick_x;
    let check_guard_hold = fighter.check_guard_hold().get_bool();
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    let is_shield_stop = fighter.global_table[STATUS_KIND_INTERRUPT] == FIGHTER_STATUS_KIND_GUARD_ON && fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN;
    if fighter.global_table[GUARD_CONT_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return true.into();
    }
    if !check_guard_hold {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) && is_have_item && item_lua_stack_no_throw {
            if pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 || (pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0 && cmd_cat3 & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4) != 0) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                    return true.into();
                }
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0 && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0 && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0 && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return true.into();
        }
    }
    /* START OF NEW ADDITION */
    //Allows platform drops out of shield
    if GroundModule::is_passable_ground(fighter.module_accessor)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS) 
    && stick_y <= squat_stick_y
    && situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return true.into();
    }
    /* END OF NEW ADDITION */
    if is_shield_stop {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) && turn_run_stick_x_threshold && check_button_attack && situation_kind == *SITUATION_KIND_GROUND && !is_have_item {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) && check_button_attack && situation_kind == *SITUATION_KIND_GROUND && !is_have_item {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return true.into();
        }
    }
    if !fighter.check_guard_attack_special_hi(check_guard_hold.into()).get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) == 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && situation_kind == *SITUATION_KIND_GROUND && !is_have_item {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                return true.into();
            }
        }
        if !check_guard_hold {
            if fighter.sub_transition_group_check_ground_jump().get_bool() {
                return true.into();
            }
        }
        false.into()
    }
    else {
        true.into()
    }
}

//Status Guard On Main, makes shield effects show up frame 1 instead of 2
#[skyline::hook(replace = L2CFighterCommon_status_GuardOn_Main)]
unsafe extern "C" fn status_guardon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    }
    if !fighter.sub_status_guard_on_main_air_common().get_bool()
    && !fighter.sub_guard_cont().get_bool()
    && !fighter.status_guard_main_common().get_bool()  {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
        }
    }
    0.into()
}

//Status Guard Main Common, handles shield special transitioning
#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        let min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if min_frame <= 0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                return true.into();
            }
        }
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if fighter_kind == *FIGHTER_KIND_NESS {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_GUARD.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_GUARD_BURST.into(), false.into());
            } 
        }
        if fighter_kind == *FIGHTER_KIND_LUCAS {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_GUARD.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST.into(), false.into());
            }   
        }
        if fighter_kind == *FIGHTER_KIND_ROSETTA {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_GUARD.into(), false.into());
        }
    }
    false.into()
}

//Sub ftStatusUniqProcessGuardFunc_updateShield. Removes shield tilting
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardFunc_updateShield)]
unsafe fn sub_ftstatusuniqprocessguardfunc_updateshield(fighter: &mut L2CFighterCommon, _param_1: L2CValue) {
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    let shield_eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID) as u32;
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    if EffectModule::is_exist_effect(fighter.module_accessor, shield_eff) {
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0)*0.1;
        EffectModule::set_scale(fighter.module_accessor, shield_eff, &Vector3f{x: ratio, y: ratio, z: ratio});
    }
}

//FighterStatusGuard set_shield_scale. Removes shield tilting, and makes shields no longer decrease in size
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__set_shield_scale)]
unsafe fn fighterstatusguard_set_shield_scale(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    let shield_eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID) as u32;
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    if EffectModule::is_exist_effect(fighter.module_accessor, shield_eff) {
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0)*0.1;
        EffectModule::set_scale(fighter.module_accessor, shield_eff, &Vector3f{x: ratio, y: ratio, z: ratio});
    }
    0.into()
}

//FighterStatusGuard__check_hit_stop_delay_flick. Removes shield SDI
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__check_hit_stop_delay_flick)]
unsafe extern "C" fn fighterstatusguard_check_hit_stop_delay_flick(_fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    false.into()
}

//Effect Guard On Common, deals with Shield Effects
#[skyline::hook(replace = L2CFighterAnimcmdEffectCommon_effect_GuardOnCommon)]
unsafe fn effect_guardoncommon(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.agent;
    agent.clear_lua_stack();
    is_excute(agent.lua_state_agent);
    let excute = agent.pop_lua_stack(1).get_bool();
    if excute {
        //Shield Smoke
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FLW_POS(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 0.5);
        LAST_EFFECT_SET_RATE(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 0.5);
        LAST_EFFECT_SET_ALPHA(agent.lua_state_agent);
        //Base Color
        let color = {agent.clear_lua_stack(); lua_args!(agent, FT_VAR_INT_TEAM_COLOR); get_value_int(agent.lua_state_agent, *FT_VAR_INT_TEAM_COLOR)};
        //External Shield, prevents shield poking
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1, false, 0, color);
        EFFECT_FOLLOW_arg12(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 0.22);
        LAST_EFFECT_SET_ALPHA(agent.lua_state_agent);
        //Internal Shield, demonstrates shield health
        let shield_hp = WorkModule::get_float(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_max = WorkModule::get_float(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_shield"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1*ratio, false, 0, color);
        EFFECT_FOLLOW_arg12(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 0.6);
        LAST_EFFECT_SET_ALPHA(agent.lua_state_agent);
        let effect_id = EffectModule::get_last_handle(agent.module_accessor) as u32;
        WorkModule::set_int(agent.module_accessor, effect_id as i32, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_guard_cont_pre,
            sub_guard_cont,
            status_guardon_main,
            status_guard_main_common,
            sub_ftstatusuniqprocessguardfunc_updateshield,
            fighterstatusguard_set_shield_scale,
            fighterstatusguard_check_hit_stop_delay_flick,
            effect_guardoncommon
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}
