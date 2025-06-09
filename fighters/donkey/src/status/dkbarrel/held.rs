use super::*;

//Barrel Held Pre Status
unsafe extern "C" fn donkey_barrel_held_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

//Barrel Held Init Status
unsafe extern "C" fn donkey_barrel_held_init_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

//Barrel Held Main Status
unsafe extern "C" fn donkey_barrel_held_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("held"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_held_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_held_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
    if !WorkModule::is_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED) {
        barrel_unlink(weapon);
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_THROW.into(), false.into());
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("held"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

//Barrel Held Exec Status
unsafe extern "C" fn donkey_barrel_held_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Held End Status
unsafe extern "C" fn donkey_barrel_held_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    if !WorkModule::is_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED) || StatusModule::status_kind(weapon.module_accessor) != *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD {
        barrel_unlink(weapon);
    }
    0.into()
}

//Barrel Held Exit Status
unsafe extern "C" fn donkey_barrel_held_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    if !WorkModule::is_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED) || StatusModule::status_kind(weapon.module_accessor) != *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD {
        barrel_unlink(weapon);
    }
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, donkey_barrel_held_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, donkey_barrel_held_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, donkey_barrel_held_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, donkey_barrel_held_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, donkey_barrel_held_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, donkey_barrel_held_exit_status)
    .install()
    ;
}