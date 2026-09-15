/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Transition Group Check Ground, makes turnarounds unbufferable
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground)]
unsafe extern "C" fn sub_transition_group_check_ground(fighter: &mut L2CFighterCommon, to_squat_wait: L2CValue) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let left_stick_x = if Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
    let notify_taunt_hash = {fighter.clear_lua_stack(); lua_args!(fighter, Hash40::new_raw(0x1daca540be)); sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if situation_kind == *SITUATION_KIND_GROUND {
        if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) {
                if notify_taunt_hash {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return true.into();
                }
            }
        }
        if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) {
                if notify_taunt_hash {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return true.into();
                }
            }
        }
        if cmd_cat2 & (*FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) {
                if notify_taunt_hash {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return true.into();
                }
            }
        }
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
                fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
                return true.into();
            }
        }
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
                fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
                return true.into();
            }
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) {
            if fighter.sub_check_command_squat().get_bool() {
                if to_squat_wait.get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), true.into());
                    return true.into();
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
                    return true.into();
                }
            }
        }
        //Vanilla checks for cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0. This normally allows it to be buffered, but this turns out to be a hinderance in EXO's engine due to wavedashes in particular almost always buffering a turnaround.
        if left_stick_x*lr <= WorkModule::get_param_float(boma, hash40("common"), hash40("turn_stick_x")) {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) {
                fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
                return true.into();
            }
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK) {
            if fighter.sub_check_command_walk().get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

//Sub Transition Group Check Ground Guard, removes grab and jump from being a valid button check for dodges. Used for wavedash leniency
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_guard)]
unsafe extern "C" fn sub_transition_group_check_ground_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
    || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
    || Buttons::from_bits_retain(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::Catch) {
        return false.into();
    }
    call_original!(fighter)
}

//Sub Transition Group Check Ground Escape, removes grab and jump from being a valid button check for dodges. Used for wavedash leniency
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_escape)]
unsafe extern "C" fn sub_transition_group_check_ground_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
    || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
    || Buttons::from_bits_retain(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::Catch) {
        return false.into();
    }
    call_original!(fighter)
}

//Sub Transition Group Check Ground Attack, removes grab from being a valid button check for grounded attacks
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_attack)]
unsafe extern "C" fn sub_transition_group_check_ground_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if Buttons::from_bits_retain(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::Catch) {
        return false.into();
    }
    call_original!(fighter)
}

//Sub Transition Group Check Ground Jump Mini Attack, disables the grab button from being used to perform the "short hop aerial macro"
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_jump_mini_attack)]
unsafe fn sub_transition_group_check_ground_jump_mini_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CHECK_GROUND_JUMP_MINI_ATTACK].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_JUMP_MINI_ATTACK].get_ptr());
            return callable(fighter);
        }
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0 && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 && fighter.sub_check_button_jump().get_bool() {
            fighter.change_status_jump_mini_attack(false.into());
            return true.into();
        }
    }
    false.into()
}

//Sub Transition Group Check Air Escape, enables double shield wavedashes
#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_escape)]
unsafe extern "C" fn sub_transition_group_check_air_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let boma = fighter.module_accessor;
    if fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && ((cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N == 0) || Buttons::from_bits_retain(ControlModule::get_trigger(boma)).intersects(Buttons::GuardHold))
    && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        return 1.into();
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_ground,
            sub_transition_group_check_ground_guard,
            sub_transition_group_check_ground_escape,
            sub_transition_group_check_ground_attack,
            sub_transition_group_check_ground_jump_mini_attack,
            sub_transition_group_check_air_escape
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}