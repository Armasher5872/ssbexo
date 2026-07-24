use super::*;

unsafe extern "C" fn edge_escape_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let stick_vec = sv_math::vec2_normalize(stick_x, stick_y);
        let escape_air_angle = (stick_vec.y/stick_vec.x.abs()).atan().to_degrees();
        if escape_air_angle > 60.0 {
            MotionModule::change_motion(boma, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("escape_air_slide"), 0.0, 7.0/6.0, false, 0.0, false, false);
        }
    } 
    else {
        MotionModule::change_motion(boma, Hash40::new("escape_air"), 0.0, 7.0/5.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_escape_air_main_loop as *const () as _))
}

unsafe extern "C" fn edge_escape_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boma = fighter.module_accessor;
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        if (25.0..=46.0).contains(&frame) {
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        if frame > 46.0 {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    if edge_check_valid_wing_enable(boma) && WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ESCAPE_AIR_ENABLE_WING) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_WING_ACTIVATE.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn edge_escape_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ESCAPE_AIR_ENABLE_WING);
    fighter.status_end_EscapeAir()
}

unsafe extern "C" fn edge_escape_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ESCAPE_AIR_ENABLE_WING);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, edge_escape_air_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR, edge_escape_air_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ESCAPE_AIR, edge_escape_air_exit_status)
    .install()
    ;
}