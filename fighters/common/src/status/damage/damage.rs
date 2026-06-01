use super::*;

static mut IS_CALCULATING: Option<(u32, u32)> = None;

//Runs as you leave hitstop, used for ASDI
#[skyline::hook(replace = L2CFighterCommon_FighterStatusUniqProcessDamage_leave_stop)]
unsafe extern "C" fn fighter_status_uniq_process_damage_leave_stop(fighter: &mut L2CFighterCommon, _param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let damage_2 = fighter.local_func__fighter_status_damage_2();
    let get_damage_fly_angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose();
    let reaction_frame_mul_speed_up = fighter.reaction_frame_mul_speed_up();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let status_kind = StatusModule::status_kind(boma);
    let damage_lr = WorkModule::get_float(boma, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
    let release_action = WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    let mut damage_motion_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    let back_damage_effective_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("back_damage_effective_frame"));
    let is_absolute = damage_2["absolute_"].get_bool();
    //let attr = {fighter.clear_lua_stack(); lua_args!(fighter, hash40("attr")); sv_information::damage_log_value(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_u64()};
    let mut start_frame = 0.0;
    if !param_3.get_bool() {
        return 0.into();
    }
    if !is_absolute /*|| (is_absolute && attr == hash40("collision_attr_auto_shift"))*/ {
        fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay(damage_2);
    }
    FighterUtil::cheer_damage(module_accessor);
    fighter.check_ryu_final_damage_03(true.into());
    if release_action != *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_AIR {
        /*
        The original code here called to WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION), assigned it to LStack_a0, 
        assigned *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_GROUND to LStack_80, then assigned LStack_80 to LStack_a0 and clears LStack_a0. I think what it was intending on doing was assigning
        the ground_to_ground const to the release action, but got fucked up in translation to Ghidra
        */
        WorkModule::set_int(boma, *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_GROUND, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    }
    else {
        StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].clone();
        fighter.global_table[PREV_SITUATION_KIND].assign(&situation_kind);
        fighter.global_table[SITUATION_KIND].assign(&SITUATION_KIND_AIR.into());
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    }
    WorkModule::set_int(boma, *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_NONE, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    if damage_motion_kind == hash40("damage_fly_roll") {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
            damage_motion_kind = hash40("damage_fly_n");
        }
    }
    if damage_lr != 0.0 {
        if damage_lr*lr >= 0.0 || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR {
            PostureModule::set_lr(boma, damage_lr);
            PostureModule::update_rot_y_lr(boma);
            WorkModule::set_float(boma, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
        }
        else {
            let cont = if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
                if damage_motion_kind != hash40("wall_damage")
                && MotionModule::motion_kind(boma) != hash40("wall_damage") {
                    false
                }
                else {
                    true
                }
            }
            else {
                false
            };
            if cont || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                WorkModule::set_float(boma, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
            }
            else {
                TurnModule::set_turn(boma, Hash40::new("back_damage"), lr, false, false, true);
                PostureModule::reverse_lr(boma);
                WorkModule::set_int(boma, back_damage_effective_frame, *FIGHTER_INSTANCE_WORK_ID_INT_BACK_DAMAGE_EFFECTIVE_FRAME);
            }
        }
    }
    if damage_motion_kind != hash40("invalid") {
        if damage_motion_kind == hash40("wall_damage") {
            start_frame = WorkModule::get_param_float(boma, hash40("common"), hash40("wall_damage_start_frame"));
            if MotionModule::is_flag_start_1_frame_from_motion_kind(boma, Hash40::new("wall_damage")) {
                start_frame -= 1.0;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if fighter.global_table[DAMAGE_MOTION_KIND_CALLBACK].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(fighter.global_table[DAMAGE_MOTION_KIND_CALLBACK].get_ptr());
                damage_motion_kind = callable(fighter, damage_motion_kind.into()).get_u64();
            }
        }
        MotionModule::change_motion(boma, Hash40::new_raw(damage_motion_kind), start_frame, 1.0, false, 0.0, false, false);
        if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
            if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
                fighter.set_damage_motion_rate(damage_motion_kind.into(), start_frame.into(), WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_TO_PIERCE).into());
                let damage_fly_angle = FighterUtil::set_damage_fly_angle(boma, 0.0, 1.0, 360.0, MotionNodeRotateCompose { _address: get_damage_fly_angle_compose.get_i32() as u8 });
                WorkModule::set_float(boma, damage_fly_angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                WorkModule::on_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
            }
        }
        else {
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                let damage_fly_angle = FighterUtil::set_damage_fly_angle(boma, 0.0, 1.0, 180.0, MotionNodeRotateCompose { _address: get_damage_fly_angle_compose.get_i32() as u8 });
                WorkModule::set_float(boma, damage_fly_angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                WorkModule::on_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
            }
            let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(damage_motion_kind), true);
            if cancel_frame <= 0.0 {
                cancel_frame = MotionModule::end_frame(boma);
            }
            if 0.0 < reaction_frame_mul_speed_up.get_f32() {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                    let frame_sub = WorkModule::get_param_float(boma, hash40("common"), 0x255c556cd3);
                    let diff = reaction_frame_mul_speed_up.get_f32()-frame_sub;
                    let modulo = diff % cancel_frame;
                    if 0.0 < modulo {
                        MotionModule::set_frame(boma, cancel_frame-modulo, true);
                    }
                }
                else {
                    MotionModule::set_rate(boma, cancel_frame/reaction_frame_mul_speed_up.get_f32());
                }
            }
        }
        WorkModule::set_int64(boma, hash40("invalid") as i64, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_damage_uniq_process_mainStop)]
unsafe extern "C" fn sub_damage_uniq_process_main_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_ELEC) {
        let get_damage_stop_frame = FighterStopModuleImpl::get_damage_stop_frame(boma);
        if get_damage_stop_frame == 1 {
            fighter.FighterStatusDamage__req_fly_roll_smoke_first();
        }
        fighter.sub_FighterStatusDamage_correctDamageVectorExecStop();
    }
    else {
        fighter.exec_damage_elec_hit_stop();
    }
    /*
    //Removes SDI functionality
    if StopModule::is_damage(boma) {
        let is_absolute = {fighter.clear_lua_stack(); lua_args!(fighter, hash40("absolute")); sv_information::damage_log_value(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
        if !is_absolute {
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PARALYZE_STOP) {
                let damage_2_func = fighter.local_func__fighter_status_damage_2();
                fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay_flick(damage_2_func);
            }
        }
    }
    */
    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind_interrupt) {
        fighter.sub_ftStatusUniqProcessDamageFlyRoll_execStop();
    }
    0.into()
}

//Credited to HDR, used for calculating Finishing Zoom

//Related to the updated finishing zoom function
#[skyline::hook(offset = 0x402f00, inline)]
unsafe extern "C" fn calculate_knockback(ctx: &InlineCtx) {
    let damage_module = ctx.registers[19].x();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = ctx.registers[20].x() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe extern "C" fn process_knockback(ctx: &InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = ctx.registers[20].x() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            calculate_finishing_hit(defender, attacker, ctx.registers[19].x() as *const f32);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            fighter_status_uniq_process_damage_leave_stop,
            sub_damage_uniq_process_main_stop
        );
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x633de0).nop(); //Removes the vanilla kill zoom in favor of the updated function. This one handles normal hits
    let _ = skyline::patching::Patch::in_text(0x6373a4).data(0xD503201Fu32); //Removes the vanilla kill zoom in favor of the updated function. This one handles throws
    let _ = skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        process_knockback,
        calculate_knockback
    );
}