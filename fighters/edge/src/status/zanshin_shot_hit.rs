use super::*;

unsafe extern "C" fn edge_zanshin_shot_hit_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_hit_init_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_hit_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("hit"), 0.0, 1.0, false, 0.0, false, false);
    EffectModule::detach_all(boma, 5);
    weapon.fastshift(L2CValue::Ptr(edge_zanshin_shot_hit_main_loop as *const () as _))
}

unsafe extern "C" fn edge_zanshin_shot_hit_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    if 10.0 < frame {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_hit_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn edge_zanshin_shot_hit_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("edge_swordbeamcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_HIT, edge_zanshin_shot_hit_pre_status)
    .status(Init, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_HIT, edge_zanshin_shot_hit_init_status)
    .status(Main, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_HIT, edge_zanshin_shot_hit_main_status)
    .status(Exec, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_HIT, edge_zanshin_shot_hit_exec_status)
    .status(End, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_HIT, edge_zanshin_shot_hit_end_status)
    .install()
    ;
}