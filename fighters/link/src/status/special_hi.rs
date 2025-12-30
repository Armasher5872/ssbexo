use super::*;

unsafe extern "C" fn link_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.15);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_start"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(), false.into());
        }
        else {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START.into(), false.into());
            }
        }
        return 1.into();
    }
    0.into()  
}

unsafe extern "C" fn link_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH].contains(&status_kind) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH].contains(&status_kind) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    }
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, link_special_hi_exit_status)
    .install()
    ;
}