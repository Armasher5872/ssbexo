use super::*;

pub unsafe extern "C" fn is_barrel(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_KOOPAJR_CANNONBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_DONKEY {
            return true;
        }
    }
    return false;
}

pub unsafe extern "C" fn should_remove_barrel(weapon: &mut L2CWeaponCommon) -> bool {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let dead_range = dead_range(weapon.lua_state_agent);
    let remove_range = pos_x < dead_range.x || pos_x > dead_range.y || pos_y > dead_range.z || pos_y < dead_range.w;
    if life <= 0 || remove_range {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn barrel_unlink(weapon: &mut L2CWeaponCommon) {
    GroundModule::set_ignore_boss(weapon.module_accessor, false);
    GroundModule::set_passable_check(weapon.module_accessor, true);
    GroundModule::set_collidable(weapon.module_accessor, true);
    JostleModule::set_status(weapon.module_accessor, true);
}

pub unsafe extern "C" fn barrel_removal(weapon: &mut L2CWeaponCommon) {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::off_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
    WorkModule::off_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_DID_ATTACK);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_THROW_ANGLE);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    barrel_unlink(weapon);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}