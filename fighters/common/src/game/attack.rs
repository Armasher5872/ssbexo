//The following section is credited to HewDraw Remix, and is utilized to get the location and hitbox id of the latest hit, and removes random hit tripping
use super::*;

//Hit Module Handle Attack Event, determines where you hit and with what hitbox id
#[skyline::hook(offset = 0x46ae84, inline)]
unsafe extern "C" fn hit_module_handle_attack_event(ctx: &InlineCtx) {
    let data = ctx.registers[1].x() as *mut u32;
    let attacker_battle_object = get_battle_object_from_id(*data);
    let attacker_category = ((*attacker_battle_object).battle_object_id >> 0x1C) as i32;
    let attacker_boma = (*attacker_battle_object).module_accessor;
    if ![*BATTLE_OBJECT_CATEGORY_FIGHTER, *BATTLE_OBJECT_CATEGORY_WEAPON, *BATTLE_OBJECT_CATEGORY_ITEM].contains(&attacker_category) {
        return;
    }
    let collision_data = ctx.registers[27].x() as *mut f32;
    if attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        WorkModule::set_int(attacker_boma, *data.add(1) as i32, *FIGHTER_INSTANCE_WORK_ID_INT_LAST_ATTACK_HITBOX_ID);
        WorkModule::set_float(attacker_boma, *collision_data.add(4), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_X);
        WorkModule::set_float(attacker_boma, *collision_data.add(5), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Y);
        WorkModule::set_float(attacker_boma, *collision_data.add(6), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Z);
    }
    if attacker_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = sv_battle_object::module_accessor(owner_id);
        WorkModule::set_int(owner_boma, *data.add(1) as i32, *FIGHTER_INSTANCE_WORK_ID_INT_LAST_ATTACK_HITBOX_ID);
        WorkModule::set_float(owner_boma, *collision_data.add(4), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_X);
        WorkModule::set_float(owner_boma, *collision_data.add(5), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Y);
        WorkModule::set_float(owner_boma, *collision_data.add(6), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Z);
    }
}

//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield
#[skyline::hook(offset = 0x4c7080)]
unsafe extern "C" fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
    let attacker_id = *(collision.add(0x24) as *const u32);
	let attacker_battle_object = get_battle_object_from_id(attacker_id);
    let attacker_category = ((*attacker_battle_object).battle_object_id >> 0x1C) as i32;
    let attacker_boma = (*attacker_battle_object).module_accessor;
    if ![*BATTLE_OBJECT_CATEGORY_FIGHTER, *BATTLE_OBJECT_CATEGORY_WEAPON, *BATTLE_OBJECT_CATEGORY_ITEM].contains(&attacker_category) {
        return;
    }
    if attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        WorkModule::set_int(attacker_boma, *(collision.add(0x33) as *const u8) as i32, *FIGHTER_INSTANCE_WORK_ID_INT_LAST_ATTACK_HITBOX_ID);
        WorkModule::set_float(attacker_boma, *(collision.add(0x10) as *const f32), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_X);
        WorkModule::set_float(attacker_boma, *(collision.add(0x14) as *const f32), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Y);
        WorkModule::set_float(attacker_boma, *(collision.add(0x18) as *const f32), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Z);
    }
    if attacker_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = sv_battle_object::module_accessor(owner_id);
        WorkModule::set_int(owner_boma, *(collision.add(0x33) as *const u8) as i32, *FIGHTER_INSTANCE_WORK_ID_INT_LAST_ATTACK_HITBOX_ID);
        WorkModule::set_float(owner_boma, *(collision.add(0x10) as *const f32), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_X);
        WorkModule::set_float(owner_boma, *(collision.add(0x14) as *const f32), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Y);
        WorkModule::set_float(owner_boma, *(collision.add(0x18) as *const f32), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LAST_ATTACK_HITBOX_LOCATION_Z);
    }
}

//Attack Module Set Attack, makes it so random tripping doesn't happen if the move doesn't have a 100% trip chance
#[skyline::hook(offset = 0x3dc180)]
unsafe extern "C" fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    if data.slip < 1.0 {
        data.slip = -1.0;
    }
    call_original!(module, id, group, data);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32); //Removes phantoms
	skyline::install_hooks!(
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
        attack_module_set_attack
    );
}