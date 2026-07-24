use super::*;

unsafe extern "C" fn edge_zanshin_shot_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let life = WorkModule::get_param_int(boma, hash40("param_swordbeamcloned"), hash40("life"));
    let speed = WorkModule::get_param_float(boma, hash40("param_swordbeamcloned"), hash40("speed"));
    let lr = PostureModule::lr(boma);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(11.0*owner_lr), y: owner_pos_y+6.0, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(edge_zanshin_shot_fly_main_loop as *const () as _))
}

unsafe extern "C" fn edge_zanshin_shot_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_i32();
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life > 0 {
        if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_ALL as u32) && current_frame > 1 {
            weapon.change_status(WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_VANISH.into(), false.into());
        }
    }
    else {
        weapon.change_status(WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_VANISH.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_fly_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge_swordbeamcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_FLY, edge_zanshin_shot_fly_pre_status)
    .status(Init, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_FLY, edge_zanshin_shot_fly_init_status)
    .status(Main, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_FLY, edge_zanshin_shot_fly_main_status)
    .status(Exec, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_FLY, edge_zanshin_shot_fly_exec_status)
    .status(End, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_FLY, edge_zanshin_shot_fly_end_status)
    .install()
    ;
}