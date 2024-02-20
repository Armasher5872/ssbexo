use super::*;

unsafe extern "C" fn link_attack_dash_bound_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_DASH | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32, 0);
    0.into()
}

unsafe extern "C" fn link_attack_dash_bound_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.2);
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

unsafe extern "C" fn link_attack_lw_4_hold_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut log_mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR);
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
    let ratio = (attack_lw4_hold_frame as f32/attack_4_hold_frame);
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
                    ret = 1.into();
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
                WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            }
            if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK)
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
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold"), -1.0, 1.0, 0.0);
                }
            }
            if !fighter.is_smash_hold(FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME.into()).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4.into(), true.into());
                ret = 1.into();
            }
        }
        ret = 0.into();
    }
    else {
        if [hash40("attack_lw4_hold_walk_f"), hash40("attack_lw4_hold_walk_b")].contains(&motion_kind) {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
        }
        else {
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
            WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    ret.into()
}

unsafe extern "C" fn link_attack_lw_4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
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
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    0.into()
}

//Special N

unsafe extern "C" fn link_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor, bow_id, *WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
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
        let kind = WorkModule::get_int(owner_boma, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        if kind != *ITEM_KIND_NONE {
            set_arrow_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
        }
    }
    else {
        if owner_kind == *FIGHTER_KIND_LINK {
            WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
        else if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
        }
    }
    if WorkModule::is_flag(weapon.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
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

unsafe extern "C" fn link_bowarrow_haved_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_bowarrow_fly_init_status(weapon: &mut L2CFighterBase) -> L2CValue {
    original_status(Init, weapon, *WN_LINK_BOWARROW_STATUS_KIND_FLY)(weapon);
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        if WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::POWER {
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
    if [*WN_LINK_BOWARROW_STATUS_KIND_STICK, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK].contains(&status_kind_next) {
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma, status, false);
            }
        }
    }
    else {
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
        }
    }
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

unsafe extern "C" fn link_bowarrow_stick_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        let team_no = if owner_kind == *FIGHTER_KIND_KIRBY {
            WorkModule::get_int(owner_boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
            WorkModule::get_int(owner_boma, FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO)
        }
        else {
            WorkModule::get_int(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO)
        };
        TeamModule::set_team(weapon.module_accessor, team_no, true);
        TeamModule::set_team_owner_id(weapon.module_accessor, (*(owner_boma)).battle_object_id);
        WorkModule::set_flag(weapon.module_accessor, false, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT);
    }
    0.into()
}

//Special S

unsafe extern "C" fn link_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
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
            let kind = WorkModule::get_int(owner_boma, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            if kind != *ITEM_KIND_NONE {
                set_boomerang_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
            }
        }
        else {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
        }
        if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
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
    if StatusModule::status_kind(owner_boma) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2 && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
        if !ItemModule::is_have_item(owner_boma, 0) {
            WorkModule::on_flag(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
        }
    }
    0.into()
}

unsafe extern "C" fn link_boomerang_fly_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let status_kind_next = StatusModule::status_kind_next(weapon.module_accessor);
    let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED)
    && !WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT)
    && (AttackModule::is_infliction_status(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_SHIELD)) {
        LinkModule::remove_model_constraint(item_boma, true);
        if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
            StatusModule::change_status_request(item_boma, status, false);
        }
    }
    else if ![*WN_LINK_BOOMERANG_STATUS_KIND_TURN, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED].contains(&status_kind_next)
    && WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        LinkModule::remove_model_constraint(item_boma, true);
        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
    }
    0.into()
}

unsafe extern "C" fn link_boomerang_swallowed_pre_status(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
    }
    original_status(Pre, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)(weapon)
}

unsafe extern "C" fn link_boomerang_swallowed_end_status(weapon: &mut L2CFighterBase) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let item_id = WorkModule::get_int64(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
        if smash::app::sv_battle_object::is_active(item_id) {
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = WorkModule::get_int(weapon.module_accessor, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
                StatusModule::change_status_request(item_boma, status, false);
            }
        }
    }
    original_status(End, weapon, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED)(weapon)
}

//Special Lw

unsafe extern "C" fn link_special_lw_blast_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .status(Pre, FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_pre_status)
    .status(Init, FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_init_status)
    .status(Main, FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_main_status)
    .status(End, FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND, link_attack_dash_bound_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, link_attack_lw_4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4, link_attack_lw_4_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, link_special_n_exit_status)
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