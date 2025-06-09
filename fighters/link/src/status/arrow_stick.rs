use super::*;

unsafe extern "C" fn link_bowarrow_stick_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    let weapon_rand = sv_math::randf(hash40("weapon"), 1.0);
    WorkModule::set_float(weapon.module_accessor, (weapon_rand*4.0)-2.0, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_STICK_ANGLE_RAND);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x220cea5125));
    GroundModule::attach(weapon.module_accessor, GroundTouchFlag(*GROUND_TOUCH_FLAG_ALL));
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stick"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        link_bowarrow_stick_sub_status(weapon, false.into());
    }
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_boma, status, false);
        }
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(link_bowarrow_stick_sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_stick_main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_stick_sub_status(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    let kind = weapon.global_table[FIGHTER_KIND].get_i32();
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let is_attach = GroundModule::is_attach(weapon.module_accessor);
    let stay_count = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_STATUS_TURN_WORK_INT_STAY_COUNT);
    if bool_check.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOWARROW_STATUS_TURN_WORK_INT_STAY_COUNT);
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if stay_count <= 0 {
            if kind == *WEAPON_KIND_LINK_BOWARROW {
                to_item(weapon.module_accessor);
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
        if life <= 0 {
            if !is_attach {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
    }
    0.into()
}

unsafe extern "C" fn link_bowarrow_stick_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let weapon_rand = sv_math::randf(hash40("weapon"), 1.0);
    if current_frame == 4.0 {
        AttackModule::clear_all(weapon.module_accessor);
    }
    WorkModule::set_float(weapon.module_accessor, weapon_rand, *WN_LINK_BOWARROW_STATUS_STICK_WORK_FLOAT_RAND);
    0.into()
}

unsafe extern "C" fn link_bowarrow_stick_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        let team_no = if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::get_int(owner_boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
            WorkModule::get_int(owner_boma, *FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else {
            WorkModule::get_int(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO)
        };
        TeamModule::set_team(weapon.module_accessor, team_no, true);
        TeamModule::set_team_owner_id(weapon.module_accessor, (*(owner_boma)).battle_object_id);
        WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
        WorkModule::off_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_INIT_FUSE);
    }
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_STICK, link_bowarrow_stick_main_status)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_STICK, link_bowarrow_stick_end_status)
    .install()
    ;
}