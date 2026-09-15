use super::*;

unsafe extern "C" fn edge_fire_xl_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn edge_fire_xl_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(boma, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_DISABLE_SOUND);
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
    if WorkModule::is_flag(boma, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_DISABLE_SOUND) {
        let ui_sound_mgr = ((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x5328f38) as *const u64).read();
        let idx = play_bgm_override(ui_sound_mgr, hash40("nowhere_to_run"), -1);
        WorkModule::set_int(boma, idx, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_INT_BGM_IDX);
        for object_id in get_all_active_battle_object_ids() {
            let object = get_battle_object_from_id(object_id);
            if object.is_null() { 
                continue; //skip null
            }
            let other_battle_object_id = (*object).battle_object_id;
            let other_boma = (*object).module_accessor;
            let other_category = other_battle_object_id >> 0x1C;
            if other_category == 0 {
                WorkModule::on_flag(other_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SE_MUTE);
                WorkModule::on_flag(other_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_VOICE_MUTE);
            }
            if other_category == 1 {
                WorkModule::on_flag(other_boma, *WEAPON_INSTANCE_WORK_ID_FLAG_SOUND_MUTE);
            }
        }
        WorkModule::off_flag(boma, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_DISABLE_SOUND);
    }
    0.into()
}

unsafe extern "C" fn edge_fire_xl_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn edge_fire_xl_fly_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let bgm_idx = WorkModule::get_int(boma, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_INT_BGM_IDX);
    for object_id in get_all_active_battle_object_ids() {
        let object = get_battle_object_from_id(object_id);
        if object.is_null() { 
            continue; //skip null
        }
        let other_battle_object_id = (*object).battle_object_id;
        let other_boma = (*object).module_accessor;
        let other_category = other_battle_object_id >> 0x1C;
        if other_category == 0 {
            WorkModule::off_flag(other_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SE_MUTE);
            WorkModule::off_flag(other_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_VOICE_MUTE);
        }
        if other_category == 1 {
            WorkModule::off_flag(other_boma, *WEAPON_INSTANCE_WORK_ID_FLAG_SOUND_MUTE);
        }
    }
    stop_status_bgm(((boma as u64) + 0x148) as *const u64, bgm_idx);
    WorkModule::set_int(boma, 0, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_INT_BGM_IDX);
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