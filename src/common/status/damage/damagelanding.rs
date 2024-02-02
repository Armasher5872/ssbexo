/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_LandingStiffness)]
unsafe fn status_landingstiffness(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_stiffness_frame = fighter.FL_get_LandingStiffness().get_f32();
    WorkModule::set_float(fighter.module_accessor, landing_stiffness_frame, *FIGHTER_STATUS_LANDING_WORK_FLOAT_STIFFNESS_FRAME);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FL_get_LandingStiffness)]
unsafe fn fl_get_landingstiffness(fighter: &mut L2CFighterCommon) -> f32 {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame") as u64, 0);
    let damage_reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let mut endlag: f32 = 0.0;
    if ![*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR].contains(&prev_status_kind) {
        if prev_status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO {
            endlag = landing_frame;
        }
        else {
            let air_lasso_landing_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_LANDING_FRAME);
            endlag = air_lasso_landing_frame as f32;
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC) {
            //Halves hitstun on non tumble Crouch Cancels.
            let hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
            WorkModule::set_float(fighter.module_accessor, hitstun*0.5, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        }
    }
    if landing_frame >= damage_reaction_frame {
        endlag = landing_frame;
    }
    else {
        endlag = damage_reaction_frame;
    }
    landing_frame
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_landingstiffness,
            fl_get_landingstiffness
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}