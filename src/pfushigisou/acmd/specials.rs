#![allow(unused_macros)]
use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::L2CAgentBase
    },
    smashline::*,
    smash_script::*,
};

//Neutral Special ACMD
#[acmd_script( agent = "pfushigisou", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME)]
unsafe fn ssbuexo_ivysaur_neutral_special_acmd(fighter: &mut L2CAgentBase) {
    let speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, speed*lr, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_DEKU {
            ItemModule::throw_item(fighter.module_accessor, 80.0, 2.3, 0.0, 0, true, 0.0);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_ivysaur_neutral_special_acmd
    );
}