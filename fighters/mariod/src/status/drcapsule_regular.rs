use super::*;

unsafe extern "C" fn mariod_drcapsule_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let pill_id = WorkModule::get_int(owner_boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    let lr = PostureModule::lr(boma);
    if pill_id == 1 /*Red*/ {
        let life = 60;
        let speed_x = 2.4*lr;
        let speed_y = 0.0;
        let gravity_accel = 0.0;
        let gravity_acl_max = 0.0;
        MotionModule::set_frame_material(boma, 0.0, MaterialAnimeKind{_address: 0});
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, gravity_acl_max);
    }
    else if pill_id == 2 /*Yellow*/ {
        let life = 55;
        let angle: f32 = 86.0;
        let speed_x = angle.to_radians().sin()*0.4*lr;
        let speed_y = angle.to_radians().cos()*4.8;
        let gravity_accel = 0.2;
        let gravity_acl_max = 4.0;
        MotionModule::set_frame_material(boma, 8.0, MaterialAnimeKind{_address: 0});
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, gravity_acl_max);
    }
    else if pill_id == 3 /*Blue*/ {
        let life = 240;
        let angle: f32 = 72.0;
        let speed_x = angle.to_radians().sin()*0.05*lr;
        let speed_y = angle.to_radians().cos()*0.2;
        let gravity_accel = 0.004;
        let gravity_acl_max = 0.5;
        MotionModule::set_frame_material(boma, 4.0, MaterialAnimeKind{_address: 0});
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, gravity_acl_max);
    }
    else /*White*/ {
        let life = WorkModule::get_param_int(boma, hash40("param_drcapsule"), hash40("life"));
        let speed = WorkModule::get_param_float(boma, hash40("param_drcapsule"), hash40("speed"));
        let angle: f32 = 45.0;
        let speed_x = angle.to_radians().sin()*speed*lr;
        let speed_y = angle.to_radians().cos()*speed;
        let gravity_accel = WorkModule::get_param_float(boma, hash40("param_drcapsule"), hash40("gravity_accel"));
        let gravity_acl_max = WorkModule::get_param_float(boma, hash40("param_drcapsule"), hash40("gravity_acl_max"));
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed, gravity_acl_max);
    }
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_entry_id = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(owner_entry_id, 0);
    WorkModule::set_int(owner_boma, UiManager::get_mariod_pill_id(owner_entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    0.into()
}

pub fn install() {
    Agent::new("mariod_drcapsule")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_pre_status)
    .status(Init, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_init_status)
    .status(End, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_end_status)
    .install()
    ;
}