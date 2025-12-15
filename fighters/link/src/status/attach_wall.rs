use super::*;

unsafe extern "C" fn link_attach_wall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attach_wall"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
    if !StopModule::is_stop(fighter.module_accessor) {
        link_attach_wall_sub_status(fighter);
    }
    VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(link_attach_wall_sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(link_attach_wall_main_loop as *const () as _))
}

unsafe extern "C" fn link_attach_wall_sub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
    let dir = stick_y.signum();
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
    }
    if stick_x.abs() >= wall_jump_stick_x && stick_x.signum() == lr {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into()
    }
    if stick_y.abs() <= 0.25 {
        if motion_kind != hash40("attach_wall") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attach_wall"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        if motion_kind != hash40("attach_wall_climb") {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attach_wall_climb"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if dir < 0.0 && current_frame <= 0.0 {
            MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), false);
        }
        MotionModule::set_rate(fighter.module_accessor, dir*0.2);
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.25*dir, z:0.0});
    }
    0.into()
}

unsafe extern "C" fn link_attach_wall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let attach_side = if 0.0 <= lr {*GROUND_TOUCH_FLAG_LEFT} else {*GROUND_TOUCH_FLAG_RIGHT};
    let remove_attach = !GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_side));
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cliff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    let stamina = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA)-1;
    let cliff_max_count = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_max_count"));
    let sweat_rate = 10.0;
	let sweat_size = 0.35;
	let modulo = stamina as f32 % sweat_rate;
    if GroundModule::can_entry_cliff(fighter.module_accessor) != 0 || fighter.sub_transition_group_check_air_cliff().get_bool() || remove_attach {
        if cliff_count < cliff_max_count {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
            fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(), true.into());
            return 1.into();
        }
        else{
            WorkModule::set_int(fighter.module_accessor, 300, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
        }
    }
    if motion_kind == hash40("attach_wall_climb") {
        WorkModule::sub_int(fighter.module_accessor, 2, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    }
    else {
        WorkModule::sub_int(fighter.module_accessor, 1, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    }
    if stamina <= 0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(), false.into());
        return 1.into();
    }
    else if stamina < 90 {
        if stamina == 45 {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 7.0, 3.0, 0, 0, 0, 1.0, true);
        }
		if modulo < 1.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sweat"), Hash40::new("top"), 0, 14.5, 3.0, 0, 0, 0, sweat_size, true);
		}
    }
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACH_WALL, link_attach_wall_main_status)
    .install()
    ;
}