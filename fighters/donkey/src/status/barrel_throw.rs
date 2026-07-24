use super::*;

unsafe extern "C" fn donkey_barrel_throw_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_throw_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let owner_situation_kind = StatusModule::situation_kind(owner_boma);
    let speed = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("speed_min"));
    let brake_x = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("brake_x"));
    let gravity = WorkModule::get_param_float(boma, hash40("param_cannonballcloned"), hash40("gravity"));
    let angle = 25.0f32;
    let speed_x = angle.to_radians().sin()*speed*owner_lr;
    let speed_y = angle.to_radians().cos()*speed;
    WorkModule::set_int(boma, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    if owner_situation_kind == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*2.5, 0.0);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (-brake_x)*owner_lr, -gravity);
        sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
        PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(20.0*owner_lr), y: owner_pos_y, z: owner_pos_z});
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*2.5, speed_y*0.5);
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, (-brake_x)*owner_lr, -gravity);
        sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
        PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(12.0*owner_lr), y: owner_pos_y+6.0, z: owner_pos_z});
    }
    ReflectorModule::set_status_all(boma, ShieldStatus(*SHIELD_STATUS_NORMAL_GLOBAL), 0);
    ReflectorModule::set_no_team(boma, true);
    0.into()
}

unsafe extern "C" fn donkey_barrel_throw_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_throw_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_throw_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    barrel_rot(boma);
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            weapon.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            PLAY_SE(weapon, Hash40::new("se_item_barrel_landing"));
            weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL.into(), false.into());
        }
    }
    if should_remove_barrel(weapon) {
        remove_barrel(weapon);
    }
    if life <= 40 {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("roll"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_throw_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn donkey_barrel_throw_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_throw_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_cannonballcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_THROW, donkey_barrel_throw_exit_status)
    .install()
    ;
}