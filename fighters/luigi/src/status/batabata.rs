use super::*;

unsafe extern "C" fn luigi_batabata_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, (*FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_START_TURN) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_batabata_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let air_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_x"), 0);
    let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, get_sum_speed_y*0.66, 0, 0, 0);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -(gravity*0.1));
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable*0.3);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable*0.3);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed*1.13);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed*1.13);
    0.into()
}

unsafe extern "C" fn luigi_batabata_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_BATABATA);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("bata_bata"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_batabata_loop as *const () as _))
}

unsafe extern "C" fn luigi_batabata_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && frame < 105.0 {
        MotionModule::set_frame(fighter.module_accessor, 106.0, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_batabata_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let gravity_mul = 0.1+(0.015*current_frame.clamp(0.0, 80.0));
    let air_speed_y_mul = 0.3+(0.015*current_frame.clamp(0.0, 80.0));
    let air_speed_x_mod = 1.13-(0.00433*current_frame.clamp(0.0, 30.0));
    let air_accel_y = -(gravity*gravity_mul);
    let air_speed_y = air_speed_y_stable*air_speed_y_mul;
    let air_speed_x = air_speed_x_stable*air_speed_x_mod;
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x);
    0.into()
}

unsafe extern "C" fn luigi_batabata_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_batabata_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_BATABATA, luigi_batabata_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_BATABATA, luigi_batabata_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_BATABATA, luigi_batabata_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_BATABATA, luigi_batabata_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_BATABATA, luigi_batabata_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_BATABATA, luigi_batabata_exit_status)
    .install()
    ;
}