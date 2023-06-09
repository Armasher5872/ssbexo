use super::*;

#[status_script(agent = "fox", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn fox_special_lw_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    //If you've hit the move and haven't hit shield, transition to jump cancel
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        if fighter.jump_cancel() {
            //If you can, transition into the appropriate status kind
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
            }
        }
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() >= 4.0 {
        //If you can, transition into the appropriate status kind
        if fighter.jump_cancel() {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
            }
        }
    }
    original!(fighter)
}

#[status_script(agent = "fox", status = FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn fox_special_lw_loop_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    //Checks if the current frame in the status is greater than or equal to 4.0 (Effectively 8 frames into Shine)
    if fighter.global_table[CURRENT_FRAME].get_f32() >= 4.0 {
        //If you can, transition into the appropriate status kind
        if fighter.jump_cancel() {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
            }
        }
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        fox_special_lw_exec_status,
        fox_special_lw_loop_exec_status
    );
}