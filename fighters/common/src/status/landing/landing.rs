/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
#![allow(unused_assignments)] //Addresses warning: value assigned to `ret` is never read
use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Landing_param)]
unsafe extern "C" fn status_pre_landing_param(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue, param_6: L2CValue) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), param_5.get_i32(), *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, param_2.get_i32(), param_3.get_i32(), param_4.get_i32(), param_6.get_i32());
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

//Landing Main Sub
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
unsafe extern "C" fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret: i32 = 0;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
        || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
            ret = 0;
        }
        let get_have_item_kind = ItemModule::get_have_item_kind(boma, 0);
        if get_have_item_kind == *ITEM_KIND_ASSIST {
            if !MotionModule::is_end(boma) {
                ret = 0;
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if !fighter.sub_landing_uniq_check_strans().get_bool() {
            if fighter.sub_landing_ground_check_common().get_bool() {
                ret = 1;
            }
            ret = 0;
        }
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
			ControlModule::reset_main_stick_x(boma);
            if GroundModule::is_passable_ground(boma) && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y {
                fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
                return 1.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    ret.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_landing_param,
            status_landing_main_sub
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}