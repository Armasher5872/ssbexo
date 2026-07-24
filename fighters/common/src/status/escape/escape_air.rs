/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre EscapeAir, used for instant wavedashes
#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
unsafe extern "C" fn status_pre_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let pos = *PostureModule::pos(boma);
    let scale = PostureModule::scale(boma);
    let dir_y = WorkModule::get_float(boma, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let lower_bound = Vector2f::new(pos.x, pos.y-(6.0*scale));
    let ground_pos_any = &mut Vector2f::zero();
    let ground_pos_stage = &mut Vector2f::zero();
    let is_touch_any = GroundModule::line_segment_check(boma, &Vector2f::new(pos.x, pos.y+(6.0*scale)), &lower_bound, &Vector2f::zero(), ground_pos_any, true);
    let is_touch_stage = GroundModule::line_segment_check(boma, &Vector2f::new(pos.x, pos.y+(6.0*scale)), &lower_bound, &Vector2f::zero(), ground_pos_stage, false);
    let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && dir_y > 0.0));
    if prev_status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FALL && WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH) && can_snap {
        GroundModule::attach_ground(boma, true);
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        PostureModule::set_pos(boma, &Vector3f::new(pos.x, ground_pos_any.y, pos.z));
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, 0, 0, 0);
    0.into()
}

//Escape Air
#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir)]
unsafe extern "C" fn status_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let stick_vec = sv_math::vec2_normalize(stick_x, stick_y);
        let escape_air_angle = (stick_vec.y/stick_vec.x.abs()).atan().to_degrees();
        if escape_air_angle > 80.0 {
            MotionModule::change_motion(boma, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("escape_air_slide"), 0.0, 7.0/6.0, false, 0.0, false, false);
        }
    } 
    else {
        MotionModule::change_motion(boma, Hash40::new("escape_air"), 0.0, 7.0/5.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_escapeair_main as *const () as _))
}

unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let dir_x = WorkModule::get_float(boma, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    let dir_y = WorkModule::get_float(boma, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let angle = (dir_y/dir_x).atan().to_degrees();
    let freeze_start_frame = if angle > 80.0 {14.0} else if angle > 45.0 {16.0} else {18.0};
    let freeze_end_frame = if angle > 80.0 {40.0} else if angle > 45.0 {38.0} else {36.0};
    let ledge_grab_enable_frame = if angle > 80.0 {50.0} else if angle > 45.0 {47.0} else {44.0};
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        if (freeze_start_frame..=freeze_end_frame).contains(&frame) {
            KineticModule::unable_energy_all(boma);
            KineticModule::clear_speed_all(boma);
        }
        if frame > freeze_end_frame {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
        if frame >= ledge_grab_enable_frame {
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }
    0.into()
}

//Sub Escape Air Common Main
#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_main)]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boma = fighter.module_accessor;
    let motion_frame = MotionModule::frame(boma);
    let prev_frame = MotionModule::prev_frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"))-1.0;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_escape_air_common_strans_main().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
    }
    /*Start of New Additions*/
    //Halts the current animation on the second to last frame of the animation if the animation of the airdodge is shorter than the cancel frame
    if end_frame < cancel_frame {
        if frame < cancel_frame {
            if prev_frame < end_frame-1.0 && motion_frame >= end_frame-1.0 {
                MotionModule::set_rate(boma, 0.0);
            }
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    /*End of New Additions*/
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

//Setup Escape Air Slide Common
#[skyline::hook(replace = L2CFighterCommon_setup_escape_air_slide_common)]
unsafe extern "C" fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) {
    let boma = fighter.module_accessor;
    let mut escape_air_slide_speed = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_speed"));
    let mut escape_air_slide_stiff_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_stiff_frame"));
    let escape_air_slide_u_stiff_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_u_stiff_frame"));
    let escape_air_slide_d_stiff_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_d_stiff_frame"));
    let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
    let escape_air_slide_back_end_frame = WorkModule::get_param_int(boma, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
    let escape_air_add_xlu_start_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    let stick_vec = sv_math::vec2_normalize(stick_x.get_f32(), stick_y.get_f32());
    let escape_air_angle = (stick_vec.y/stick_vec.x.abs()).atan().to_degrees();
    if escape_air_angle > 80.0 {
        escape_air_slide_speed = 2.5;
    }
    if escape_air_angle > 45.0 && escape_air_angle <= 80.0 {
        escape_air_slide_speed = 3.0;
    }
    let escape_air_slide_speed_vec = Vector2f{x: escape_air_slide_speed*stick_vec.x, y: escape_air_slide_speed*stick_vec.y};
    let lerp;
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        WorkModule::set_float(boma, stick_vec.x, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        WorkModule::set_float(boma, stick_vec.y, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, escape_air_slide_speed_vec.x, escape_air_slide_speed_vec.y, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.11);
        if escape_air_angle < 0.0 {
            lerp = (escape_air_angle*-1.0)/90.0;
            escape_air_slide_stiff_frame = Lerp::lerp(&lerp, &escape_air_slide_d_stiff_frame, &escape_air_slide_stiff_frame);
        }
        else {
            lerp = escape_air_angle/90.0;
            escape_air_slide_stiff_frame = Lerp::lerp(&lerp, &escape_air_slide_u_stiff_frame, &escape_air_slide_stiff_frame);
        }
        WorkModule::set_float(boma, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
        WorkModule::set_int(boma, escape_air_slide_back_end_frame+escape_air_add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
        WorkModule::set_float(boma, escape_air_slide_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
        EffectModule::req_on_joint(boma, Hash40::new("sys_smash_flash_s"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 4.0, z: 8.0}, &Vector3f::zero(), 1.1, &Vector3f{x: 18.0, y: 12.0, z: 0.0}, &Vector3f::zero(), false, 0, 0, 0);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_escapeair,
            status_escapeair,
            sub_escape_air_common_main,
            setup_escape_air_slide_common
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}