/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   LANDING STATUSES   */
//Landing Light Sub
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_LandingLight_Main)]
pub unsafe fn status_landinglight_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret: i32 = 0;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
        || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
            ret = 0.into();
        }
        let get_have_item_kind = ItemModule::get_have_item_kind(boma, 0);
        if get_have_item_kind == *ITEM_KIND_ASSIST {
            if !MotionModule::is_end(boma) {
                fighter.sub_landing_cancel_check_damage_face();
                ret = 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if !fighter.sub_landing_uniq_check_strans().get_bool() {
            if fighter.sub_landing_ground_check_common().get_bool() {
                ret = 1.into();
            }
            ret = 0.into();
        }
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
            if GroundModule::is_passable_ground(boma)
            && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y
            && fighter.global_table[STICK_Y].get_f32() < pass_stick_y {
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

//Status End Landing
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_LandingLight)]
unsafe fn status_end_landinglight(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP);
    fighter.sub_landing_cancel_damage_face();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_landinglight_main,
            status_end_landinglight
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}