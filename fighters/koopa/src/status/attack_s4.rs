use super::*;

unsafe extern "C" fn koopa_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_AttackS4(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(koopa_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let special_zoom_gfx = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if special_zoom_gfx > 0 {
        WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if special_zoom_gfx == 2 {
        SlowModule::set_whole(boma, 8, 80);
        CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
    }
    if special_zoom_gfx >= 4 {
        SlowModule::clear_whole(boma);
        CameraModule::reset_all(boma);
        EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
        CAM_ZOOM_OUT(fighter);
    }
    fighter.status_AttackS4_Main()
}

unsafe extern "C" fn koopa_attack_s4_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER && collision_kind == *COLLISION_KIND_HIT {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        }
    }
    0.into()
}

unsafe extern "C" fn koopa_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    0.into()
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, koopa_attack_s4_main_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S4, koopa_attack_s4_check_attack_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, koopa_attack_s4_end_status)
    .install()
    ;
}