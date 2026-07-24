/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Jumpsquat Common, enables JC grab
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_common)]
unsafe extern "C" fn status_jumpsquat_common(fighter: &mut L2CFighterCommon, param_2: L2CValue) {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let stick_jump_command_life = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if stick_jump_command_life == 0 || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0 {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        if ControlModule::is_jump_mini_button(boma) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if param_2.get_bool() {
        PostureModule::set_stick_lr(boma, 0.0);
        PostureModule::update_rot_y_lr(boma);
    }
    ControlModule::reset_flick_y(boma);
    ControlModule::reset_flick_sub_y(boma);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
    if ![*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&prev_status_kind) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH); //Added
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        MotionAnimcmdModule::enable_skip_delay_update(boma);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    }
}

//Status Jumpsquat Main, enables Wavedash out of Jumpsquat
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe extern "C" fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
    }
    else {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) && situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        if !fighter.sub_transition_group_check_ground_item().get_bool() {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                    }
                }
            }
            if !fighter.sub_transition_specialflag_hoist().get_bool() {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) {
                    if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                        if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
                            return 0.into();
                        }
                        if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_ATTACK_DASH_ATTACK_HI4 != 0 {
                            if situation_kind == *SITUATION_KIND_GROUND {
                                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                            }
                        }
                    }
                }
            }
        }
    }
    0.into()
}

//Sub Jump Squat Uniq Check Sub Mini Attack, seeks to remove the SH Macro
#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub_mini_attack)]
unsafe extern "C" fn sub_jump_squat_uniq_check_sub_mini_attack(fighter: &mut L2CFighterCommon) {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) {
        return;
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON) {
            if current_frame > 0.0 {
                FighterControlModuleImpl::reserve_on_attack_button(boma);
                WorkModule::off_flag(boma, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
            }
        }
    }
    else {
        let mut cont = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK);
        if cont {
            cont = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP);
            if !cont {
                cont = WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK);
            }
        }
        else {
            cont = WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK);
        }
        if !cont {
            return;
        }
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
}

//Sub Jump Squat Uniq Check Sub, handles dealing with C Stick Drift
#[skyline::hook(replace = L2CFighterCommon_sub_jump_squat_uniq_check_sub)]
unsafe extern "C" fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, flag: L2CValue) {
    let boma = fighter.module_accessor;
    let c_stick_on = Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride);
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let stick_y = if c_stick_on {ControlModule::get_sub_stick_y(boma)} else {fighter.global_table[STICK_Y].get_f32()};
    let jump_neutral_y = WorkModule::get_param_float(boma, hash40("common"), hash40("jump_neutral_y"));
    let jump_squat_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("jump_squat_frame"));
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) {
        return;
    }
    if WorkModule::is_flag(boma, flag.get_i32()) {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::is_jump_mini_button(boma) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        if current_frame >= jump_squat_frame-1 {
            if c_stick_on || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                ControlModule::reset_main_stick_x(boma);
            }
            if c_stick_on || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                ControlModule::reset_main_stick_x(boma);
            }
        }
    }
    else {
        if c_stick_on {
            ControlModule::reset_main_stick_x(boma);
        }
        if stick_y < jump_neutral_y {
            if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) || !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                }
            }
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_jumpsquat_common,
            status_jumpsquat_main,
            sub_jump_squat_uniq_check_sub,
            sub_jump_squat_uniq_check_sub_mini_attack
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}