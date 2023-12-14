use super::*;

unsafe extern "C" fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(boma);
    //Glide SFX
    if [
        *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
    ].contains(&status_kind) { 
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    };
}

pub fn install() {
    Agent::new("metaknight")
    .on_line(Main, metaknight_frame)
    .install()
    ;
}