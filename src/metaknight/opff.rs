use super::*;

unsafe extern "C" fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let lr = PostureModule::lr(boma);
    let base_power = WorkModule::get_param_float(boma, hash40("param_glide"), hash40("base_power"));
    //Glide SFX
    if [
        *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT, *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
    ].contains(&status_kind) { 
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    };
    //Up Special Energy Set & SFX
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) {
        fighter.sub_air_check_fall_common();
        if frame > 22.0 {
            WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        }
        if frame > 25.0 {
            KineticUtility::reset_enable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, boma, *ENERGY_STOP_RESET_TYPE_FREE, &Vector2f{x: base_power*lr, y: 0.0}, &Vector3f{x: base_power*lr, y: 0.0, z: 0.0});
        }   
        if (29.0..30.0).contains(&frame) {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
        }
    }
    println!("Galaxia Beam Exist: {}", ArticleModule::is_exist(boma, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_GALAXIA_BEAM));
}

pub fn install() {
    Agent::new("metaknight")
    .on_line(Main, metaknight_frame)
    .install()
    ;
}