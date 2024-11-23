use super::*;

unsafe extern "C" fn mariod_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 1);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_Attack()
}

unsafe extern "C" fn mariod_attack_s3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 1);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackS3()
}

unsafe extern "C" fn mariod_attack_hi3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 1);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackHi3()
}

unsafe extern "C" fn mariod_attack_lw3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 1);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackLw3()
}

unsafe extern "C" fn mariod_attack_dash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 1);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackDash()
}

unsafe extern "C" fn mariod_attack_s4_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 2);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackS4Start()
}

unsafe extern "C" fn mariod_attack_hi4_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 2);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackHi4Start()
}

unsafe extern "C" fn mariod_attack_lw4_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 2);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackLw4Start()
}

unsafe extern "C" fn mariod_attack_air_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 3);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_AttackAir()
}

unsafe extern "C" fn mariod_catch_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 3);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_CatchAttack()
}

unsafe extern "C" fn mariod_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 3);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.status_pre_Throw()
}

unsafe extern "C" fn mariod_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE) {
        let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE);
        let capsule_life = WorkModule::get_int(article_boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if capsule_life <= 0 {
            UiManager::set_mariod_meter_info(entry_id, 0);
            WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
        }
    }
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let pill_id = WorkModule::get_int(owner_boma, FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_drcapsule"), hash40("speed"));
    let gravity_acl_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_drcapsule"), hash40("gravity_acl_max"));
    let gravity_accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_drcapsule"), hash40("gravity_accel"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let mut angle: f32 = 0.0;
    if pill_id == 0 {
        angle = 45.0;
    }
    if pill_id == 1 {
        angle = -55.0;
    }
    if pill_id == 2 {
        angle = 85.0;
    }
    if pill_id == 3 {
        angle = 25.0;
    }
    let speed_x = angle.to_radians().sin()*speed*lr;
    let speed_y = angle.to_radians().cos()*speed;
    if pill_id == 1 {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -speed_x, -2.0);
    }
    else if pill_id == 2 {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x/4.0, 3.5);
    }
    else if pill_id == 3 {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*1.5, speed_y/2.0);
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    }
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity_accel);
    sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed, gravity_acl_max);
    0.into()
}

unsafe extern "C" fn mariod_drcapsule_regular_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_entry_id = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(owner_entry_id, 0);
    WorkModule::set_int(owner_boma, UiManager::get_mariod_pill_id(owner_entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    0.into()
}

unsafe extern "C" fn mariod_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn mariod_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn mariod_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(fighter.module_accessor, UiManager::get_mariod_pill_id(entry_id), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("mariod")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK, mariod_attack_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, mariod_attack_s3_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI3, mariod_attack_hi3_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW3, mariod_attack_lw3_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_DASH, mariod_attack_dash_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_START, mariod_attack_s4_start_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, mariod_attack_hi4_start_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, mariod_attack_lw4_start_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, mariod_attack_air_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_ATTACK, mariod_catch_attack_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_THROW, mariod_throw_pre_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, mariod_special_n_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, mariod_special_s_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, mariod_special_hi_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, mariod_special_lw_pre_status)
    .install()
    ;
    Agent::new("mariod_drcapsule")
    .status(Pre, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_pre_status)
    .status(Init, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_init_status)
    .status(End, *WEAPON_MARIOD_DRCAPSULE_STATUS_KIND_REGULAR, mariod_drcapsule_regular_end_status)
    .install()
    ;
}