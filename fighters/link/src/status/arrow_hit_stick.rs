use super::*;

unsafe extern "C" fn link_bowarrow_hit_stick_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x220cea5125));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("hit_stick"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        link_bowarrow_hit_stick_sub_status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(link_bowarrow_hit_stick_sub_status as *const () as _));
    fun_710002bc00(weapon);
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        WorkModule::off_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_INIT_FUSE);
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_boma, status, false);
        }
    }
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_hit_stick_main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_hit_stick_sub_status(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if bool_check.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if life <= 0 {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 0.into();
        }
        fun_710002bc00(weapon);
    }
    0.into()
}

unsafe extern "C" fn fun_710002bc00(weapon: &mut L2CWeaponCommon) {
    let stick_x = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_STICK_X);
    let stick_y = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_STICK_Y);
    let stick_z = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_STICK_Z);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("root"), &Vector3f{x: stick_x, y: stick_y, z: stick_z}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
}

unsafe extern "C" fn link_bowarrow_hit_stick_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    if current_frame == 4.0 {
        AttackModule::clear_all(weapon.module_accessor);
    }
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK, link_bowarrow_hit_stick_main_status)
    .install()
    ;
}