use super::*;

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_down_special_attack_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(marth_down_special_attack_status_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

pub unsafe fn marth_down_special_attack_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
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
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
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
    install_status_scripts!(marth_down_special_attack_status_main);
}