use super::*;

//switch status before anything happens in LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN
#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ivysaur_neutral_special_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    PFUSHIGISOU_IS_SPECIAL_N[entry_id] = true;
    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
    0.into()
}

//new neutral-special script in light item throw status
#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_ITEM_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ivysaur_light_item_throw_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        fighter.sub_shift_status_main(L2CValue::Ptr(ivysaur_light_item_throw_status_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

//resets flag when status ends
#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_ITEM_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn ivysaur_light_item_throw_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    PFUSHIGISOU_IS_SPECIAL_N[entry_id] = false;
    original!(fighter)
}

pub unsafe fn ivysaur_light_item_throw_status_loop(fighter: &mut L2CFighterCommon) -> bool {
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

pub fn install() {
    install_status_scripts!(
        ivysaur_neutral_special_status_pre,
        ivysaur_light_item_throw_status_main,
        ivysaur_light_item_throw_status_end
    );
}