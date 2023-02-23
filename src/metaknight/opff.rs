use {
    crate::custom::gliding::*,
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smashline::*,
    smash_script::*,
};

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let stick_x = ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let params = GlideParams::get(fighter);
        //Dair
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
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
            WorkModule::set_int(fighter.module_accessor, 6, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            if frame > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if frame > 25.0 {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: params.speed_mul_start, y: 0.0, z: 0.0});
            }   
            if frame >= 29.0 && frame < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
            if frame > 32.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            fighter.sub_air_check_fall_common();
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
            WorkModule::set_int(fighter.module_accessor, 6, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            if frame > 22.0 {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
            if frame > 25.0 {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: params.speed_mul_start, y: 0.0, z: 0.0});
            }    
            if frame >= 29.0 && frame < 30.0 {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_start"));
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_metaknight_glide_loop"));
            }
            if frame > 31.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            };
        }
    }
}

pub fn install() {
    install_agent_frames!(
        metaknight_frame
    );
}