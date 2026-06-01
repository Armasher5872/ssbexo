use super::*;

unsafe extern "C" fn ike_slash_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let life = WorkModule::get_param_int(boma, hash40("param_slash"), hash40("life"));
    let speed_max = WorkModule::get_param_float(boma, hash40("param_slash"), hash40("speed_max"));
    let lr = PostureModule::lr(boma);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(16.0*owner_lr), y: owner_pos_y, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(ike_slash_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn ike_slash_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let boma = weapon.module_accessor;
    if GroundModule::is_floor_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(boma, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
    }
    if should_remove_projectile(weapon) {
        slash_removal(weapon);
    }
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn ike_slash_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("ike_slash"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("ike_slash")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_pre_status)
    .status(Init, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_init_status)
    .status(Main, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_main_status)
    .status(Exec, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_exec_status)
    .status(End, *WEAPON_IKE_SLASH_STATUS_KIND_SHOOT, ike_slash_shoot_end_status)
    .install()
    ;
}