use super::*;

unsafe extern "C" fn ssbexo_armstrong_firepillar_burst_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 180.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 6.0, 0.0, 7.0, 0.0, Some(0.0), Some(22.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 80, 0, 60, 6.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn ssbexo_armstrong_firepillar_burst_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        let owner_boma = get_owner_boma(agent);
        let owner_lr = PostureModule::lr(owner_boma);
        let owner_pos_x = PostureModule::pos_x(owner_boma);
        let owner_pos_y = PostureModule::pos_y(owner_boma);
        let owner_pos_z = PostureModule::pos_z(owner_boma);
        let pos_x = WorkModule::get_float(agent.module_accessor, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
        let pos_y = WorkModule::get_float(agent.module_accessor, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        let effect = EffectModule::req(agent.module_accessor, Hash40::new("armstrong_ground_flame_crack"), &Vector3f{x: owner_pos_x+(pos_x*owner_lr), y: owner_pos_y+pos_y, z: owner_pos_z}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        WorkModule::set_int(agent.module_accessor, effect as i32, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        let effect = WorkModule::get_int(agent.module_accessor, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
        EffectModule::set_rate(agent.module_accessor, effect as u32, 0.2);
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_shock"), Hash40::new("top"), 0, 11, -3, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.15);
    }
    frame(agent.lua_state_agent, 180.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        let effect = EffectModule::req_on_joint(agent.module_accessor, Hash40::new("armstrong_flame_pillar"), Hash40::new("top"), &Vector3f::zero(), &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, -1, 0);
        WorkModule::set_int(agent.module_accessor, effect as i32, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
}

pub fn install() {
    Agent::new("ganon_firepillar")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_burst", ssbexo_armstrong_firepillar_burst_acmd, Low)
    .effect_acmd("effect_burst", ssbexo_armstrong_firepillar_burst_effect, Low)
    .install()
    ;
}