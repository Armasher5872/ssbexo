use super::*;

unsafe extern "C" fn inkling_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn inkling_splashbomb_explode_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_agent = get_fighter_common_from_accessor(&mut *owner_boma);
    let owner_fighter = owner_agent.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let r = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_R);
    let g = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_G);
    let b = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_B);
    let pos = PostureModule::pos(weapon.module_accessor);
    let get_touch_pos = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let get_touch_normal = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let get_ink_work_id = WorkModule::get_float(owner_boma, FighterSpecializer_Inkling::get_ink_work_id(owner_fighter, 0));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, true, 0.0, false, false);
    VisibilityModule::set_whole(weapon.module_accessor, false);
    EffectModule::detach_all(weapon.module_accessor, 5);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new_raw(0x18c19c7ee8), &Vector3f{x: get_touch_pos.x, y: get_touch_pos.y, z: (*pos).z}, &Vector3f{x: (get_touch_normal.x).atan(), y: (get_touch_normal.y).atan().to_degrees(), z: (*pos).z}, 1.0, 0u32, -1, false, 0);
        EffectModule::set_rgb_partial_last(weapon.module_accessor, r, g, b);
        if get_ink_work_id > 0.0 {
            FighterSpecializer_Inkling::generate_rollerink(owner_fighter);
            FighterSpecializer_Inkling::request_paint(owner_fighter, Hash40::new("top"), &Vector3f{x: 0.0, y: 0.01, z: 0.0}, &Vector2f{x: 0.0, y: 0.01}, 1.0);
        }
    }
    ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    weapon.fastshift(L2CValue::Ptr(inkling_splashbomb_explode_main_loop as *const () as _))
}

unsafe extern "C" fn inkling_splashbomb_explode_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let float_attack = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_ATTACK);
    let float_ink = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_INK);
    AttackModule::set_power(weapon.module_accessor, 0, float_attack, false);
    AttackModule::set_ink_value(weapon.module_accessor, 0, float_ink);
    if 1.0 < current_frame {
        if !AttackModule::is_attack(weapon.module_accessor, 0, false) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 0.into();
        }
    }
    if current_frame >= 20.0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("inkling")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, inkling_special_s_pre_status)
    .install()
    ;
    Agent::new("inkling_splashbomb")
    .status(Main, *WEAPON_INKLING_SPLASHBOMB_STATUS_KIND_EXPLODE, inkling_splashbomb_explode_main_status)
    .install()
    ;
}