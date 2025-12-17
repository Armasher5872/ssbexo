use super::*;

//Up Special Glide Sound
unsafe extern "C" fn ssbexo_link_special_hi_glide_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let sound = SoundModule::play_status_se(agent.module_accessor, Hash40::new("se_link_appear01"), true, false, false);
        SoundModule::set_se_vol(agent.module_accessor, sound as i32, 1.2, 0);
    }
}

//Up Special Glide Expression
unsafe extern "C" fn ssbexo_link_special_hi_glide_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
        }
        wait(agent.lua_state_agent, 25.0);
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_specialhiglide", ssbexo_link_special_hi_glide_sound, Low)
    .expression_acmd("expression_specialhiglide", ssbexo_link_special_hi_glide_expression, Low)
    .sound_acmd("sound_specialhiglidef", ssbexo_link_special_hi_glide_sound, Low)
    .expression_acmd("expression_specialhiglidef", ssbexo_link_special_hi_glide_expression, Low)
    .sound_acmd("sound_specialhiglideb", ssbexo_link_special_hi_glide_sound, Low)
    .expression_acmd("expression_specialhiglideb", ssbexo_link_special_hi_glide_expression, Low)
    .install()
    ;
}