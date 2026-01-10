use super::*;

unsafe extern "C" fn metaknight_beam_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn metaknight_beam_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_beam"), hash40("life"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_beam"), hash40("speed_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(11.0*owner_lr), y: owner_pos_y, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn metaknight_beam_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(metaknight_beam_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_beam_shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
    }
    if should_remove_projectile(weapon) {
        beam_removal(weapon);
    }
    0.into()
}

unsafe extern "C" fn metaknight_beam_shoot_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn metaknight_beam_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("metaknight_beam"), false, false);
    WorkModule::set_float(owner_boma, 0.0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
    0.into()
}

pub fn install() {
    Agent::new("metaknight_beam")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_METAKNIGHT_BEAM_STATUS_KIND_SHOOT, metaknight_beam_shoot_pre_status)
    .status(Init, *WEAPON_METAKNIGHT_BEAM_STATUS_KIND_SHOOT, metaknight_beam_shoot_init_status)
    .status(Main, *WEAPON_METAKNIGHT_BEAM_STATUS_KIND_SHOOT, metaknight_beam_shoot_main_status)
    .status(Exec, *WEAPON_METAKNIGHT_BEAM_STATUS_KIND_SHOOT, metaknight_beam_shoot_exec_status)
    .status(End, *WEAPON_METAKNIGHT_BEAM_STATUS_KIND_SHOOT, metaknight_beam_shoot_end_status)
    .install()
    ;
}