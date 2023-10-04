use super::*;

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn yoshi_jump_aerial_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Script is effectively vanilla script, except the Armor Values were removed
    let turn_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_cont_value"));
    if fighter.global_table[STICK_X].get_f32() * -1.0 * PostureModule::lr(fighter.module_accessor) > turn_cont_value {
        let turn_count = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_count"));
        WorkModule::set_int(fighter.module_accessor, turn_count, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    } 
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.status_JumpAerial();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        yoshi_jump_aerial_main_status
    );
}