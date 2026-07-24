use super::*;

unsafe extern "C" fn ganon_volley_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn ganon_volley_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let life = WorkModule::get_param_int(boma, hash40("param_cannonballcloned"), hash40("life"));
    let speed_min = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("speed_max"));
    let lr = PostureModule::lr(boma);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let volley_damage_charge = WorkModule::get_float(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_DAMAGE_CHARGE);
    let volley_scale = WorkModule::get_float(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_SCALE_CHARGE);
    let is_charged = WorkModule::is_flag(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    let direction = WorkModule::get_int(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_N_DIRECTION);
    let speed = if is_charged {speed_max} else {speed_min};
    let speed_y = 0.0f32.to_radians().cos()*speed;
    let y_speed = if direction == 2 {-speed_y/4.5} else if direction == 1 {speed_y/4.5} else {0.0};
    if owner_status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, y_speed);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, y_speed);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::set_flag(boma, is_charged, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLAG_CHARGED);
        WorkModule::set_float(boma, volley_damage_charge, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_DAMAGE_SCALE);
        WorkModule::set_float(boma, volley_scale, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_SCALE);
        if !is_charged {
            ReflectorModule::set_status(boma, *WEAPON_GANON_VOLLEY_SHIELD_KIND_BODY, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
        else {
            let effect_id_1 = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new_raw(0x0c12cb676b), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true);
            let effect_id_2 = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new_raw(0x0be7a40ca7), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true);
            let effect_id_3 = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new_raw(0x0aa78d649a), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true);
            let effect_id_4 = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new_raw(0x080482867e), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true);
            let effect_id_5 = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new_raw(0x0ac11513c9), &Vector3f::zero(), &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true);
            WorkModule::set_int(boma, effect_id_1 as i32, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_ZERO);
            WorkModule::set_int(boma, effect_id_2 as i32, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_ONE);
            WorkModule::set_int(boma, effect_id_3 as i32, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_TWO);
            WorkModule::set_int(boma, effect_id_4 as i32, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_THREE);
            WorkModule::set_int(boma, effect_id_5 as i32, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_FOUR);
        }
        ModelModule::set_scale(boma, 1.0);
        PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(10.0*owner_lr), y: owner_pos_y+14.0, z: owner_pos_z});
        WorkModule::on_flag(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_HAS_ACTIVE_VOLLEY);
    }
    else {
        StatusModule::change_status_force(boma, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, false);
    }
    0.into()
}

unsafe extern "C" fn ganon_volley_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    if WorkModule::is_flag(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED) {
        MotionModule::change_motion(boma, Hash40::new("fly_charge"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);   
    }
    weapon.fastshift(L2CValue::Ptr(ganon_volley_fly_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_volley_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let boma = weapon.module_accessor;
    if should_remove_projectile(weapon) || (GroundModule::is_floor_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0) {
        volley_removal(weapon);
    }
    0.into()
}

unsafe extern "C" fn ganon_volley_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn ganon_volley_fly_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("ganon_volley"), true, true);
    STOP_SE(weapon, Hash40::new("se_ganon_special_n05"));
    STOP_SE(weapon, Hash40::new("se_ganon_special_n06"));
    WorkModule::off_flag(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLAG_CHARGED);
    WorkModule::set_float(boma, 1.0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_DAMAGE_SCALE);
    WorkModule::set_float(boma, 1.0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_SCALE);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_TOTAL_HIT_COUNT);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FIRST_HIT_ID);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_SECOND_HIT_ID);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_THIRD_HIT_ID);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FOURTH_HIT_ID);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_ZERO);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_ONE);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_TWO);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_THREE);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_FOUR);
    0.into()
}

unsafe extern "C" fn ganon_volley_fly_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("ganon_volley"), true, true);
    STOP_SE(weapon, Hash40::new("se_ganon_special_n05"));
    STOP_SE(weapon, Hash40::new("se_ganon_special_n06"));
    WorkModule::off_flag(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLAG_CHARGED);
    WorkModule::set_float(boma, 1.0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_DAMAGE_SCALE);
    WorkModule::set_float(boma, 1.0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_SCALE);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_TOTAL_HIT_COUNT);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FIRST_HIT_ID);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_SECOND_HIT_ID);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_THIRD_HIT_ID);
    WorkModule::set_int(boma, -1, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FOURTH_HIT_ID);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_ZERO);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_ONE);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_TWO);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_THREE);
    WorkModule::set_int(boma, 0, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_EFFECT_ID_FOUR);
    0.into()
}

pub fn install() {
    Agent::new("ganon_cannonballcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_GANON_VOLLEY_STATUS_KIND_FLY, ganon_volley_fly_pre_status)
    .status(Init, *WEAPON_GANON_VOLLEY_STATUS_KIND_FLY, ganon_volley_fly_init_status)
    .status(Main, *WEAPON_GANON_VOLLEY_STATUS_KIND_FLY, ganon_volley_fly_main_status)
    .status(Exec, *WEAPON_GANON_VOLLEY_STATUS_KIND_FLY, ganon_volley_fly_exec_status)
    .status(End, *WEAPON_GANON_VOLLEY_STATUS_KIND_FLY, ganon_volley_fly_end_status)
    .status(End, *WEAPON_GANON_VOLLEY_STATUS_KIND_FLY, ganon_volley_fly_exit_status)
    .install()
    ;
}