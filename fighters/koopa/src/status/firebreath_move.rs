use super::*;

unsafe extern "C" fn koopa_firebreath_move_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, 0, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_param_float(boma, hash40("param_breath"), hash40("life")) as i32;
    let speed_max = WorkModule::get_param_float(boma, hash40("param_breath"), hash40("max_speed"));
    let lr = PostureModule::lr(boma);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_scale(boma, 1.0, false);
    if !StopModule::is_stop(boma) {
        koopa_firebreath_move_substatus(weapon, false.into());
    }
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(koopa_firebreath_move_substatus as *const () as _));
    MotionModule::change_motion(boma, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(koopa_firebreath_move_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_firebreath_move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    let boma = weapon.module_accessor;
    if param_1.get_bool() {
        WorkModule::dec_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = PostureModule::pos(boma);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)
    || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_LEFT as u32) || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) 
    || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_UP as u32) || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(boma, Hash40::new("sys_bomb_b"), pos, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        SoundModule::play_se(boma, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    EffectModule::detach_all(boma, 5);
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