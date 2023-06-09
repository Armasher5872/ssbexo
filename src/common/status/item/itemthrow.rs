/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   ITEM STATUSES   */
//Item Throw
#[skyline::hook(replace = L2CFighterCommon_status_ItemThrowCommon)]
unsafe fn status_itemthrowcommon(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == *FIGHTER_STATUS_KIND_TURN_RUN {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_FROM_TURN_RUN);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_FROM_TURN_RUN);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_STATUS_ITEM_THROW_WORK_FLOAT_LR);
    let item_throw_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
    if item_throw_motion_kind == 0 {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.ItemThrowLightMotionDecisionAir();
        }
        else {
            fighter.ItemThrowLightMotionDecision();
        }
    }
    ControlModule::clear_command(fighter.module_accessor, false);
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[FLICK_X].assign(&0xFE.into());
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_NONE, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_SITUATION);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_LOOP_FIRST);
    /* START OF NEW ADDITION */
    //Allows platform drops out of item throw
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) as i32;
    if GroundModule::is_passable_ground(fighter.module_accessor) && fighter.global_table[STICK_Y].get_f32() < pass_stick_y && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
    }
    /* END OF NEW ADDITION */
    fighter.status_ItemThrow_Loop()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_itemthrowcommon);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}