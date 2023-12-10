use super::*;

//Side Special Exec Status
unsafe extern "C" fn mario_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if (12.0..24.0).contains(&frame) {
            macros::SET_SPEED_EX(fighter, 1.5, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if (25.0..40.0).contains(&frame) {
            macros::SET_SPEED_EX(fighter, 0.75, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

//Up Special Main Status, goes into the loop status
unsafe extern "C" fn mario_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_loop as *const () as _));
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
}

//Up Special Loop, goes into the Super Jump Punch Main function
unsafe extern "C" fn special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_main();
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_exec_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, mario_special_hi_main_status)
    .install()
    ;
}