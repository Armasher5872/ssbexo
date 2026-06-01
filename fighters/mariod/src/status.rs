use super::*;

unsafe extern "C" fn mariod_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    if ArticleModule::is_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE) {
        let article_boma = get_article_boma(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE);
        let capsule_life = WorkModule::get_int(article_boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if capsule_life <= 0 {
            UiManager::set_mariod_meter_info(entry_id, 0);
            WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
        }
    }
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let pill_id = WorkModule::get_int(owner_boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    let speed = WorkModule::get_param_float(boma, hash40("param_drcapsule"), hash40("speed"));
    let gravity_acl_max = WorkModule::get_param_float(boma, hash40("param_drcapsule"), hash40("gravity_acl_max"));
    let gravity_accel = WorkModule::get_param_float(boma, hash40("param_drcapsule"), hash40("gravity_accel"));
    let lr = PostureModule::lr(boma);
    let angle: f32 = 45.0;
    let speed_x = angle.to_radians().sin()*speed*lr;
    let speed_y = angle.to_radians().cos()*speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
    sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed, gravity_acl_max);
    if pill_id == 1 {
        MotionModule::set_frame_material(boma, 0.0, MaterialAnimeKind{_address: 0});
    }
    if pill_id == 2 {
        MotionModule::set_frame_material(boma, 8.0, MaterialAnimeKind{_address: 0});
    }
    if pill_id == 3 {
        MotionModule::set_frame_material(boma, 4.0, MaterialAnimeKind{_address: 0});
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
    Agent::new("mariod")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, mariod_special_n_end_status)
    .install()
    ;
    Agent::new("mariod_drcapsule")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_pre_status)
    .status(Init, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_init_status)
    .status(End, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_end_status)
    .install()
    ;
}