use super::*;

unsafe extern "C" fn luigi_obakyumu_special_lw_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_special_lw_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    WorkModule::set_int(weapon.module_accessor, 300, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 300, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    if owner_lr == -1.0 {
        MotionModule::set_flip(weapon.module_accessor, false, true, true);
    }
    weapon.fastshift(L2CValue::Ptr(luigi_obakyumu_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_obakyumu_special_lw_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos = *PostureModule::pos(weapon.module_accessor);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y+8.0, z: pos.z}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn luigi_obakyumu_special_lw_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let status_kind = weapon.global_table[STATUS_KIND].get_i32();
    let owner_boma = get_owner_boma(weapon);
    let object_id = WorkModule::get_int(owner_boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    if ![
        *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_START, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_LOOP, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_END,
        *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_PLUNGER, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_PULL, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_WAIT,
        *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_JUMP, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_TURN, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_CATCH_WALK, 
        *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_THROW
    ].contains(&status_kind) {
        ArticleModule::remove_exist_object_id(owner_boma, object_id as u32);
        ArticleModule::remove_exist(owner_boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi_obakyumu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW, luigi_obakyumu_special_lw_pre_status)
    .status(Main, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW, luigi_obakyumu_special_lw_main_status)
    .status(End, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW, luigi_obakyumu_special_lw_end_status)
    .install()
    ;
}