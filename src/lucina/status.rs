use super::*;

unsafe extern "C" fn lucina_down_special_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(lucina_down_special_attack_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT)(fighter)
    }
}

unsafe extern "C" fn lucina_down_special_attack_loop(fighter: &mut L2CFighterCommon) -> bool {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit"), -1.0, 1.0, 0.0, false, false);
        }
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into();
    }
    return false.into()
}

pub fn install() {
    Agent::new("lucina")
    .status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, lucina_down_special_attack_main_status)
    .install()
    ;
}