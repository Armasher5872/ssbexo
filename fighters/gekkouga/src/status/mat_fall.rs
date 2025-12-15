use super::*;

unsafe extern "C" fn gekkouga_mat_fall_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_situation_kind = StatusModule::situation_kind(owner_boma);
    if !StatusModule::is_changing(owner_boma) || !StatusModule::is_situation_changed(owner_boma) {
        if owner_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            weapon.set_situation(SITUATION_KIND_AIR.into());
        }
        else {
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            weapon.set_situation(SITUATION_KIND_GROUND.into());
        }
    }
    WorkModule::set_int(weapon.module_accessor, 45, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 45, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    weapon.fastshift(L2CValue::Ptr(gekkouga_mat_fall_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_mat_fall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    if should_remove_projectile(weapon) || (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR) {
        mat_removal(weapon);
    }
    if motion_kind == hash40("special_air_lw_start") && frame == 28.0 {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    }
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("gekkouga_mat")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_pre_status)
    .status(Init, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_init_status)
    .status(Main, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_main_status)
    .status(Exec, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_exec_status)
    .status(End, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_end_status)
    .install()
    ;
}