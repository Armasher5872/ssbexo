use super::*;

pub unsafe extern "C" fn attack_hi4_cancel(fighter: &mut L2CFighterCommon) -> bool {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let attack_dash_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
    let is_cancel_catch = is_cancel_catch(fighter);
    let is_tilt_input = attack_dash_frame > 1 && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    let is_attack_hi4 = stick_y >= 0.7 && is_cancel_catch || is_tilt_input || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0;
    WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) && is_attack_hi4 && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
}

pub unsafe extern "C" fn attack_lw4_cancel(fighter: &mut L2CFighterCommon) -> bool {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let attack_dash_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
    let is_cancel_catch = is_cancel_catch(fighter);
    let is_tilt_input = attack_dash_frame > 1 && cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    let is_attack_lw4 = stick_y <= -0.7 && is_cancel_catch || is_tilt_input || cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0;
    WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) && is_attack_lw4 && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
}

unsafe fn is_cancel_catch(fighter: &mut L2CFighterCommon) -> bool {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
}