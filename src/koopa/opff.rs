//Credit to Ultimate S
use super::*;

unsafe extern "C" fn fireball_gfx_frame(weapon: &mut L2CFighterBase) {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    let entry_id = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let color = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if CAN_FIREBALL[entry_id] == true {
        AttackModule::set_power_up(weapon.module_accessor, 1.0);
        if FIREBALL_GFX[entry_id] % 14 == 0 {
            if color == 7 {
                EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
                let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(boma, f1, 1.0, 1.0, 0.333);
                EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
            else if color == 3 {
                EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
                let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(boma, f1, 0.502, 0.494, 0.929);
                EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
            else {
                EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
                let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
                EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
        };
    }
    if CAN_FIREBALL[entry_id] == false {
        AttackModule::set_power_up(weapon.module_accessor, 0.2);
    }
}

pub fn install() {
    Agent::new("koopa_breath")
    .on_line(Main, fireball_gfx_frame)
    .install()
    ;
}