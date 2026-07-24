use super::*;

unsafe extern "C" fn edge_fire_xl_burst_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn edge_fire_xl_burst_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    GroundModule::set_collidable(boma, false);
    MotionModule::change_motion(boma, Hash40::new("burst_xl"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    weapon.fastshift(L2CValue::Ptr(edge_fire_xl_burst_main_loop as *const () as _))
}

unsafe extern "C" fn edge_fire_xl_burst_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let module_accessor = weapon.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let current_frame = weapon.global_table[CURRENT_FRAME].get_i32();
    let boma = weapon.module_accessor;
    let effect_check_frame_burst = WorkModule::get_param_int(boma, hash40("param_fire"), hash40("effect_check_frame_burst"));
    if current_frame == effect_check_frame_burst {
        weapon_specializer_edge_fire_request_effect(module_accessor);
    }
    0.into()
}

unsafe extern "C" fn edge_fire_xl_burst_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_XL, edge_fire_xl_burst_pre_status)
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_XL, edge_fire_xl_burst_main_status)
    .status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_BURST_XL, edge_fire_xl_burst_end_status)
    .install()
    ;
}