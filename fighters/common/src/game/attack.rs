use super::*;

//Hit Module Handle Attack Event, determines where you hit and with what hitbox id
#[skyline::hook(offset = 0x46ae84, inline)]
unsafe extern "C" fn hit_module_handle_attack_event(ctx: &InlineCtx) {
    let data = ctx.registers[1].x() as *mut u32;
    let attacker_id = *data;
    let collision_id = *data.add(1);
    let battle_object = &mut *get_battle_object_from_id(attacker_id);
    let category = smash::app::utility::get_category(&mut (*battle_object.module_accessor));
    if ![*BATTLE_OBJECT_CATEGORY_FIGHTER, *BATTLE_OBJECT_CATEGORY_WEAPON, *BATTLE_OBJECT_CATEGORY_ITEM].contains(&category) {
        return;
    }
    let collision_data = ctx.registers[27].x() as *mut f32;
    let loc_x = *collision_data.add(4);
    let loc_y = *collision_data.add(5);
    let loc_z = *collision_data.add(6);
    LAST_ATTACK_HITBOX_ID = collision_id as i32;
    LAST_ATTACK_HITBOX_LOCATION_X = loc_x;
    LAST_ATTACK_HITBOX_LOCATION_Y = loc_y;
    LAST_ATTACK_HITBOX_LOCATION_Z = loc_z;
}

//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield
#[skyline::hook(offset = 0x4c7080)]
unsafe extern "C" fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    let attacker_id = *(collision.add(0x24) as *const u32);
	let attacker_battle_object = &mut *get_battle_object_from_id(attacker_id);
    let attacker_category = smash::app::utility::get_category(&mut (*attacker_battle_object.module_accessor));
    if ![*BATTLE_OBJECT_CATEGORY_FIGHTER, *BATTLE_OBJECT_CATEGORY_WEAPON, *BATTLE_OBJECT_CATEGORY_ITEM].contains(&attacker_category) {
        return;
    }
    let hitbox_id = *(collision.add(0x33) as *const u8);
    let loc_x = *(collision.add(0x10) as *const f32);
    let loc_y = *(collision.add(0x14) as *const f32);
    let loc_z = *(collision.add(0x18) as *const f32);
    LAST_ATTACK_HITBOX_ID = hitbox_id as i32;
    LAST_ATTACK_HITBOX_LOCATION_X = loc_x;
    LAST_ATTACK_HITBOX_LOCATION_Y = loc_y;
    LAST_ATTACK_HITBOX_LOCATION_Z = loc_z;
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
}

//Attack Module Set Attack, makes it so random tripping doesn't happen if the move doesn't have a 100% trip chance (Credit to HDR)
#[skyline::hook(offset = 0x3dc180)]
unsafe extern "C" fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    if data.slip < 1.0 {
        data.slip = -1.0;
    }
    call_original!(module, id, group, data);
}

//Notify Log Event Collision Hit, dictates several things when you've hit the opponent
#[skyline::hook(offset=0x67a7b0)]
unsafe extern "C" fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *smash::app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_status_kind = StatusModule::status_kind(attacker_boma);
    if attacker_status_kind == WEAPON_PURIN_DISARMING_VOICE_STATUS_KIND_SHOOT {
        ItemModule::drop_item(defender_boma, 0.0, 0.0, 0);
    }
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32); //Removes phantoms
	skyline::install_hooks!(
        notify_log_event_collision_hit,
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
        attack_module_set_attack
    );
}