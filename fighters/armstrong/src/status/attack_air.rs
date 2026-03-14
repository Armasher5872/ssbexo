use super::*;

unsafe extern "C" fn armstrong_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N || motion_kind == hash40("attack_air_n") {
        armstrong_charge_move(fighter, 2.0, 7.0, 0.025, 0.0, ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK), false);
    }
    if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F || motion_kind == hash40("attack_air_f") {
        armstrong_charge_move(fighter, 4.0, 12.0, 0.045, 0.0, ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK), false);
    }
    if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B || motion_kind == hash40("attack_air_b") {
        armstrong_charge_move(fighter, 1.0, 5.0, 0.025, 0.0, ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK), false);
    }
    if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI || motion_kind == hash40("attack_air_hi") {
        armstrong_charge_move(fighter, 1.0, 5.0, 0.025, 0.0, ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK), false);
    }
    if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW || motion_kind == hash40("attack_air_lw") {
        armstrong_charge_move(fighter, 3.0, 11.0, 0.045, 0.0, ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK), false);
    }
    if !fighter.status_AttackAir_Main().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            FighterUtil::check_cloud_through_out(fighter.module_accessor);
        }
    }
    0.into()
}

unsafe extern "C" fn armstrong_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    armstrong_clear_charge(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn armstrong_attack_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    armstrong_clear_charge(fighter.module_accessor);
    fighter.sub_attack_air_uniq_process_exit()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, armstrong_attack_air_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, armstrong_attack_air_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, armstrong_attack_air_exit_status)
    .install()
    ;
}