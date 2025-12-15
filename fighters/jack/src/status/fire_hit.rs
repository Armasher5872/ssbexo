use super::*;

unsafe extern "C" fn jack_fire_hit_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    /*
    let module_accessor = weapon.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let gravity_angle = smash::app::SlopeModuleSimple::gravity_angle(module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    let param_id = WorkModule::get_int64(weapon.module_accessor, *WEAPON_JACK_FIRE_INSTANCE_WORK_ID_INT_PARAM_ID);
    let hit_life = WorkModule::get_param_int(weapon.module_accessor, param_id, hash40("hit_life"));
    WorkModule::set_int(weapon.module_accessor, hit_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if !StopModule::is_stop(weapon.module_accessor) {
        fun_710003b9a0(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710003b9a0 as *const () as _));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("hit"), 0.0, 1.0, false, 0.0, false, false);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: gravity_angle*lr, y: 0.0, z: 0.0}, 0);
    WorkModule::add_float(get_owner_boma(weapon), 1.0, 0x4D);
    weapon.fastshift(L2CValue::Ptr(jack_fire_hit_main_loop as *const () as _))
    */
    let ret = original_status(Main, weapon, *WEAPON_JACK_FIRE_STATUS_KIND_HIT)(weapon);
    WorkModule::add_float(get_owner_boma(weapon), 1.0, 0x4D);
    ret
}

/*
unsafe extern "C" fn fun_710003b9a0(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn jack_fire_hit_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}
*/

pub fn install() {
    Agent::new("jack_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_JACK_FIRE_STATUS_KIND_HIT, jack_fire_hit_main_status)
    .install()
    ;
}