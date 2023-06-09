/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   LANDING STATUSES   */
//Landing Main Sub
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
pub unsafe fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR || StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
        let shield_drop_check = (fighter.global_table[CMD_CAT2].get_i32() & (*FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0) || fighter.down_input() == true;
        if GroundModule::is_passable_ground(boma) {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
        && shield_drop_check
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH)) {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return true.into();
        }
    }
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_landing_main_sub);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}