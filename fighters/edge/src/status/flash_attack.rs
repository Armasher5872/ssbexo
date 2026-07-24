use super::*;

unsafe extern "C" fn edge_flash_attack_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let opponent_id = WorkModule::get_int(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    WorkModule::set_int(boma, opponent_id, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_INT_OPPONENT_OBJECT_ID);
    0.into()
}

unsafe extern "C" fn edge_flash_attack_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let opponent_id = WorkModule::get_int(boma, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_INT_OPPONENT_OBJECT_ID);
    if opponent_id != *BATTLE_OBJECT_ID_INVALID {
        let opponent_battle_object = get_battle_object_from_id(opponent_id as u32);
        let opponent_boma = (*opponent_battle_object).module_accessor;
        PostureModule::set_pos(boma, &Vector3f{x: PostureModule::pos_x(opponent_boma), y: PostureModule::pos_y(opponent_boma), z: PostureModule::pos_z(opponent_boma)});
    }
    MotionModule::change_motion(boma, Hash40::new("attack"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(edge_flash_attack_main_loop as *const () as _))
}

unsafe extern "C" fn edge_flash_attack_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn edge_flash_attack_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let effect_scale_no_hit = WorkModule::get_param_float(boma, hash40("param_flash"), hash40("effect_scale_no_hit"));
    let effect_id = WorkModule::get_int(boma, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_EFFECT_ID);
    if EffectModule::is_exist_effect(boma, effect_id as u32) {
        EffectModule::set_scale(boma, effect_id as u32, &Vector3f{x: effect_scale_no_hit, y: effect_scale_no_hit, z: effect_scale_no_hit});
    }
    0.into()
}

unsafe extern "C" fn edge_flash_attack_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    WorkModule::set_float(boma, 0.0, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_FLOAT_OPPONENT_POS_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_FLOAT_OPPONENT_POS_Y);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_INT_OPPONENT_OBJECT_ID);
    WorkModule::set_int(owner_boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    WorkModule::set_int(owner_boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE_TIMER);
    0.into()
}

pub fn install() {
    Agent::new("edge_flash")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, edge_flash_attack_init_status)
    .status(Main, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, edge_flash_attack_main_status)
    .status(Exec, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, edge_flash_attack_exec_status)
    .status(End, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, edge_flash_attack_end_status)
    .install()
    ;
}