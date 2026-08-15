use super::*;

unsafe extern "C" fn ness_attack_air_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_init()
}

unsafe extern "C" fn ness_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_attack_air_common(false.into());
    MotionModule::set_trans_move_speed_no_scale(boma, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(ness_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn ness_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_AttackAir_Main_common().get_bool() {
        return 1.into();
    }
    else {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
    }
    0.into()
}

unsafe extern "C" fn ness_attack_air_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec()
}

unsafe extern "C" fn ness_attack_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit()
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, ness_attack_air_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, ness_attack_air_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, ness_attack_air_exec_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, ness_attack_air_exit_status)
    .install()
    ;
}