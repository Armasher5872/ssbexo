use super::*;

//Barrel Pull Pre Status
unsafe extern "C" fn donkey_barrel_pull_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

//Barrel Pull Init Status
unsafe extern "C" fn donkey_barrel_pull_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_barrel"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_THROW_ANGLE);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
    WorkModule::set_float(weapon.module_accessor, 10.0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_DID_ATTACK);
    0.into()
}

//Barrel Pull Main Status
unsafe extern "C" fn donkey_barrel_pull_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    GroundModule::set_ignore_boss(weapon.module_accessor, true);
    GroundModule::set_passable_check(weapon.module_accessor, false);
    GroundModule::set_collidable(weapon.module_accessor, false);
    JostleModule::set_status(weapon.module_accessor, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("pull"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_pull_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_pull_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let hp = WorkModule::get_float(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    let owner_boma = get_owner_boma(weapon);
    let owner_vector = &mut Vector3f::zero();
    ModelModule::joint_global_position(owner_boma, Hash40::new("throw"), owner_vector, false);
    if should_remove_barrel(weapon) {
        barrel_removal(weapon);
    }
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_vector.x, y: owner_vector.y-6.0, z: owner_vector.z});
    if life <= 40 || hp <= 0.0 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_HELD.into(), false.into());
    }
    0.into()
}

//Barrel Pull Exec Status
unsafe extern "C" fn donkey_barrel_pull_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Pull End Status
unsafe extern "C" fn donkey_barrel_pull_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    if !WorkModule::is_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED) || StatusModule::status_kind(weapon.module_accessor) != *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD {
        barrel_unlink(weapon);
    }
    0.into()
}

//Barrel Pull Exit Status
unsafe extern "C" fn donkey_barrel_pull_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    if !WorkModule::is_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED) || StatusModule::status_kind(weapon.module_accessor) != *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD {
        barrel_unlink(weapon);
    }
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_exit_status)
    .install()
    ;
}