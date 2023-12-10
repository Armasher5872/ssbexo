//Credit to C# and WuBoy
use super::*;

unsafe extern "C" fn plizardon_throw_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    return false.into();
}

unsafe extern "C" fn plizardon_throw_kirby_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_ThrowKirby();
}

unsafe extern "C" fn plizardon_throw_kirby_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_uniq_process_ThrowKirby_initStatus();
    let hit_stop = 8;
    WorkModule::set_int(fighter.module_accessor, hit_stop, *FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME);
    return false.into();
}

unsafe extern "C" fn plizardon_throw_kirby_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_ThrowKirby();
}

unsafe extern "C" fn plizardon_throw_kirby_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
    let capture_boma = smash::app::sv_battle_object::module_accessor(capture_id);
    let frame_fallloop = FRAME_FALL+2.0;
    let current_frame = MotionModule::frame(boma);
    let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
    let damage_plizardon = DamageModule::damage(boma, 0);
    let damage_opponent = DamageModule::damage(capture_boma, 0);
    let max_damage = 30.0;
    let damage_factor = 1.0+((damage_plizardon-damage_opponent).abs()/max_damage);
    let influence_factor: f32 = 0.05*damage_factor.max(0.3);
    let influence = if stick_x.abs() < 0.2 {0.0} else {influence_factor*stick_x.signum()}; 
    CameraModule::set_enable_camera(boma, true, 0);
    CameraModule::set_enable_update_pos(boma, *CAMERA_UPDATE_POS_Y as u8, 0);
    if current_frame >= FRAME_LAND {
        let grounded = GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
        let last_frame = if grounded {MotionModule::end_frame(boma)-3.0} else {MotionModule::end_frame(boma)-9.0};
        if current_frame >= last_frame { 
            if !grounded {
                let speed = smash::phx::Vector3f { x: 0.0, y: -0.1, z: 0.0 };
                KineticModule::add_speed(boma, &speed);
            }
        }
        return false.into();
    }
    //If we go past a certain frame, then freeze animation and accel downwards, and add speed based on your control stick, your percentage, your opponents control stick, and your opponents percentage
    if current_frame >= frame_fallloop && current_frame < FRAME_LAND {
        MotionModule::set_rate(boma, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let speed = smash::phx::Vector3f { x: influence, y: -12.0, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    //Groundcast to see if we touched the ground (only after falling), then cut to the landing frame
    if current_frame >= FRAME_FALL {
        if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32) {
            MotionModule::set_frame_sync_anim_cmd(boma, FRAME_LAND, true, true, false);
            MotionModule::set_rate(boma, 1.0);
            KineticModule::resume_energy(boma,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            //Effects
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_line"), false, false);
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 5, 0, 5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
            macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_l"));
        }
    }
    return false.into();
}

unsafe extern "C" fn plizardon_throw_kirby_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_ThrowKirby();
}

unsafe extern "C" fn plizardon_throw_kirby_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.sub_status_uniq_process_ThrowKirby_exitStatus();
}

pub fn install() {
    Agent::new("plizardon")
    .status(Exec, *FIGHTER_STATUS_KIND_THROW, plizardon_throw_exec_status)
    .status(Pre, *FIGHTER_STATUS_KIND_THROW_KIRBY, plizardon_throw_kirby_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_THROW_KIRBY, plizardon_throw_kirby_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW_KIRBY, plizardon_throw_kirby_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_THROW_KIRBY, plizardon_throw_kirby_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW_KIRBY, plizardon_throw_kirby_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_THROW_KIRBY, plizardon_throw_kirby_exit_status)
    .install()
    ;
}