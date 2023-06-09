/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*SHIELD STATUSES*/
//Sub Guard Cont Pre
#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont_pre)]
unsafe fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
        WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    }
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B
    ];
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    if GroundModule::is_passable_ground(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

//Sub Guard Cont
#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont)]
unsafe fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let cont_stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = cont_stick_x * lr;
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    let turn_run_stick_x_threshold = stick_x * lr <= turn_run_stick_x;
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
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) && ItemModule::is_have_item(fighter.module_accessor, 0) && item_lua_stack_no_throw {
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 
            || (fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0
            && fighter.global_table[CMD_CAT3].get_i32() & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4) != 0) {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                    return true.into();
                }
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return true.into();
        }
        /* START OF NEW ADDITION */
        //Allows platform drops out of shield
        let shield_drop_check = (fighter.global_table[CMD_CAT2].get_i32() & (*FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0) || fighter.down_input() == true;
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
        && shield_drop_check
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return true.into();
        }
        /* END OF NEW ADDITION */
    }
    if is_shield_stop {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
        && turn_run_stick_x_threshold
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return true.into();
        }
    }
    if !fighter.check_guard_attack_special_hi(check_guard_hold.into()).get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) == 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
            && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
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

//Status Guard Main Common, Yandere Dev Ass Coding in order to make variable color effects
#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let shield = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let sgt = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER);
    if shield < 0.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
        true.into()
    } 
    else {
        WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER);
        if sgt > 10 {
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER);
        }
        if sgt == 1 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0xafae75f05), false, false);
            fighter.clear_lua_stack(); 
            lua_args!(fighter, Hash40::new_raw(0xafae75f05), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.1, false, 0, *FT_VAR_INT_TEAM_COLOR); 
            smash::app::sv_animcmd::EFFECT_FOLLOW_arg12(fighter.lua_state_agent);
            if color == 0 {
                //Red
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0222*shield, 0.0, 0.0);
            }
            else if color == 1 {
                //Blue
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0222*shield);
            }
            else if color == 2 {
                //Green
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0112*shield, 0.0);
            }
            else if color == 3 {
                //Yellow
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0222*shield, 0.0222*shield, 0.0);
            }
            else if color == 4 {
                //Orange
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0222*shield, 0.0144*shield, 0.0);
            }
            else if color == 5 {
                //Cyan
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0222*shield, 0.0222*shield);
            }
            else if color == 6 {
                //Pink
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0222*shield, 0.0167*shield, 0.0177*shield);
            }
            else {
                //Purple
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0112*shield, 0.0, 0.0112*shield);
            }
        }
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) 
        && WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME) <= 0 
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
            true.into()
        } 
        else {
            false.into()
        }
    }
}

//Effect GuardOnCommon
#[skyline::hook(replace = L2CFighterAnimcmdEffectCommon_effect_GuardOnCommon)]
unsafe fn effect_guardoncommon(fighter: &mut L2CAgentBase) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    smash::app::sv_animcmd::EFFECT_FLW_POS(fighter.lua_state_agent);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_guard_cont_pre,
            sub_guard_cont,
            status_guard_main_common,
            effect_guardoncommon
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}