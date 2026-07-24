use super::*;

unsafe extern "C" fn gaogaen_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    ItemModule::set_have_item_visibility(boma, false, 0);
    gaogaen_sub_status_catch(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_catch_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_sub_status_catch(fighter: &mut L2CFighterCommon) {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    if stick_y >= 0.7 {
        MotionModule::change_motion(boma, Hash40::new("catch_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if stick_y <= -0.7 {
        MotionModule::change_motion(boma, Hash40::new("catch_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("catch"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
}

unsafe extern "C" fn gaogaen_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(boma) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 1.into();
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_CATCH, gaogaen_catch_main_status)
    .install()
    ;
}