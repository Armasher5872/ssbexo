use super::*;

unsafe extern "C" fn rockman_rockbuster_shoot_turn_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_TURN, *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK11) as u64, (*FIGHTER_STATUS_ATTR_SCALE_KINETIC_ENERGY | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_1 as u32, 0);
    0.into()
}

unsafe extern "C" fn rockman_rockbuster_shoot_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    rockman_rockbuster_main_helper(fighter, false.into(), false.into(), false.into(), true.into());
    PostureModule::reverse_lr(fighter.module_accessor);
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    if step == 2 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
    rockman_rockbuster_main_helper(fighter, false.into(), false.into(), false.into(), true.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_L_SHOULDER_FIX);
    let ude_motion = MotionModule::motion_kind_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE);
    if ude_motion != hash40("invalid") {
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE, false);
    }
    WorkModule::set_int(fighter.module_accessor, step, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
    WorkModule::set_int(fighter.module_accessor, 4, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack1_turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_turn_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = false;
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_rockman_smash_s02"));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGING) {
        let charge_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCK_BUSTER_CHARGE_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCK_BUSTER_CHARGE_FRAME);
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGING);
        }
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("rockman_chargeshot_hold"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("rockman_chargeshot_elec"), false, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("rockman_chargeshot_hold"), Hash40::new("handl"), 4, 0, 0.5, 0, 0, 0, 0.25+(1.25*(charge_frame as f32/120.0)), true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 0.5, true);
        if charge_frame > 119 {
            fighter.gimmick_flash();
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGED);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_CHARGING);
            if sit == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return 0.into();
        }
    }
    else {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("rockman_chargeshot_hold"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("rockman_chargeshot_elec"), false, true);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCK_BUSTER_CHARGE_FRAME);
    }
    if sit == *SITUATION_KIND_AIR {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
        let status = if helper_ret {FIGHTER_STATUS_KIND_FALL} else {FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR};
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if sit == *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("attack1"));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack1"), end_frame, 1.0, 0.0, false, false);
            let prev_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
            if prev_step == 2 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT_END);
            }
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
            let status = if helper_ret {FIGHTER_STATUS_KIND_WAIT} else {FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT};
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn rockman_rockbuster_shoot_turn_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if !rockman_rockbuster_pre_helper(status_kind.into()).get_bool() {
        rockman_rockbuster_end_var_reset(fighter);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 0);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
    0.into()
}

pub fn install() {
    Agent::new("rockman")
    .status(Pre, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, rockman_rockbuster_shoot_turn_pre_status)
    .status(Main, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, rockman_rockbuster_shoot_turn_main_status)
    .status(End, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, rockman_rockbuster_shoot_turn_end_status)
    .install()
    ;
}