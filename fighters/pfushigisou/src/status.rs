use super::*;

unsafe extern "C" fn pfushigisou_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn pfushigisou_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

unsafe extern "C" fn pfushigisou_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_start"), L2CValue::Hash40s("special_air_n_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(pfushigisou_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(boma) {
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

unsafe extern "C" fn pfushigisou_sludge_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn pfushigisou_sludge_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_param_int(boma, hash40("param_cannonballcloned"), hash40("life"));
    let speed_max = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("speed_max"));
    let gravity = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("gravity"));
    let angle: f32 = 80.0;
    let lr = PostureModule::lr(boma);
    let speed_x = angle.to_radians().sin()*speed_max*lr;
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(boma, 0.001);
    PostureModule::set_pos(boma, &Vector3f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)+12.0, z: PostureModule::pos_z(boma)});
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x/4.0, speed_max);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn pfushigisou_sludge_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    if should_remove_projectile(weapon) {
        sludge_removal(weapon);
    }
    weapon.fastshift(L2CValue::Ptr(pfushigisou_sludge_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn pfushigisou_sludge_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        sludge_removal(weapon);
    }
    if should_remove_projectile(weapon) {
        sludge_removal(weapon);
    }
    0.into()
}

unsafe extern "C" fn pfushigisou_sludge_shoot_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pfushigisou_sludge_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("pfushigisou")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, pfushigisou_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, pfushigisou_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, pfushigisou_special_n_main_status)
    .install()
    ;
    Agent::new("pfushigisou_cannonballcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_PFUSHIGISOU_SLUDGE_STATUS_KIND_SHOOT, pfushigisou_sludge_shoot_pre_status)
    .status(Init, *WEAPON_PFUSHIGISOU_SLUDGE_STATUS_KIND_SHOOT, pfushigisou_sludge_shoot_init_status)
    .status(Main, *WEAPON_PFUSHIGISOU_SLUDGE_STATUS_KIND_SHOOT, pfushigisou_sludge_shoot_main_status)
    .status(Exec, *WEAPON_PFUSHIGISOU_SLUDGE_STATUS_KIND_SHOOT, pfushigisou_sludge_shoot_exec_status)
    .status(End, *WEAPON_PFUSHIGISOU_SLUDGE_STATUS_KIND_SHOOT, pfushigisou_sludge_shoot_end_status)
    .install()
    ;
}