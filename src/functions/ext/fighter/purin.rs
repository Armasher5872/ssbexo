use super::*;

//Purin Disarming Voice Functions

pub unsafe extern "C" fn is_disarming_voice(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_KOOPAJR_CANNONBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_PURIN {
            return true;
        }
    }
    return false;
}

pub unsafe extern "C" fn should_remove_disarming_voice_on_hit(weapon: &mut L2CWeaponCommon) -> bool {
    if AttackModule::is_infliction_status(weapon.module_accessor, WEAPON_PURIN_DISARMING_VOICE_STATUS_KIND_SHOOT)
    || StopModule::is_stop(weapon.module_accessor)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn disarming_voice_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("rosetta_ring_erase"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

pub unsafe extern "C" fn disarming_voice_hit_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_flash"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("poke_meloetta_bullet"), false, false);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("rosetta_ring_erase"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}