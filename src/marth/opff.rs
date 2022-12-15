use {
    crate::functions::special_lw_mot_helper,
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_MARTH )]
fn marth_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
        && frame > 28.0 {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
            special_lw_mot_helper(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        marth_frame
    );
}