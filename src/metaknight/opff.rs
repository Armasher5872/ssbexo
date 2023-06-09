use super::*;

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let params = crate::common::status::aerial::gliding::GlideParams::get(fighter);
        let lr = PostureModule::lr(module_accessor);
        //Glide SFX
        if [
            *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind) { 
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
        };
        //Up Special Glide Transition
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if frame > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if frame > 25.0 {
                crate::common::status::aerial::gliding::KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: params.base_speed * lr, y: 0.0}, Vector3f{x: params.base_speed * lr, y: 0.0, z: 0.0});
            }   
            if (29.0..30.0).contains(&frame) {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            if frame > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if frame > 25.0 {
                crate::common::status::aerial::gliding::KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, Vector2f{x: params.base_speed * lr, y: 0.0}, Vector3f{x: params.base_speed * lr, y: 0.0, z: 0.0});
            }    
            if (29.0..30.0).contains(&frame) {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        metaknight_frame
    );
}