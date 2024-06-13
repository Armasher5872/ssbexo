use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_pfushigisou_neutral_special_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let speed = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, speed*lr, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if ItemModule::get_have_item_kind(agent.module_accessor, 0) == *ITEM_KIND_DEKU {
            ItemModule::throw_item(agent.module_accessor, 80.0, 2.3, 1.0, 0, true, 0.0);
            PFUSHIGISOU_IS_ACTIVE_BOMB[entry_id] = true;
        }
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .game_acmd("game_specialnstart", ssbexo_pfushigisou_neutral_special_acmd, Priority::Low)
    .game_acmd("game_specialairnstart", ssbexo_pfushigisou_neutral_special_acmd, Priority::Low)
    .install()
    ;
}