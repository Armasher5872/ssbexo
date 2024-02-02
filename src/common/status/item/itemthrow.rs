/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   ITEM STATUSES   */

//Pre Item Throw
#[skyline::hook(replace = L2CFighterCommon_status_pre_ItemThrow)]
unsafe fn status_pre_itemthrow(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kinetic_condition;
    let arg_6_check;
    let treaded_kind;
    if !fighter.sub_pre_ItemThrow().get_bool() {
        if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
            kinetic_condition = *FIGHTER_KINETIC_TYPE_MOTION;
            arg_6_check = true;
            treaded_kind = *FIGHTER_TREADED_KIND_NO_REAC;
        }
        else if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
            kinetic_condition = *FIGHTER_KINETIC_TYPE_MOTION_FALL;
            arg_6_check = false;
            treaded_kind = *FIGHTER_TREADED_KIND_DISABLE;
        }
        else {
            kinetic_condition = *FIGHTER_KINETIC_TYPE_UNIQ;
            arg_6_check = true;
            treaded_kind = *FIGHTER_TREADED_KIND_NO_REAC;
        }
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), kinetic_condition, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), arg_6_check, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, treaded_kind, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_THROW_ITEM | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,  0, 0, 0);
    }
    0.into()
}

//Item Throw Common
#[skyline::hook(replace = L2CFighterCommon_status_ItemThrowCommon)]
unsafe fn status_itemthrowcommon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let item_throw_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_flick_y")) as i32;
    if prev_status_kind != *FIGHTER_STATUS_KIND_TURN_RUN {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_FROM_TURN_RUN);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_FROM_TURN_RUN);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_STATUS_ITEM_THROW_WORK_FLOAT_LR);
    if item_throw_motion_kind == 0 {
        if situation_kind != *SITUATION_KIND_GROUND {
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
    if GroundModule::is_passable_ground(fighter.module_accessor) && fighter.global_table[STICK_Y].get_f32() < pass_stick_y && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
        fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
    }
    /* END OF NEW ADDITION */
    fighter.status_ItemThrow_Loop()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_itemthrow,
            status_itemthrowcommon
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}