use super::*;

//Bowser Jr Cannonball On Attack Offset
unsafe extern "C" fn koopajr_cannonball_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let pos = *PostureModule::pos(boma);
    if owner_kind == *FIGHTER_KIND_PURIN {
        EffectModule::req(boma, Hash40::new("sys_flash"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("poke_meloetta_bullet"), false, false);
        EffectModule::kill_kind(boma, Hash40::new("rosetta_ring_erase"), false, false);
    }
    if [*FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_IKE].contains(&owner_kind) {
        EffectModule::req(boma, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    }
    if owner_kind == *FIGHTER_KIND_PFUSHIGISOU {
        EffectModule::req(boma, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
    }
    if owner_kind == *FIGHTER_KIND_KOOPAJR {
        agent.clear_lua_stack();
        lua_args!(agent, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_bomb_b"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        sv_module_access::effect(agent.lua_state_agent);
    }
    *(weapon as *mut bool).add(0x90) = false;
    normal_weapon_hit_handler(vtable, weapon, log)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51d93e8).data(koopajr_cannonball_on_attack as u64);
}