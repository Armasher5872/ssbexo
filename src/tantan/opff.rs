use super::*;

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        /*
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        //let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        HitModule::set_status_joint(boma, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr1"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr2"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr4"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr5"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("handr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml1"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml2"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml4"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml5"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("handl"), HitStatus(*HIT_STATUS_NORMAL), 0);
        if [
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL_AERIAL, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BACK,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BRAKE, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BRAKE_BACK
        ].contains(&status_kind) {
            HitModule::set_status_joint(boma, Hash40::new("arml"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("arml1"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("arml2"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("arml4"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("arml5"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("handl"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("armr"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("armr1"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("armr2"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("armr4"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("armr5"), HitStatus(*HIT_STATUS_NORMAL), 0);
            HitModule::set_status_joint(boma, Hash40::new("handr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        */
    }
}

pub fn install() {
    install_agent_frames!(tantan_frame);
}