use super::*;

//switch status before anything happens in LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN
unsafe extern "C" fn pfushigisou_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    PFUSHIGISOU_IS_SPECIAL_N[entry_id] = true;
    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
    0.into()
}

//new neutral-special script in light item throw status
unsafe extern "C" fn pfushigisou_light_item_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if PFUSHIGISOU_IS_SPECIAL_N[entry_id] {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_light_item_throw_status_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_ITEM_THROW)(fighter)
    }
}

unsafe extern "C" fn pfushigisou_light_item_throw_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
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

//resets flag when status ends
unsafe extern "C" fn pfushigisou_light_item_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    PFUSHIGISOU_IS_SPECIAL_N[entry_id] = false;
    original_status(End, fighter, *FIGHTER_STATUS_KIND_ITEM_THROW)(fighter)
}

pub fn install() {
    Agent::new("pfushigisou")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, pfushigisou_special_n_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ITEM_THROW, pfushigisou_light_item_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ITEM_THROW, pfushigisou_light_item_throw_end_status)
    .install()
    ;
}