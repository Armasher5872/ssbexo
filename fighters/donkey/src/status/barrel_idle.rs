use super::*;

unsafe extern "C" fn donkey_barrel_idle_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let boma = weapon.module_accessor;
    let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let gravity = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("gravity"));
    if situation_kind == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, speed_y);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
        sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
        weapon.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    ReflectorModule::set_status_all(boma, ShieldStatus(*SHIELD_STATUS_NORMAL_GLOBAL), 0);
    ReflectorModule::set_no_team(boma, true);
    MotionModule::change_motion(boma, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_idle_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_idle_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = weapon.module_accessor;
    let gravity = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("gravity"));
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    if should_remove_barrel(weapon) {
        remove_barrel(weapon);
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            weapon.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
            sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
            weapon.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    if life <= 40 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_idle_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_cannonballcloned")
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