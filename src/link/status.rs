use super::*;

unsafe extern "C" fn link_attack_dash_bound_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_DASH | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32, 0);
    0.into()
}

unsafe extern "C" fn link_attack_dash_bound_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.7);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    0.into()
}

unsafe extern "C" fn link_attack_dash_bound_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash_bound"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_attack_dash_bound_main_loop as *const () as _))
}

unsafe extern "C" fn link_attack_dash_bound_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn link_attack_dash_bound_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_attack_s_4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    }
    0.into()
}

unsafe extern "C" fn link_attack_s_4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    0.into()
}

unsafe extern "C" fn link_attack_hi_4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    }
    0.into()
}

unsafe extern "C" fn link_attack_hi_4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    0.into()
}

unsafe extern "C" fn link_attack_lw_4_hold_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut log_mask_flag = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR;
    if 0 < WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) {
        log_mask_flag = *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP;
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_HOLD_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_HOLD_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_HOLD_FLOAT, (*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_NO_REACTION) as i32);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, log_mask_flag as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn link_attack_lw_4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let attack_4_hold_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("attack_4_hold_frame"), 0);
    let attack_lw4_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_lw4_hold_frame"), 0);
    let attack_lw4_hold_keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_lw4_hold_keep_frame"), 0);
    let ratio = attack_lw4_hold_frame as f32/attack_4_hold_frame;
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    WorkModule::set_int(fighter.module_accessor, attack_lw4_hold_keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    if stick_x >= 0.5 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_f"), 0.0, attack_lw4_hold_keep_frame as f32/ratio, false, 0.0, false, false);
    }
    else if stick_x <= -0.5 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_b"), 0.0, attack_lw4_hold_keep_frame as f32/ratio, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold"), 0.0, attack_lw4_hold_keep_frame as f32/ratio, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(link_attack_lw_4_hold_main_loop as *const () as _))
}

unsafe extern "C" fn link_attack_lw_4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mut ret: i32 = 0;
    if situation_kind != *SITUATION_KIND_AIR {
        if 0 < mini_jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor) {
                if fighter.sub_check_button_jump().get_bool() {
                    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(0x10f40d7b92), -1);
                    WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                    fighter.change_status_jump_mini_attack(true.into());
                    ret = 1;
                }
            }
        }
        if mini_jump_attack_frame == 1 {
            let is_stop = fighter.global_table[IS_STOP].get_bool();
            if !is_stop {
                if 0 < attack_kind {
                    FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
                    WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                }
            }
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            if fighter.global_table[CURRENT_FRAME].get_f32() >= 59.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            }
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK)
            && motion_kind == hash40("attack_lw4_hold") {
                if stick_x >= 0.5 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_f"), -1.0, 1.0, 0.0);
                }
                if stick_x <= -0.5 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_b"), -1.0, 1.0, 0.0);
                }
            }
            if [hash40("attack_lw4_hold_walk_f"), hash40("attack_lw4_hold_walk_b")].contains(&motion_kind) {
                if motion_kind == hash40("attack_lw4_hold_walk_f") {
                    if stick_x <= -0.5 {
                        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_b"), -1.0, 1.0, 0.0);
                    }
                }
                if motion_kind == hash40("attack_lw4_hold_walk_b") {
                    if stick_x >= 0.5 {
                        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_f"), -1.0, 1.0, 0.0);
                    }
                }
                if (-0.5..0.5).contains(&stick_x) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold"), -1.0, 1.0, 0.0);
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold"), -1.0, 1.0, 0.0);
                }
            }
            if !fighter.is_smash_hold(FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME.into()).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4.into(), true.into());
                ret = 1;
            }
        }
        ret = 0;
    }
    else {
        if [hash40("attack_lw4_hold_walk_f"), hash40("attack_lw4_hold_walk_b")].contains(&motion_kind) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    ret.into()
}

unsafe extern "C" fn link_attack_lw_4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    }
    0.into()
}

unsafe extern "C" fn link_attack_lw_4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw4_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(link_attack_lw_4_main_loop as *const () as _))
}

unsafe extern "C" fn link_attack_lw_4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let lw4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("lw4_combo_max"), 0);
        if combo < lw4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_lw4_mtrans_common(hash40("attack_lw4").into());
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_attack_lw_4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    0.into()
}

//Wall Cling/Wall Climb

unsafe extern "C" fn link_attachwall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut start_stamina = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attach_wall_frame"));
    let cliff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    start_stamina -= cliff_count*20;
    WorkModule::set_int(fighter.module_accessor, start_stamina as i32, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attach_wall"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        link_attachwall_substatus(fighter);
    }
    GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(link_attachwall_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(link_attachwall_main_loop as *const () as _))
}

unsafe extern "C" fn link_attachwall_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
    if stick_x.abs() >= wall_jump_stick_x && stick_x.signum() == lr {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into()
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        WorkModule::sub_int(fighter.module_accessor, 30, *FIGHTER_STATUS_ATTACH_WALL_WORK_INT_FRAME);
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
    }
    if stick_y.abs() <= 0.25 {
        if motion_kind != hash40("attach_wall") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attach_wall"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        WorkModule::sub_int(fighter.module_accessor, 2, *FIGHTER_STATUS_ATTACH_WALL_WORK_INT_FRAME);
        let dir = stick_y.signum();
        if motion_kind != hash40("attach_wall_climb") {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attach_wall_climb"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if dir < 0.0 && MotionModule::frame(fighter.module_accessor) <= 0.0 {
            MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), false);
        }
        MotionModule::set_rate(fighter.module_accessor, dir*0.2);
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.25*dir, z: 0.0});
    }
    0.into()
}

unsafe extern "C" fn link_attachwall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let attach_side = if 0.0 <= lr {*GROUND_TOUCH_FLAG_LEFT} else {*GROUND_TOUCH_FLAG_RIGHT};
    let remove_attach = !GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_side));
    let stamina = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME)-1;
    let max_cliff = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_max_count"));
    let sweat_rate = 10.0;
    let modulo = stamina as f32%sweat_rate;
    WorkModule::sub_int(fighter.module_accessor, 1, *FIGHTER_STATUS_ATTACH_WALL_WORK_INT_FRAME);
    if GroundModule::can_entry_cliff(fighter.module_accessor) != 0  || fighter.sub_transition_group_check_air_cliff().get_bool() || remove_attach {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < max_cliff {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
            fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(), true.into());
            return 1.into();
        }
        else{
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME, 0);
        }
    }
    if stamina <= 0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(), false.into());
        return 1.into();
    }
    else if stamina < 90 {
        if stamina == 45 {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 7.0, 3.0, 0, 0, 0, 1.0, true);
        }
		if modulo < 1.0 {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sweat"), Hash40::new("top"), 0, 14.5, 3.0, 0, 0, 0, 0.35, true);
		}
    }
    0.into()
}

//Special N

unsafe extern "C" fn link_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor, bow_id, *WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
            }
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(0));
    0.into()
}

unsafe extern "C" fn link_bowarrow_haved_main_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if ItemModule::is_have_item(owner_boma, 0) {
        set_arrow_fuse_params(weapon.module_accessor, ItemModule::get_have_item_kind(owner_boma, 0), FuseKind::FUSE, ItemModule::get_have_item_trait(owner_boma, 0) as i32);
    }
    else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
        let kind = WorkModule::get_int(owner_boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        if kind != *ITEM_KIND_NONE {
            set_arrow_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
        }
    }
    else {
        if owner_kind == *FIGHTER_KIND_LINK {
            WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
        else if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
    }
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(item_boma, *ITEM_LINK_NO_HAVE);
        }
        if !LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            VisibilityModule::set_whole(item_boma, true);
            LinkModule::link(item_boma, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
        }
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_haved_main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_haved_main_loop(_weapon: &mut L2CFighterBase) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_bowarrow_fly_init_status(weapon: &mut L2CFighterBase) -> L2CValue {
    original_status(Init, weapon, *WN_LINK_BOWARROW_STATUS_KIND_FLY)(weapon);
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        if WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::POWER {
            let lr = PostureModule::lr(weapon.module_accessor);
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 12.0*lr, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            AttackModule::set_power_mul(weapon.module_accessor, 2.5);
        }
    }
    0.into()
}

unsafe extern "C" fn link_bowarrow_fly_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let status_kind_next = StatusModule::status_kind_next(weapon.module_accessor);
    let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    if [*WN_LINK_BOWARROW_STATUS_KIND_STICK, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK].contains(&status_kind_next) {
        if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma, status, false);
            }
        }
    }
    else {
        if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
        && sv_battle_object::is_active(item_id) {
            /*
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
            */
            LinkModule::remove_model_constraint(item_boma, true);
            StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
        }
    }
    EffectModule::detach_all(weapon.module_accessor, 5);
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
    }
    0.into()
}

//Special S

unsafe extern "C" fn link_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
        }
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(0));
    }
    0.into()
}

unsafe extern "C" fn link_boomerang_haved_main_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
    if StatusModule::status_kind(owner_boma) == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if ItemModule::is_have_item(owner_boma, 0) {
            set_boomerang_fuse_params(weapon.module_accessor, ItemModule::get_have_item_kind(owner_boma, 0), FuseKind::FUSE, ItemModule::get_have_item_trait(owner_boma, 0) as i32);
        }
        else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
            let kind = WorkModule::get_int(owner_boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            if kind != *ITEM_KIND_NONE {
                set_boomerang_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
            }
        }
        else {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
        }
        if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink(item_boma, *ITEM_LINK_NO_HAVE);
            }
            if !LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                VisibilityModule::set_whole(item_boma, true);
                LinkModule::link(item_boma, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
                LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("have"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                let offset_pos = Vector3f{x: 0.0, y: 9.5, z: 0.0};
                LinkModule::set_constraint_translate_offset(item_boma, &offset_pos);
            }
        }
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
        weapon.fastshift(L2CValue::Ptr(link_boomerang_haved_main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
        weapon.fastshift(L2CValue::Ptr(link_boomerang_haved_main_loop as *const () as _))
    }
}

unsafe extern "C" fn link_boomerang_haved_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REMOVE_SELF) {
        notify_event_msc_cmd!(weapon,Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn link_boomerang_haved_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if StatusModule::status_kind(owner_boma) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2 && WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
        if !ItemModule::is_have_item(owner_boma, 0) {
            WorkModule::on_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
        }
    }
    0.into()
}

unsafe extern "C" fn link_boomerang_fly_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let status_kind_next = StatusModule::status_kind_next(weapon.module_accessor);
    let item_id = WorkModule::get_int64(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
    && !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT)
    && (AttackModule::is_infliction_status(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_SHIELD)) {
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_boma, status, false);
        }
    }
    else if ![*WN_LINK_BOOMERANG_STATUS_KIND_TURN, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED].contains(&status_kind_next)
    && WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        LinkModule::remove_model_constraint(item_boma, true);
        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
    }
    0.into()
}

unsafe extern "C" fn link_boomerang_swallowed_pre_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let item_id = WorkModule::get_int64(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
    && sv_battle_object::is_active(item_id) {
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
    }
    original_status(Pre, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)(weapon)
}

unsafe extern "C" fn link_boomerang_swallowed_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        if smash::app::sv_battle_object::is_active(item_id) {
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma, status, false);
            }
        }
    }
    original_status(End, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)(weapon)
}

//Special Hi
unsafe extern "C" fn link_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        0.into()
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_hi").into(), Hash40::new("special_air_hi").into(), false.into());
        fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_main_loop as *const () as _))
    }
}

unsafe extern "C" fn link_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND) {
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);     
        let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);  
        let mut min_pos_y = pos_y;
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if GroundModule::ray_check_hit_pos(fighter.module_accessor, &smash::phx::Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: 100.0}, ground_hit_pos, true) {
            min_pos_y = ground_hit_pos.y;
        }
        let ground = find_ascendable_ground(fighter.module_accessor, pos_x, min_pos_y+height, pos_y+100.0, height);
        if pos_y < ground && ground < pos_y+100.0 {
            WorkModule::set_float(fighter.module_accessor, pos_y, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
            WorkModule::set_float(fighter.module_accessor, ground+5.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_START.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    0.into()
}

//Special Hi End

unsafe extern "C" fn link_special_hi_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_end_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_spin_attack"), 0.0, 1.0, false, 0.0, false, false);
    let rslash_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, rslash_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
    0.into()
}

//Special Hi Ascend Start

unsafe extern "C" fn link_special_hi_ascend_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_CLIFF, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, false);
    GroundModule::set_collidable(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, false);
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_ascend_start"), 0.0, 1.0, false, 0.0, false, false);
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.25, /* Green */ y: 1.0, /* Blue */ z: 0.75, /* Alpha */ w: 0.5};
    ColorBlendModule::set_main_color(fighter.module_accessor, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_ascend_start_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_ascend_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {    
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND.into(), false.into());
    }
    0.into()
}

//Special Hi Ascend

unsafe extern "C" fn link_special_hi_ascend_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_CLIFF, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, false);
    GroundModule::set_collidable(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_ascend"), 0.0, 1.0, false, 0.0, false, false);
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.25, /* Green */ y: 1.0, /* Blue */ z: 0.75, /* Alpha */ w: 0.5};
    ColorBlendModule::set_main_color(fighter.module_accessor, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_ascend_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_ascend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    let target_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let mut max_y = target_y+height+20.0;
    let modulo = current_frame % 10.0;
    WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    macros::SET_SPEED_EX(fighter, 0.0, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y+4.0}, &Vector2f{x: 0.0, y: -height/1.5}, ground_hit_pos, true) && pos_y >= max_y-(height*2.0) {
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ground_hit_pos.y, z: 0.0});
        GroundModule::set_attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_END.into(), false.into());
        return 0.into();
    }
    if modulo < 1.0 {
        if GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x, y: target_y+5.0}, &Vector2f{x: 0.0, y: -10.0}, true) != 1 {
            if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: target_y+20.0}, &Vector2f{x: 0.0, y: -40.0}, ground_hit_pos, true) {
                WorkModule::set_float(fighter.module_accessor, ground_hit_pos.y, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
                max_y = ground_hit_pos.y;
            }
            else {
                max_y = -999.0;
            }
        }
    }
    if pos_y > max_y {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_damage_paralyze"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::PLAY_SE(fighter, Hash40::new("vc_link_damage01"));
        macros::SET_SPEED_EX(fighter, 0.0, 2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 11.6, z: 0.0});
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_ascend"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
    GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{ x: 0.0, y: 0.0});
    macros::COL_NORMAL(fighter);
    macros::BURN_COLOR_NORMAL(fighter);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"),false,false);
    0.into()
}

//Special Hi Ascend End

unsafe extern "C" fn link_special_hi_ascend_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    GroundModule::set_collidable(fighter.module_accessor, true);
    GroundModule::set_gr_collision_mode(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, true);
    KineticModule::clear_speed_all(fighter.module_accessor);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        PostureModule::add_pos(fighter.module_accessor, &Vector3f::zero());
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_ascend_end_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_ascend_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::update_trans_move_speed(fighter.module_accessor);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    0.into()
}

//Special Hi Glide

unsafe extern "C" fn link_special_hi_glide_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    let limit_y_mul = 0.15;
    let combined_y_speed = air_speed_y_stable*limit_y_mul;
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*lr, 0.0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let get_stick_prev_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if get_stick_prev_y < squat_stick_y {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP.into(), false.into());
    }
    if stick_x < turn_run_stick_x {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide"), false, -1.0);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special Hi Glide Turn

unsafe extern "C" fn link_special_hi_glide_turn_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_turn_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    let limit_y_mul = 0.15;
    let combined_y_speed = air_speed_y_stable*limit_y_mul;
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    PostureModule::reverse_lr(fighter.module_accessor);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*1.5, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, (air_speed_x_stable*lr)*1.5, 0.0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_turn"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide_turn"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_turn_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_stick_prev_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if get_stick_prev_y < squat_stick_y {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_turn_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special Hi Glide Drop

unsafe extern "C" fn link_special_hi_glide_drop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_drop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_drop"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide_drop"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_drop_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_drop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_drop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special Hi Glide Land

unsafe extern "C" fn link_special_hi_glide_land_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_land_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_land"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide_land"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_land_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_land_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special Lw

unsafe extern "C" fn link_special_lw_blast_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, link_attack_s_4_hold_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, link_attack_s_4_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, link_attack_hi_4_hold_end_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4, link_attack_hi_4_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, link_attack_lw_4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4, link_attack_lw_4_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACH_WALL, link_attachwall_main_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, link_special_n_exit_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, link_special_s_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_START, link_special_hi_ascend_start_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_START, link_special_hi_ascend_start_main_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_exec_status)
    .status(Exit, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_exit_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_END, link_special_hi_ascend_end_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_END, link_special_hi_ascend_end_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_END, link_special_hi_ascend_end_exec_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_END, link_special_hi_ascend_end_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP, link_special_hi_glide_drop_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP, link_special_hi_glide_drop_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP, link_special_hi_glide_drop_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND, link_special_hi_glide_land_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND, link_special_hi_glide_land_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND, link_special_hi_glide_land_end_status)
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST, link_special_lw_blast_pre_status)
    .install()
    ;
    Agent::new("link_bowarrow")
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, link_bowarrow_haved_main_status)
    .status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, link_bowarrow_fly_init_status)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_FLY, link_bowarrow_fly_end_status)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_STICK, link_bowarrow_stick_end_status)
    .status(End, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK, link_bowarrow_stick_end_status)
    .install()
    ;
    Agent::new("link_boomerang")
    .status(Main, *WN_LINK_BOOMERANG_STATUS_KIND_HAVED, link_boomerang_haved_main_status)
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_HAVED, link_boomerang_haved_end_status)
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_FLY, link_boomerang_fly_end_status)
    .status(Pre, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED, link_boomerang_swallowed_pre_status)
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED, link_boomerang_swallowed_end_status)
    .install()
    ;
}