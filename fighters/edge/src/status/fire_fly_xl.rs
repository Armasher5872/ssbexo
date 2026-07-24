use super::*;

unsafe extern "C" fn edge_fire_xl_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn edge_fire_xl_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.0035);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.55);
    PostureModule::set_pos(boma, &Vector3f{x: PostureModule::pos_x(owner_boma)+(6.0*PostureModule::lr(owner_boma)), y: PostureModule::pos_y(owner_boma)+10.0, z: PostureModule::pos_z(owner_boma)});
    WorkModule::set_int(boma, 100, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(boma, Hash40::new("special_n4"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(edge_fire_xl_fly_main_loop as *const () as _))
}

unsafe extern "C" fn edge_fire_xl_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() && weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        StopModule::set_other_stop(boma, 2, StopOtherKind(0));
    }
    if life <= 0 {
        weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_BURST_XL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn edge_fire_xl_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn edge_fire_xl_fly_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_XL, edge_fire_xl_fly_pre_status)
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_XL, edge_fire_xl_fly_main_status)
    .status(Exec, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_XL, edge_fire_xl_fly_exec_status)
    .status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_XL, edge_fire_xl_fly_end_status)
    .install()
    ;
}