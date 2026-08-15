use super::*;

unsafe extern "C" fn snake_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let restart_frame = WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_s4_s"), restart_frame, 1.0, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn snake_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let count = WorkModule::get_int(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    let motion_kind = MotionModule::motion_kind(boma);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::inc_int(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
        }
    }
    if CancelModule::is_enable_cancel(boma)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if count == 1 && motion_kind == hash40("attack_s4_s") {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(boma, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if count == 2 && motion_kind == hash40("attack_s4_s2") {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(boma, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
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

unsafe extern "C" fn snake_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_attack_s4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_attack_s4_end_status)
    .install()
    ;
}