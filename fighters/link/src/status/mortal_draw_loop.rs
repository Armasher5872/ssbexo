//Credit to AParticularUser for Mortal Draw code
use super::*;

unsafe extern "C" fn link_mortal_draw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn link_mortal_draw_loop_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_mortal_draw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("mortal_draw_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_mortal_draw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn link_mortal_draw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK.into(), false.into());
        return 1.into();
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 95.0, 1.0, false, 5.0, true, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn link_mortal_draw_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_mortal_draw_loop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_mortal_draw_loop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, link_mortal_draw_loop_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, link_mortal_draw_loop_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, link_mortal_draw_loop_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, link_mortal_draw_loop_exec_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, link_mortal_draw_loop_end_status)
    .status(Exit, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, link_mortal_draw_loop_exit_status)
    .install()
    ;
}