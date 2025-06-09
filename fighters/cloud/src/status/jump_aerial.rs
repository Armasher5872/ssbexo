use super::*;

unsafe extern "C" fn cloud_jump_aerial_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_jump_aerial_item_rocketbelt();
    cloud_status_jump_aerial_sub(fighter, false.into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_JumpAerial_Main as *const () as _))
}

unsafe extern "C" fn cloud_status_jump_aerial_sub(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let global_stick_x = fighter.global_table[STICK_X].get_f32();
    let jump_neutral_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
    let wall_jump_disable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("wall_jump_disable_frame"));
    let is_punisher = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = global_stick_x*lr*-1.0;
    let motion_kind;
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        WorkModule::set_int(fighter.module_accessor, wall_jump_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);
    }
    fighter.sub_air_check_fall_common_pre();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    if !param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    if param_1.get_bool() {
        motion_kind = param_1.get_u64();
    }
    else {
        if jump_neutral_x > stick_x {
            if is_punisher {
                motion_kind = hash40("punish_jump_aerial_f");
            }
            else {
                motion_kind = hash40("jump_aerial_f");
            }
        }
        else {
            if is_punisher {
                motion_kind = hash40("punish_jump_aerial_b");
            }
            else {
                motion_kind = hash40("jump_aerial_b");
            }
        }
    }
    if motion_kind != hash40("invalid") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_item_usagihat_jump_02"), true, false, false, false, enSEType(0));
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
    if fighter.global_table[FALL_BRAKE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[FALL_BRAKE_UNIQ].get_ptr());
        callable(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, cloud_jump_aerial_main_status)
    .install()
    ;
}