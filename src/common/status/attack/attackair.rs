/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   AIR ATTACK STATUSES   */
//Attack Air Main Status, used the animations movement by default, namely used for DJC
pub unsafe extern "C" fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(L2CValue::Bool(false));
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status_loop as *const () as _))
}

//Attack Air Main Status Loop, leniency window for DJC
pub unsafe extern "C" fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_AERIAL {
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_YOSHI {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 20.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_NESS {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 14.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_MEWTWO {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 8.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_TRAIL {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 16.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
    }
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    }
    else {
        1.into()
    }
}

//Status Attack Air Main Common, used for continual platform drops and ECB Shifts
#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.attack_air_common_strans().get_bool() {
        /* START OF NEW ADDITIONS */
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        let frame = fighter.global_table[CURRENT_FRAME].get_f32();
        let motion_kind = MotionModule::motion_kind(boma);
        let stick_x = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(boma);
        let mut pos = Vector3f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)}; // get current pos
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_PASS {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) != true {
                GroundModule::set_passable_check(boma, true);
            }
            if frame <= 1.0 {
                pos.y += 4.5;
                PostureModule::set_pos(boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            }
        }
        //Angleable Dair
        if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
            if motion_kind == hash40("attack_air_lw") {
                if stick_x > 0.5
                && frame < 7.0 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_lw_diagonal_r"), -1.0, 1.0, 0.0, false, false);
                }
                if stick_x < -0.5
                && frame < 7.0 {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_lw_diagonal_l"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
        /* END OF NEW ADDITIONS */
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return false.into();
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(fighter.module_accessor) {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Init, inherits the double jump animation movement when doing an aerial (init)
#[smashline::hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_initEv")]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.sub_attack_air_kind();
    if motion_kind == hash40("jump_aerial_f") || motion_kind == hash40("jump_aerial_b") {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        } 
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
    } 
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    let _ = fighter.sub_attack_air_uniq_process_init();
    0.into()
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Exec, inherits the double jump animation movement when doing an aerial (exec)
#[smashline::hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon59sub_attack_air_inherit_jump_aerial_motion_uniq_process_execEv")]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_AERIAL {
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_YOSHI {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 20.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_NESS {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 14.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_MEWTWO {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 8.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_TRAIL {
            if fighter.global_table[CURRENT_FRAME].get_f32() <= 16.0
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        }
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_attackair_main_common);
    }
}

pub fn install() {
    install_hooks!(
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl,
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl
    );
    skyline::nro::add_hook(nro_hook);
}