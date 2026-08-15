use super::*;

pub unsafe extern "C" fn edge_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ESCAPE_AIR_ENABLE_WING);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_CHARGED);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_BLADE_DASH_EARLY_CANCEL);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_CHANGE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ATTACK_COUNT);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE_TIMER);
    UiManager::set_edge_materia_info(entry_id, 0.0, 1.0, WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED));
}

pub unsafe extern "C" fn edge_taunt_hold(fighter: &mut L2CFighterCommon, motion: u64, restart_frame: f32) {
    let boma = fighter.module_accessor;
    let hi_check_on = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw_check_on = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let hi_check_off = ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw_check_off = ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let motion_kind = MotionModule::motion_kind(boma);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_ENABLE_LOOP) {
        if motion == hash40("appeal_hi_l") {
            if hi_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_hi_l_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_hi_r") {
            if hi_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_hi_r_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_s_l") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::change_motion(boma, Hash40::new("appeal_s_l_trans"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_s_r") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                MotionModule::change_motion(boma, Hash40::new("appeal_s_r_trans"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_lw_l") {
            if lw_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_lw_l_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("appeal_lw_r") {
            if lw_check_on {
                MotionModule::change_motion(boma, Hash40::new("appeal_lw_r_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_ENABLE_LOOP);
    }
    if motion_kind == hash40("appeal_hi_l_loop") {
        if hi_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_hi_r_loop") {
        if hi_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_s_l_loop") {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_s_r_loop") {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_lw_l_loop") {
        if lw_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
    if motion_kind == hash40("appeal_lw_r_loop") {
        if lw_check_off {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
        }
    }
}

pub unsafe extern "C" fn edge_check_valid_wing_enable(boma: *mut BattleObjectModuleAccessor) -> bool {
    let activate_point_upper_limit = WorkModule::get_param_float(boma, hash40("param_one_winged"), hash40("activate_point_upper_limit"));
    let threshold_activate_point = WorkModule::get_float(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_THRESHOLD_ACTIVATE_POINT);
    let activate_point = WorkModule::get_float(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
    let winged_state = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
    let should_instant_wing = if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_RULE_HP) {WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SUDDEN_DEATH)} else {false};
    let off_req = if !WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_WING_OFF_REQ) {winged_state != 2} else {false};
    let mut ret = false;
    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL))
    && (should_instant_wing || (off_req & (threshold_activate_point <= activate_point_upper_limit && threshold_activate_point <= activate_point))) {
        ret = true;
    }
    ret
}

pub unsafe extern "C" fn edge_special_kinetic_handler(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if !param_1 && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            return 0.into();
        }
        sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, false);
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(set_needs_set_param, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, false);
        }
    }
    0.into()
}

pub unsafe extern "C" fn edge_enable_cancel_terms(boma: *mut BattleObjectModuleAccessor) {
    WorkModule::set_int(boma, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
}

pub unsafe extern "C" fn edge_try_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let boma = fighter.module_accessor;
    let is_winged = WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
    if fighter.sub_check_command_guard().get_bool() {
        if situation_kind == *SITUATION_KIND_AIR {
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                return 1.into();
            }
        }
        else {
            WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    if fighter.sub_check_jump_in_charging().get_bool() {
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
            return 1.into();
        }
        else {
            WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
            return 1.into();
        }
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        if notify_taunt_hash {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_APPEAL.into(), true.into());
            return 1.into();
        }
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_PARRY.into(), true.into());
        return 1.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
        return 1.into();
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
        return 1.into();
    }
    if is_winged {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_ZANSHIN.into(), true.into());
            return 1.into();
        }
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_PARRY_FLASH.into(), true.into());
        return 1.into();
    }
    0.into()
}

//Credit to WuBoyTH
pub unsafe extern "C" fn edge_special_hi_param_float_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), param).into()
}

//Credit to WuBoyTH
pub unsafe extern "C" fn edge_special_hi_param_helper_inner(hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let hash = hash.get_u64();
    if !charged_rush.get_bool() {
        return hash.into();
    }
    let new_hash = if hash == hash40("rot_decide_frame") {
        hash40("charged_rot_decide_frame")
    }
    else if hash == hash40("rot_end_frame") {
        hash40("charged_rot_end_frame")
    }
    else if hash == hash40("rush_frame") {
        hash40("charged_rush_frame")
    }
    else if hash == hash40("rush_speed") {
        hash40("charged_rush_speed")
    }
    else if hash == hash40("rush_brake_frame") {
        hash40("charged_rush_brake_frame")
    }
    else if hash == hash40("rush_brake") {
        hash40("charged_rush_brake")
    }
    else if hash == hash40("ground_speed_x_mul") {
        hash40("charged_ground_speed_x_mul")
    }
    else if hash == hash40("landing_speed_x_mul") {
        hash40("charged_landing_speed_x_mul")
    }
    else if hash == hash40("landing_brake_x") {
        hash40("charged_landing_brake_x")
    }
    else if hash == hash40("landing_fix_frame") {
        hash40("charged_landing_fix_frame")
    }
    else if hash == hash40("rotate_back_begin_frame") {
        hash40("charged_rotate_back_begin_frame")
    }
    else if hash == hash40("rotate_back_end_frame") {
        hash40("charged_rotate_back_end_frame")
    }
    else if hash == hash40("rush_end_speed_mul") {
        hash40("charged_rush_end_speed_mul")
    }
    else if hash == hash40("rush_end_brake_x") {
        hash40("charged_rush_end_brake_x")
    }
    else if hash == hash40("rush_end_gravity_accel") {
        hash40("charged_rush_end_gravity_accel")
    }
    else if hash == hash40("control_accel_x_mul") {
        hash40("charged_control_accel_x_mul")
    }
    else if hash == hash40("control_speed_x_max_mul") {
        hash40("charged_control_speed_x_max_mul")
    }
    else{
        hash
    };
    new_hash.into()
}

pub unsafe extern "C" fn fun_71009de2b0(fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let vtable_slow = (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0) && !StopModule::is_stop(boma) && !SlowModule::is_skip(boma);
    let winged_state = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
    match winged_state {
        0 => {
            VisibilityModule::set_status_default(boma, Hash40::new("wing"), Hash40::new("wing_normal"));
            WorkModule::set_int(boma, 1, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
        },
        1 => {
            if vtable_slow {
                WorkModule::count_down_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_FLARE_EFFECT_FRAME, 0);
            }
            let effect_frame = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_FLARE_EFFECT_FRAME);
            if effect_frame < 1 {
                let flicker = WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_FLICKER_ON);
                let frame = if !flicker {hash40("flicker_off_frame")} else {hash40("flicker_on_frame")};
                let flare_effect_frame = WorkModule::get_param_int(boma, hash40("param_one_winged"), frame);
                WorkModule::set_int(boma, flare_effect_frame, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_FLARE_EFFECT_FRAME);
                if flicker {
                    MotionModule::remove_motion_partial(boma, *FIGHTER_EDGE_MOTION_PART_SET_KIND_WING_SHINE, false);
                    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_FLICKER_ON);
                }
                else {
                    let wing_motion_partial_kind = MotionModule::motion_kind_partial(boma, *FIGHTER_EDGE_MOTION_PART_SET_KIND_WING);
                    let bool_check = wing_motion_partial_kind != hash40("invalid") && (WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_MOTION_BLEND_STATE) == 0 || *(battle_object_slow as *const u32) == 2);
                    if !bool_check {
                        MotionModule::add_motion_partial(boma, *FIGHTER_EDGE_MOTION_PART_SET_KIND_WING_SHINE, Hash40::new("wing"), 0.0, 1.0, false, false, 0.0, true, true, false);
                    }
                    else {
                        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_WING_OFF_REQ) {
                            WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_WING_OFF_REQ);
                            VisibilityModule::set_status_default(boma, Hash40::new("wing"), Hash40::new("wing_hide"));
                            WorkModule::set_int(boma, 2, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
                            WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_FLARE_EFFECT_FRAME);
                            MotionModule::remove_motion_partial(boma, *FIGHTER_EDGE_MOTION_PART_SET_KIND_WING_SHINE, false);
                            WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_FLICKER_ON);
                        }
                    }
                    WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_FLICKER_ON);
                }
            }
            if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_WING_OFF_REQ) {
                WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_WING_OFF_REQ);
                VisibilityModule::set_status_default(boma, Hash40::new("wing"), Hash40::new("wing_hide"));
                WorkModule::set_int(boma, 2, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
                WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_FLARE_EFFECT_FRAME);
                MotionModule::remove_motion_partial(boma, *FIGHTER_EDGE_MOTION_PART_SET_KIND_WING_SHINE, false);
                WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_FLICKER_ON);
            }
        },
        3 => {
            let deactivate_wing_delete_frame = WorkModule::get_param_int(boma, hash40("param_one_winged"), hash40("deactivate_wing_delete_frame"));
            WorkModule::set_int(boma, deactivate_wing_delete_frame, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_DEACTIVATE_WING_FRAME);
            WorkModule::set_int(boma, 4, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
        },
        4 => {
            let deactivate_wing_frame = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_DEACTIVATE_WING_FRAME);
            if 0 < deactivate_wing_frame {
                if vtable_slow {
                    WorkModule::count_down_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_DEACTIVATE_WING_FRAME, 0);
                }
            }
            VisibilityModule::set_status_default(boma, Hash40::new("wing"), Hash40::new("wing_hide"));
            WorkModule::set_int(boma, -1, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
            WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_FLARE_EFFECT_FRAME);
            MotionModule::remove_motion_partial(boma, *FIGHTER_EDGE_MOTION_PART_SET_KIND_WING_SHINE, false);
            WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_FLICKER_ON);
        },
        _ => {}
    };
    if fighter.battle_object.kind as i32 == *FIGHTER_KIND_EDGE {
        let wing_state = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
        let manager_check = *(singletons::FighterManager() as *const u64).add(0xBE) != 0;
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_ON) {
            if 1 < wing_state || manager_check {
                MotionAnimcmdModule::call_script_single(boma, 3, Hash40::new_raw(0x1507232964), -1);
                WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_ON);
            }
        }
        else if (1 >= wing_state && !manager_check) && *(singleton_fighter_hook(fighter) as *const u64).add(0x56) == 0 {
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
                MotionAnimcmdModule::call_script_single(boma, 3, Hash40::new_raw(0x106ad49d7f), -1);
                WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_FLARE_EFFECT_ON);
            }
        }
    }
}

pub unsafe extern "C" fn fun_71009de890(fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let hair_state = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_HAIR_STATE);
    if !WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_HAIR_OFF_CHANGED) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_HAIR_OFF_REQ) || !WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            if hair_state != 0 {
                WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_HAIR_STATE);
                VisibilityModule::set_status_default(boma, Hash40::new("hair"), Hash40::new("hair_none"));
            }
            WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_HAIR_OFF_REQ);
            WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_HAIR_OFF_CHANGED);
        }
        if hair_state != 1 {
            WorkModule::set_int(boma, 1, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_HAIR_STATE);
            VisibilityModule::set_status_default(boma, Hash40::new("hair"), Hash40::new("hair_wing_on"));
        }
    }
}

const FULL_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0
];

#[derive(Default, Copy, Clone)]
pub struct EdgeMateria {
    pub base_pane: u64,
    pub sphere: u64,
    pub full_pane: u64,
    pub wing_pane: u64,
    pub sphere_xy: (f32, f32),
    pub sphere_width_height: (f32, f32),
    pub percent: f32,
    pub enabled: bool,
}

impl EdgeMateria {
    pub fn new(layout_data: u64) -> Self {
        let base_pane = get_pane_from_layout(layout_data, "edge_materia_base\0").expect("Could not find base sphere!");
        let sphere = get_pane_from_layout(layout_data, "edge_materia_sphere\0").expect("Could not find sphere!");
        let full_pane = get_pane_from_layout(layout_data, "edge_materia_full\0").expect("Could not find full pane!");
        let wing_pane = get_pane_from_layout(layout_data, "edge_materia_wing\0").expect("Could not find wing pane!");
        return Self {
            base_pane: base_pane,
            sphere: sphere,
            full_pane: full_pane,
            wing_pane: wing_pane,
            sphere_xy: (-1.0, -1.0),
            sphere_width_height: (-1.0, -1.0),
            percent: -1.0,
            enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.base_pane, true);
        set_pane_visible(self.sphere, true);
        set_pane_visible(self.full_pane, false);
        set_pane_visible(self.wing_pane, false);
        if self.sphere_xy == (-1.0, -1.0) {
            self.sphere_xy = get_pane_pos(self.sphere);
        }
        if self.sphere_width_height == (-1.0, -1.0) {
            self.sphere_width_height = get_width_height(self.sphere);
        }
        self.percent = 0.0;
    }
    pub fn set_percent(&mut self, nom: f32, den: f32, is_winged: bool) {
        let percent = if is_winged {
            2.0
        }
        else {
            if !den.is_nan() && den != 0.0 {
                (nom/den).clamp(0.0, 1.0)
            }
            else {
                0.0
            }
        };
        self.percent = percent;
    }
    pub fn update_percent(&mut self) {
        set_tex_coords(self.base_pane, FULL_TEXCOORDS);
        set_tex_coords(self.full_pane, FULL_TEXCOORDS);
        set_tex_coords(self.wing_pane, FULL_TEXCOORDS);
        if self.percent >= 2.0 {
            set_pane_visible(self.base_pane, false);
            set_pane_visible(self.sphere, false);
            set_pane_visible(self.full_pane, false);
            set_pane_visible(self.wing_pane, true);
        }
        else {
            if self.percent >= 1.0 {
                set_pane_visible(self.base_pane, false);
                set_pane_visible(self.sphere, false);
                set_pane_visible(self.full_pane, true);
                set_pane_visible(self.wing_pane, false);
            }
            else {
                set_pane_visible(self.base_pane, true);
                set_pane_visible(self.sphere, true);
                set_pane_visible(self.full_pane, false);
                set_pane_visible(self.wing_pane, false);
            }
        }
        set_pane_pos(self.sphere, self.sphere_xy.0, 44.0+(10.0*self.percent));
        set_tex_coords(self.sphere, [
            0.0, 1.0-self.percent,
            1.0, 1.0-self.percent,
            0.0, 1.0,
            1.0, 1.0
        ]);
        set_width_height(self.sphere, self.sphere_width_height.0, self.sphere_width_height.1*self.percent);
    }
}