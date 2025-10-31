use super::*;

//Cargo Up Throw Main Status
unsafe extern "C" fn donkey_throw_f_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_7100021780(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_throw_f_hi_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_throw_f_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
        else {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

//Cargo Up Throw End Status
unsafe extern "C" fn donkey_throw_f_hi_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI, donkey_throw_f_hi_main_status)
    .status(End, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI, donkey_throw_f_hi_end_status)
    .install()
    ;
}