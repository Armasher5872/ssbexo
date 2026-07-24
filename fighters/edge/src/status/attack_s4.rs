use super::*;

unsafe extern "C" fn edge_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    edge_sub_attack_s4(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn edge_sub_attack_s4(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let get_stick_dir = ControlModule::get_stick_dir(boma);
    let attack_s4_stick_dir_hi = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_s4_stick_dir_hi"));
    let attack_s4_stick_dir_lw = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_s4_stick_dir_lw"));
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if attack_s4_stick_dir_hi < get_stick_dir {
            WorkModule::set_int64(boma, hash40("attack_s4_hi_wing") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
        }
        else if get_stick_dir < attack_s4_stick_dir_lw {
            WorkModule::set_int64(boma, hash40("attack_s4_lw_wing") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
        }
        else {
            WorkModule::set_int64(boma, hash40("attack_s4_s") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
        }
    }
    else {
        WorkModule::set_int64(boma, hash40("attack_s4_s") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
    }
    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
}

unsafe extern "C" fn edge_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let count = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    let motion_kind = MotionModule::motion_kind(boma);
    let is_attack = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4;
    if !StatusModule::is_changing(boma) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            if fighter.global_table[CMD_CAT1].get_i32() & is_attack != 0 {
                WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
            }
        }
    }
    else {
        fighter.attack_s4_mtrans();
    }
    if CancelModule::is_enable_cancel(boma)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if count == 1 && motion_kind == hash40("attack_s4_lw_wing") {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(boma, Hash40::new("attack_s4_lw_wing2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

unsafe extern "C" fn edge_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, edge_attack_s4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, edge_attack_s4_end_status)
    .install()
    ;
}