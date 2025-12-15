use super::*;

unsafe extern "C" fn kirby_special_hi_2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let speed = smash::phx::Vector3f{ x: stick_x/2.0, y: 2.5, z: 0.0 };
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM);
    fun_71001a6740(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi2") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    KineticModule::add_speed(fighter.module_accessor, &speed);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_2_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_hi_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.sub_off_passive_opponent(L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_TARGET_ID), L2CValue::I32(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_METEOR_HIT_NUM), false.into());
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_CANCEL) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if frame > 1.0 {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5.into(), false.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 5);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_2_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5].contains(&status_kind) {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_pre_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_main_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, kirby_special_hi_2_exit_status)
    .install()
    ;
}