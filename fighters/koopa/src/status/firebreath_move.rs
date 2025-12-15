use super::*;

unsafe extern "C" fn koopa_firebreath_move_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, 0, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life")) as i32;
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_speed"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_scale(weapon.module_accessor, 1.0, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        koopa_firebreath_move_substatus(weapon, false.into());
    }
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(koopa_firebreath_move_substatus as *const () as _));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(koopa_firebreath_move_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_firebreath_move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = PostureModule::pos(weapon.module_accessor);
    let owner_boma = get_owner_boma(weapon);
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) 
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) 
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_bomb_b"), pos, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if life <= 0 {
        WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    EffectModule::detach_all(weapon.module_accessor, 5);
    WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    0.into()
}

pub fn install() {
    Agent::new("koopa_breath")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, koopa_firebreath_move_pre_status)
    .status(Main, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, koopa_firebreath_move_main_status)
    .status(End, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, koopa_firebreath_move_end_status)
    .install()
    ;
}