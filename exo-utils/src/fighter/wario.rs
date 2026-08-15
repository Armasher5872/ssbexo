use super::*;

pub unsafe extern "C" fn wario_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 1.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_PILEDRIVER_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
}

pub unsafe extern "C" fn wario_try_charge(fighter: &mut L2CFighterCommon, start_frame: f32, rate: f32, default_rate: f32) {
    let boma = fighter.module_accessor;
    let charge = WorkModule::get_int(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
    let frame = MotionModule::frame(boma);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && frame >= start_frame {
        if charge < 45 {
            MotionModule::set_rate(boma, rate*(charge as f32));
            wario_charge_effect(fighter, charge);
            WorkModule::inc_int(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
        }
    }
    else {
        MotionModule::set_rate(boma, default_rate);
    }
    if charge >= 45 && !WorkModule::is_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED) {
        STOP_SE(fighter, Hash40::new("se_wario_special_l02"));
        WorkModule::on_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
        MotionModule::set_rate(boma, default_rate);
    }
}

unsafe extern "C" fn wario_charge_effect(fighter: &mut L2CFighterCommon, charge_frames: i32) {
    let lua_state = fighter.lua_state_agent;
    if charge_frames >= 10 {
        if charge_frames == 17 {
            PLAY_SE(fighter, Hash40::new("se_wario_special_l02"));
        }
        if [12, 18, 24, 30, 36].contains(&charge_frames) {
            fighter.clear_lua_stack();
            lua_args!(fighter, 0.15, 0.08, 1.0, 0.5);
            sv_animcmd::FLASH_NO_STOP(lua_state);
        }
        if [15, 21, 27, 33, 39].contains(&charge_frames) {
            fighter.clear_lua_stack();
            lua_args!(fighter, 1.0, 1.0, 1.0, 0.5);
            sv_animcmd::FLASH_NO_STOP(lua_state);
        }
        if charge_frames % 3 == 2 {
            COL_NORMAL(fighter);
        }
    }
}

pub unsafe extern "C" fn wario_special_n_end(fighter: &mut L2CFighterCommon, is_throw: bool) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
    ].contains(&status_kind) {
        if CatchModule::is_catch(boma) {
            let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let capture_status = StatusModule::status_kind(capture_boma);
                let pos = *PostureModule::pos(boma);
                if capture_status != *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN {
                    StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, false);
                }
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            CatchModule::set_send_cut_event(boma, true);
            CatchModule::catch_cut(boma, false, false);
            if is_throw {
                damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            }
        }
    }
}

pub unsafe extern "C" fn wario_special_n_exit(fighter: &mut L2CFighterCommon, is_throw: bool) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
    ].contains(&status_kind) {
        if LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
            if capture_id != 0x50000000 {
                let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                let capture_status = StatusModule::status_kind(capture_boma);
                let pos = *PostureModule::pos(boma);
                if capture_status != *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN {
                    StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, false);
                }
                PostureModule::set_pos(capture_boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
            sv_module_access::_catch(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
        if is_throw {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
    }
}