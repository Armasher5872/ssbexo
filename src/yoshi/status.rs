use super::*;

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn yoshi_jump_aerial_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Script is effectively vanilla script, except the Armor Values were removed
    fighter.status_JumpAerialSub(false.into(), false.into());
    let turn_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_cont_value"));
    if fighter.global_table[STICK_X].get_f32() * -1.0 * PostureModule::lr(fighter.module_accessor) > turn_cont_value {
        let turn_count = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_count"));
        WorkModule::set_int(fighter.module_accessor, turn_count, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    fighter.status_JumpAerial_Main();
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn yoshi_attack_air_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_init();
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn yoshi_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_main_status(fighter)
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn yoshi_attack_air_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec();
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn yoshi_attack_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit();
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn yoshi_neutral_special_1_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let mut pos: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    pos = GroundModule::hang_cliff_pos_3f(fighter.module_accessor);
    if (19.0..=23.0).contains(&frame)
    && pos.y < -3.0
    && (pos.x.abs() > 1.0 && pos.x.abs() < 6.0) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_tether"), 0.0, 1.0, false, 0.0, false, false);
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        yoshi_jump_aerial_main_status,
        yoshi_attack_air_init_status,
        yoshi_attack_air_main_status,
        yoshi_attack_air_exec_status,
        yoshi_attack_air_exit_status,
        yoshi_neutral_special_1_exec_status
    );
}