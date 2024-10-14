/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   SPECIAL STATUSES   */

//Super Jump Punch Main, seems to control code regarding Super Jump Punch like moves
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_super_jump_punch_main)]
pub unsafe fn super_jump_punch_main(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let trans_move_speed = MotionModule::trans_move_speed(boma);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return;
    }
    /* START OF NEW ADDITIONS */
    //Permits Mario to be able to Wall Jump during Up Special
    if fighter_kind == *FIGHTER_KIND_MARIO
    && situation_kind == *SITUATION_KIND_AIR {
        if (19.0..37.0).contains(&frame) {
            if !SPECIAL_WALL_JUMP
            && (GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32) || GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32)) 
            && boma.is_cat_flag(Cat1::TurnDash) {
                SPECIAL_WALL_JUMP = true;
                fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), true.into());
            }
        }
        else {
            SPECIAL_WALL_JUMP = false;
        }
    }
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
            if prev_situation_kind == SITUATION_KIND_AIR && situation_kind == SITUATION_KIND_GROUND && trans_move_speed.value[1] < 0.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE) && prev_situation_kind == SITUATION_KIND_AIR && situation_kind == SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
    }
    if MotionModule::is_end(boma) {
        let new_status = WorkModule::get_int(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        fighter.change_status_req(new_status, false);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(super_jump_punch_main);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}