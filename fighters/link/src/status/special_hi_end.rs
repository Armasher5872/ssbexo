use super::*;

unsafe extern "C" fn link_special_hi_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_end_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_spin_attack"), 0.0, 1.0, false, 0.0, false, false);
    let rslash_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, rslash_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_end_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, link_special_hi_end_end_status)
    .install()
    ;
}