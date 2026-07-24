use super::*;

unsafe extern "C" fn edge_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_CHARGED);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    fighter.sub_attack_air_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn edge_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(boma);
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let charge_frame = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW || motion_kind == hash40("attack_air_lw") {
            if frame >= 6.0 && frame < 17.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    if charge_frame < 24 {
                        MotionModule::set_rate(boma, 0.02*(charge_frame as f32));
                        WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                    }
                }
                else {
                    MotionModule::set_rate(boma, 1.0);
                }
                if charge_frame >= 24 && !WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_CHARGED) {
                    WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_CHARGED);
                    MotionModule::set_rate(boma, 1.0);
                }
            }
        }
    }
    if !fighter.status_AttackAir_Main().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            FighterUtil::check_cloud_through_out(boma);
        }
    }
    0.into()
}

unsafe extern "C" fn edge_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    if status_kind != *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_CHARGED);
        WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    }
    0.into()
}

unsafe extern "C" fn edge_attack_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    if status_kind != *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_CHARGED);
        WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    }
    fighter.sub_attack_air_uniq_process_exit()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, edge_attack_air_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, edge_attack_air_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, edge_attack_air_exit_status)
    .install()
    ;
}