use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_CapturePulled_common)]
unsafe extern "C" fn status_pre_capturepulled_common(fighter: &mut L2CFighterCommon, param_1: &mut L2CValue, param_2: &mut L2CValue) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(param_1.get_i32()), *FIGHTER_KINETIC_TYPE_CAPTURE, param_2.get_u32(), GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_KEEP_2NDARY) as u32, 0, 0);
    PostureModule::set_lr(fighter.module_accessor, -LinkModule::get_parent_lr(fighter.module_accessor, *LINK_NO_CAPTURE));
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CapturePulled_Main)]
unsafe extern "C" fn status_capturepulled_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.CapturePulledCommon_Main();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_WAIT) {
        if !fighter.is_capture_pulled_special_fighter().get_bool() {
            if !MotionModule::is_end(fighter.module_accessor) {
                if !fighter.is_capture_pulled_special_fighter().get_bool() {
                    return 0.into();
                }
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_MOTION_END_FLAG);
            }
            fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_WAIT.into(), false.into());
        }
        else {
            if !MotionModule::is_end(fighter.module_accessor)
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_MOTION_END_FLAG) {
                if !fighter.is_capture_pulled_special_fighter().get_bool() {
                    return 0.into();
                }
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_MOTION_END_FLAG);
            }
            fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CapturePulled)]
unsafe fn status_end_capturepulled(fighter: &mut L2CFighterCommon) -> L2CValue {
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(fighter.module_accessor, flags[x]);
    }
    //Fighter Specific
    WorkModule::sub_int(fighter.module_accessor, 1, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            //status_pre_capturepulled_common,
            status_capturepulled_main,
            status_end_capturepulled
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}