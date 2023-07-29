use super::*;

//Status AttackLw3 Main, adds Kirby's Down Tilt transition
#[skyline::hook(replace = L2CFighterCommon_status_AttackLw3_Main)]
unsafe fn status_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            let combo = ComboModule::count(fighter.module_accessor) as i32;
            let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
            if combo < s3_combo_max
            || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
                fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
            }
        }
        else {
            fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(mot), -1);
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if !fighter.global_table[IS_STOP].get_bool()
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    /* START OF NEW ADDITIONS */
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_KIRBY {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw32"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    /* END OF NEW ADDITIONS */
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_attacklw3_main);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}