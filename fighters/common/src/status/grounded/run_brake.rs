/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Run Brake
#[skyline::hook(replace = L2CFighterCommon_status_RunBrake_Main)]
unsafe extern "C" fn status_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32(); //New
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let is_have_item = ItemModule::is_have_item(boma, 0);
    let lr = PostureModule::lr(boma);
    let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y")); //New
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y")); //New
    let notify_taunt_hash = {fighter.clear_lua_stack(); fighter.push_lua_stack(&mut L2CValue::new_int(0x1daca540be)); sv_battle_object::notify_event_msc_cmd(lua_state); fighter.pop_lua_stack(1).get_bool()};
    let pad_attack = pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    let is_catch = cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_RUN_BRAKE_HI4) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
            fighter.clear_lua_stack(); 
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW); 
            sv_module_access::item(lua_state); 
            let mut item_throw = fighter.pop_lua_stack(1).get_bool();
            if !item_throw {
                fighter.clear_lua_stack(); 
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT); 
                sv_module_access::item(lua_state); 
                if fighter.pop_lua_stack(1).get_bool() {
                    item_throw = ItemModule::get_shoot_item_bullet(boma, 0) <= 0
                }
            }
            if item_throw {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_ground_item().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) {
        fighter.clear_lua_stack(); 
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW); 
        sv_module_access::item(lua_state); 
        if fighter.pop_lua_stack(1).get_bool() {
            if pad_attack {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), true.into());
                return 0.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE) {
        if is_have_item {
            if is_catch {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
                sv_module_access::item(lua_state);
                if !fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                    return 1.into();
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH) {
        if is_have_item {
            if is_catch {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
                sv_module_access::item(lua_state); 
                if !fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), true.into());
                    return 0.into();
                }
            }
        }
    }
    if fighter.sub_transition_group_check_ground_catch().get_bool() {
        return 0.into();
    }
    if fighter.sub_transition_group_check_ground_escape().get_bool() {
        return 0.into();
    }
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) {
        fighter.clear_lua_stack(); 
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING); 
        sv_module_access::item(lua_state); 
        if fighter.pop_lua_stack(1).get_bool() {
            if pad_attack {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) {
        if pad_attack {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
            return 0.into();
        }
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 || FighterUtil::is_valid_auto_catch_item(boma, false) {
        if cmd_cat1 & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(lua_state);
            if fighter.pop_lua_stack(1).get_bool() {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH) {
                    if !is_have_item {
                        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                        return 1.into();
                    }
                }
            }
            if ItemModule::get_pickable_item_size(boma) == *ITEM_SIZE_LIGHT as u64 {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
                    sv_module_access::item(lua_state);
                    if fighter.pop_lua_stack(1).get_bool() {
                        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
                        return 1.into();
                    }
                }
            }
        }
    }
    if fighter.sub_transition_group_check_ground_jump().get_bool() {
        return 0.into();
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_RUN_BRAKE_FLAG_TURN_RUN) {
        if stick_x*lr <= turn_run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_RUN.into(), false.into());
            return 0.into();
        }
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
            return 1.into();
        }
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
            return 1.into();
        }
    }
    if fighter.sub_ground_check_ottotto_motion_end().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP_SHAKE) {
        let speed_length = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP); sv_kinetic_energy::get_speed_length(lua_state)};
        let run_brake_stop_shake_speed = WorkModule::get_param_float(boma, hash40("common"), hash40("run_brake_stop_shake_speed"));
        if speed_length >= run_brake_stop_shake_speed {
            let start_speed = WorkModule::get_float(boma, *FIGHTER_STATUS_RUN_BRAKE_WORK_FLOAT_START_SPEED);
            let shake_data_brake_scale = WorkModule::get_param_float(boma, hash40("common"), hash40("shake_data_brake_scale"));
            let ratio = (speed_length-run_brake_stop_shake_speed)/(start_speed-run_brake_stop_shake_speed);
            let lerp = fighter.lerp(run_brake_stop_shake_speed.into(), 1.0_f32.into(), ratio.into()).get_f32();
            let scale = shake_data_brake_scale*lerp;
            ShakeModule::set_scale_kind(boma, Hash40::new("brake"), scale);
        }
        else {
            ShakeModule::stop_kind(boma, Hash40::new("brake"));
            WorkModule::on_flag(boma, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP_SHAKE);
        }
    }
    /*Start of New*/
    if GroundModule::is_passable_ground(boma) && fighter.global_table[STICK_Y].get_f32() < pass_stick_y && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
        return 0.into();
    }
    if (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0) 
    || (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0)) 
    && notify_taunt_hash {
        fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        return 0.into();
    }
    /*End of New*/
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_runbrake_main);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}