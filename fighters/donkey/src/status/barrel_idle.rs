use super::*;

unsafe extern "C" fn donkey_barrel_idle_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.03);
    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -3.0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("rotx"), &Vector3f{x: 0.0, y: 3.5, z: 0.0}, false, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_idle_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_idle_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("rotx"), &Vector3f{x: 0.0, y: 3.5, z: 0.0}, false, false);
    if should_remove_barrel(weapon) {
        remove_barrel(weapon);
    }
    if life <= 40 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_IDLE, donkey_barrel_idle_exit_status)
    .install()
    ;
}