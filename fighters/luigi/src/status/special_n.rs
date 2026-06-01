use super::*;

unsafe extern "C" fn luigi_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(boma, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(boma, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let is_attack_active = WorkModule::is_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ATTACK_ACTIVE);
    let held_neutral_special_timer = WorkModule::get_int(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_N_HELD_TIMER);
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(boma) {
        if StatusModule::is_situation_changed(boma) {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !is_attack_active && current_frame < 12.0 {
        WorkModule::inc_int(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_N_HELD_TIMER);
    }
    if held_neutral_special_timer >= 12 {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ATTACK_ACTIVE);
    }
    if current_frame > 12.0 && is_attack_active {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_N_ATTACK.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, luigi_special_n_main_status)
    .install()
    ;
}