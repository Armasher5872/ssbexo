use {
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
        phx::Hash40,
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let stick_x = ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR 
        && motion_kind == hash40("attack_air_lw") {
            if stick_x > 0.5
            && frame < 7.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_lw_diagonal_r"), -1.0, 1.0, 0.0, false, false);
            }
            if stick_x < -0.5
            && frame < 7.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_lw_diagonal_l"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        metaknight_frame
    );
}