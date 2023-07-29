/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Jumpsquat Main, sets the motion rate for Jump Cancelable Statuses
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    /* START OF NEW ADDITIONS */
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    if fighter_kind == *FIGHTER_KIND_DONKEY {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            MotionModule::set_rate(boma, 0.8);
        }
    }
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
            return 0.into();
        }
        if !fighter.sub_transition_specialflag_hoist().get_bool() {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
                    return 0.into();
                }
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                    return 0.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1daca540be));
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    0.into()
}

//Status End Jumpsquat, clears flags
#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe fn status_end_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    if fighter_kind == *FIGHTER_KIND_DONKEY {
        MotionModule::set_rate(boma, 1.0);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jumpsquat_main,
            status_end_jumpsquat
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}