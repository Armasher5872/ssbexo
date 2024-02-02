/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   LANDING STATUSES   */
//Landing Main Sub
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
pub unsafe fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
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

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_landing_main_sub);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}