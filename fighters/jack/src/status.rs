use super::*;

unsafe extern "C" fn jack_fire_hit_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    /*
    let module_accessor = weapon.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let gravity_angle = smash::app::SlopeModuleSimple::gravity_angle(module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    let param_id = WorkModule::get_int64(weapon.module_accessor, *WEAPON_JACK_FIRE_INSTANCE_WORK_ID_INT_PARAM_ID);
    let hit_life = WorkModule::get_param_int(weapon.module_accessor, param_id, hash40("hit_life"));
    WorkModule::set_int(weapon.module_accessor, hit_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if !StopModule::is_stop(weapon.module_accessor) {
        fun_710003b9a0(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710003b9a0 as *const () as _));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("hit"), 0.0, 1.0, false, 0.0, false, false);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: gravity_angle*lr, y: 0.0, z: 0.0}, 0);
    WorkModule::add_float(get_owner_boma(weapon), 1.0, 0x4D);
    weapon.fastshift(L2CValue::Ptr(jack_fire_hit_main_loop as *const () as _))
    */
    let ret = original_status(Main, weapon, *WEAPON_JACK_FIRE_STATUS_KIND_HIT)(weapon);
    WorkModule::add_float(get_owner_boma(weapon), 1.0, 0x4D);
    ret
}

/*
unsafe extern "C" fn fun_710003b9a0(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn jack_fire_hit_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}
*/

unsafe extern "C" fn jack_summon_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn jack_summon_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn jack_summon_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);
    if rebel_gauge >= 100.0 {
        if FighterSpecializer_Jack::is_cut_in_effect(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SUMMON_FLAG_CUT_IN_EFFECT);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SUMMON_FLAG_CUT_IN_EFFECT) {
            MotionAnimcmdModule::flush(fighter.module_accessor, false);
            FighterSpecializer_Jack::set_cut_in_effect(fighter.module_accessor);
        }
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("summon"), 0.0, 1.0, false, 0.0, false, false);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("mask") as i64, hash40("on") as i64);
    VisibilityModule::set_material_anim_priority(fighter.module_accessor, Hash40::new("mask"), true);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_summon_main_loop as *const () as _))
}

unsafe extern "C" fn jack_summon_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        };
    }
    0.into()
}

unsafe extern "C" fn jack_dispatch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn jack_dispatch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn jack_dispatch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dispatch"), 0.0, 1.0, false, 0.0, false, false);
    VisibilityModule::set_material_anim_priority(fighter.module_accessor, Hash40::new("mask"), true);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_dispatch_main_loop as *const () as _))
}

unsafe extern "C" fn jack_dispatch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        };
    }
    0.into()
}

unsafe extern "C" fn jack_dispatch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_material_anim_priority(fighter.module_accessor, Hash40::new("mask"), false);
    VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mask") as i64, hash40("on") as i64);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE);
    0.into()
}

pub fn install() {
    Agent::new("jack")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_JACK_STATUS_KIND_SUMMON, jack_summon_pre_status)
    .status(Init, *FIGHTER_JACK_STATUS_KIND_SUMMON, jack_summon_init_status)
    .status(Main, *FIGHTER_JACK_STATUS_KIND_SUMMON, jack_summon_main_status)
    .status(Pre, *FIGHTER_JACK_STATUS_KIND_DISPATCH, jack_dispatch_pre_status)
    .status(Init, *FIGHTER_JACK_STATUS_KIND_DISPATCH, jack_dispatch_init_status)
    .status(Main, *FIGHTER_JACK_STATUS_KIND_DISPATCH, jack_dispatch_main_status)
    .status(End, *FIGHTER_JACK_STATUS_KIND_DISPATCH, jack_dispatch_end_status)
    .install()
    ;
    Agent::new("jack_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_JACK_FIRE_STATUS_KIND_HIT, jack_fire_hit_main_status)
    .install()
    ;
}