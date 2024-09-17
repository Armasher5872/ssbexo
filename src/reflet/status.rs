use super::*;

//Resurrection Book

unsafe extern "C" fn reflet_resurrection_book_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn reflet_resurrection_book_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

//Resurrection Levin Sword

unsafe extern "C" fn reflet_resurrection_thundersword_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn reflet_resurrection_thundersword_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

//Side Special

unsafe extern "C" fn reflet_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.clear_lua_stack();
    let app_fighter: *mut Fighter = std::mem::transmute(sv_system::battle_object(fighter.lua_state_agent));
    let current_point = WorkModule::get_float(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
    FighterSpecializer_Reflet::change_hud_kind(app_fighter, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    if situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if current_point > 0.0 {
        FighterSpecializer_Reflet::change_grimoire(fighter_module_accessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK);
    }
    else {
        FighterSpecializer_Reflet::change_grimoire(fighter_module_accessor, -1);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let enhanced_magic_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_landing_smoke"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
            sv_module_access::effect(fighter.lua_state_agent);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) && current_frame < 8.0 {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
    }
    if enhanced_magic_timer >= 7 {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn reflet_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE);
    0.into()
}

//Arcfire

unsafe extern "C" fn reflet_gigafire_shoot0_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn reflet_gigafire_shoot0_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_gigafire"), hash40("life"));
    let pillar_hp = WorkModule::get_param_float(weapon.module_accessor, hash40("param_gigafire"), hash40("pillar_hp"));
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    if owner_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        WorkModule::on_flag(weapon.module_accessor, WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA);
    }
    if !WorkModule::is_flag(weapon.module_accessor, WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    else {
        WorkModule::set_int(weapon.module_accessor, 130, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, 130, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x+(10.0*PostureModule::lr(weapon.module_accessor)), y: pos_y-7.0, z: pos_z});
    }
    WorkModule::set_float(weapon.module_accessor, pillar_hp, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLOAT_HP);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_VANISH);
    HitModule::set_whole(weapon.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    0.into()
}

unsafe extern "C" fn reflet_gigafire_shoot0_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(reflet_gigafire_shoot0_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_gigafire_shoot0_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
        if WorkModule::is_flag(weapon.module_accessor, WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_BOLGANONE) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
        weapon.change_status(WEAPON_REFLET_GIGAFIRE_STATUS_KIND_RISE.into(), false.into());
        return 0.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) 
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) 
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
        weapon.change_status(WEAPON_REFLET_GIGAFIRE_STATUS_KIND_RISE.into(), false.into());
        return 0.into();
    }
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn reflet_gigafire_shoot0_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Up Special

unsafe extern "C" fn reflet_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
    }
    0.into()
}

unsafe extern "C" fn reflet_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let special_hi_landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_frame"));
    let special_hi_current_point = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_float(fighter.module_accessor, special_hi_landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    if special_hi_current_point <= 0 {
        FighterSpecializer_Reflet::set_flag_to_table(module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    reflet_special_hi_check_jump(fighter);
    reflet_special_hi_try_2nd(fighter);
    0.into()
}

unsafe extern "C" fn reflet_special_hi_check_jump(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let get_special_hi_jump_speed = FighterSpecializer_Reflet::get_special_hi_jump_speed(module_accessor);
    let grav_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let control_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x2c13759450);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_x_limit"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_add(control_energy as *mut smash::app::FighterKineticEnergyController, air_accel_x_add*control_mul);
        smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_mul(control_energy as *mut smash::app::FighterKineticEnergyController, air_accel_x_mul*control_mul);
        smash::app::lua_bind::KineticEnergy::reset_energy(control_energy as *mut smash::app::KineticEnergy, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, &Vector2f{x: 0.0, y: 0.0}, &Vector3f::zero(), fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::reset_energy(grav_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: get_special_hi_jump_speed.y}, &Vector3f::zero(), fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::unable(stop_energy);
        smash::app::lua_bind::KineticEnergyNormal::set_limit_speed(control_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: air_speed_x_limit*control_mul, y: 0.0});
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, get_special_hi_jump_speed.x, 0.0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
}

unsafe extern "C" fn reflet_special_hi_try_2nd(fighter: &mut L2CFighterCommon) {
    let special_hi_current_point = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if special_hi_current_point > 0 {
            fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2.into(), false.into())
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
}

//Down Special

unsafe extern "C" fn reflet_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.clear_lua_stack();
    let app_fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(fighter.lua_state_agent));
    FighterSpecializer_Reflet::change_hud_kind(app_fighter, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
    AttackModule::set_overlap_hit(fighter.module_accessor, true);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_OBJECT_ID);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) > 0 {
        FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
    }
    else {
        FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut FighterModuleAccessor, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_LW);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_AIR_LW);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let catch_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_WORK_INT_CATCH_STATUS);
    let enhanced_magic_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_GRAB_IS_GRAB, 0);
    sv_module_access::grab(fighter.lua_state_agent);
    if fighter.pop_lua_stack(1).get_bool() {
        if !FighterSpecializer_Reflet::check_special_lw_pos(fighter.module_accessor as *mut FighterModuleAccessor) {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_GRAB_CLEAR, 0);
            sv_module_access::grab(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_GRAB_CLEAR, 1);
            sv_module_access::grab(fighter.lua_state_agent);
        }
    }
    if catch_status == *FIGHTER_REFLET_STATUS_SPECIAL_LW_CATCH_STATUS_CATCH_START {
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE.into(), false.into());
        return 1.into()
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) && current_frame < 8.0 {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
    }
    if enhanced_magic_timer >= 7 {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE);
    }
    if situation_kind != prev_situation_kind {
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_LW);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0);
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_EFFECT_LANDING_EFFECT, hash40("sys_landing_smoke"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, false, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_module_access::effect(fighter.lua_state_agent);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_AIR_LW);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn reflet_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE);
    0.into()
}

pub fn install() {
    Agent::new("reflet")
    .status(Pre, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_BOOK, reflet_resurrection_book_pre_status)
    .status(Init, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_BOOK, reflet_resurrection_book_init_status)
    .status(Pre, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_THUNDERSWORD, reflet_resurrection_thundersword_pre_status)
    .status(Init, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_THUNDERSWORD, reflet_resurrection_thundersword_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, reflet_special_s_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, reflet_special_s_end_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, reflet_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, reflet_special_hi_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, reflet_special_lw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, reflet_special_lw_end_status)
    .install()
    ;
    Agent::new("reflet_gigafire")
    .status(Pre, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_SHOOT0, reflet_gigafire_shoot0_pre_status)
    .status(Init, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_SHOOT0, reflet_gigafire_shoot0_init_status)
    .status(Main, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_SHOOT0, reflet_gigafire_shoot0_main_status)
    .status(Exec, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_SHOOT0, reflet_gigafire_shoot0_exec_status)
    .install()
    ;
}