use super::*;

unsafe extern "C" fn ssbexo_armstrong_firepillar_burst_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 180.0);
    if is_excute(agent) {
        let charge = WorkModule::get_float(boma, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_CHARGE);
        let damage_a = 8.0+(10.0*charge);
        let damage_b = 10.0+(10.0*charge);
        ATTACK(agent, 0, 0, Hash40::new("top"), damage_a, 361, 80, 0, 60, 6.0, 0.0, 7.0, 0.0, Some(0.0), Some(22.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage_b, 361, 80, 0, 60, 6.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn ssbexo_armstrong_firepillar_burst_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let owner_boma = get_owner_boma(agent);
        let owner_lr = PostureModule::lr(owner_boma);
        let owner_pos_x = PostureModule::pos_x(owner_boma);
        let owner_pos_y = PostureModule::pos_y(owner_boma);
        let owner_pos_z = PostureModule::pos_z(owner_boma);
        let pos_x = WorkModule::get_float(boma, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_X);
        let pos_y = WorkModule::get_float(boma, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_FLOAT_POS_Y);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        let effect = EffectModule::req(boma, Hash40::new("armstrong_ground_flame_crack"), &Vector3f{x: owner_pos_x+(pos_x*owner_lr), y: owner_pos_y+pos_y, z: owner_pos_z}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        WorkModule::set_int(boma, effect as i32, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        let effect = WorkModule::get_int(boma, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
        EffectModule::set_rate(boma, effect as u32, 0.2);
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("armstrong_blast_shock"), Hash40::new("top"), 0, 11, -3, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.15);
    }
    frame(lua_state, 180.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        let effect = EffectModule::req_on_joint(boma, Hash40::new("armstrong_flame_pillar"), Hash40::new("top"), &Vector3f::zero(), &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, -1, 0);
        WorkModule::set_int(boma, effect as i32, *WEAPON_ARMSTRONG_FIREPILLAR_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
}

unsafe extern "C" fn ssbexo_armstrong_firepillar_burst_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 180.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

pub fn install() {
    Agent::new("ganon_firepillar")
    .set_costume(get_armstrong_costumes_acmd())
    .acmd("game_burst", ssbexo_armstrong_firepillar_burst_acmd, Low)
    .acmd("effect_burst", ssbexo_armstrong_firepillar_burst_effect, Low)
    .acmd("sound_burst", ssbexo_armstrong_firepillar_burst_sound, Low)
    .install()
    ;
}