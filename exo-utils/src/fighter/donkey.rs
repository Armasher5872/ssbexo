use super::*;

pub unsafe extern "C" fn donkey_var(boma: *mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_UNLINK);
}

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

pub unsafe extern "C" fn barrel_rot(boma: *mut BattleObjectModuleAccessor) {
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut joint_rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_rotation(boma, Hash40::new("rotx"), &mut joint_rot, false);
    let abs_horizontal_speed = speed_x.abs();
    let rot_speed_rad = abs_horizontal_speed/7.5;
    let rot_speed_deg = rot_speed_rad.to_degrees();
    let model_rot = Vector3f{x: joint_rot.x+rot_speed_deg, y: 0.0, z: 0.0};
    ModelModule::set_joint_rotate(boma, Hash40::new("rotx"), &model_rot, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
}

pub unsafe extern "C" fn should_remove_barrel(weapon: &mut L2CWeaponCommon) -> bool {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let dead_range = dead_range(weapon.lua_state_agent);
    let remove_range = pos_x < dead_range.x || pos_x > dead_range.y || pos_y > dead_range.z || pos_y < dead_range.w;
    if life <= 0 || remove_range {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn remove_barrel(weapon: &mut L2CWeaponCommon) {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}