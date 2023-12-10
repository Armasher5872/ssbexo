use super::*;

unsafe extern "C" fn yoshi_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    }
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_TREAD_JUMP
    && motion_kind == hash40("attack_air_lw")
    && (12.0..=36.0).contains(&frame)
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.015);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.05);
    }
}

pub fn install() {
    Agent::new("yoshi")
    .on_line(Main, yoshi_frame)
    .install()
    ;
}