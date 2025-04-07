use super::*;

unsafe extern "C" fn cloud_squat_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait_common(false.into());
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("punish_squat_wait"), 0.0, 1.0, 0.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_squat_wait_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let squat_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_SQUAT_WORK_INT_FRAME);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let mut can_loop = true;
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if fighter.sub_squat_common_Main().get_bool() {
        return 1.into();
    }
    if fighter.sub_squat_check_front().get_bool() {
        return 1.into();
    }
    if fighter.sub_squat_check_back().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV) {
        if squat_stick_y < stick_y {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_RV.into(), true.into());
                return 1.into();
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SQUAT_FLAG_ENABLE_PASS) {
        if squat_frame == 0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), false.into());
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        can_loop = fighter.sub_squat_wait_mtrans_conditions().get_bool();
    }
    if can_loop {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_STATUS_SQUAT_FLAG_ENABLE_PASS);
        sv_module_access::item(fighter.lua_state_agent);
        let item_function = fighter.pop_lua_stack(1.into());
        if item_function != hash40("squat_wait_item") {
            let switch_squat_motion = {fighter.clear_lua_stack(); lua_args!(fighter, 0); sv_fighter_util::switch_squat_motion(fighter.lua_state_agent, 0)};
            if switch_squat_motion == 1 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_wait2"), 0.0, 1.0, false, 0.0, false, false);
            }
            if switch_squat_motion == -1 {
                MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 0.0, true, false, false);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_squat_wait"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_wait"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_wait_item"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, cloud_squat_wait_main_status)
    .install()
    ;
}