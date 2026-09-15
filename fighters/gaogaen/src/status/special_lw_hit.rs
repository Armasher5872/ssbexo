use super::*;

unsafe extern "C" fn gaogaen_special_lw_hit_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), false.into());
    GroundModule::correct(boma, GroundCorrectKind(if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_AIR} else {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP}));
    fun_7100013e50(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_lw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    println!("Current X Speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(boma) {
        GroundModule::correct(boma, GroundCorrectKind(if situation_kind == *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP} else {*GROUND_CORRECT_KIND_AIR}));
        MotionModule::change_motion_inherit_frame(boma, Hash40::new(if situation_kind == *SITUATION_KIND_GROUND {"special_lw"} else {"special_air_lw"}), -1.0, 1.0, 0.0, false, false);
        fun_7100013e50(fighter, false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(if situation_kind == *SITUATION_KIND_GROUND {FIGHTER_STATUS_KIND_WAIT.into()} else {FIGHTER_STATUS_KIND_FALL.into()}, false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT, gaogaen_special_lw_hit_main_status)
    .install()
    ;
}
