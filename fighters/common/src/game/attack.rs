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
    let attacker_category = utility::get_category(&mut *attacker_boma);
    let defender_category = utility::get_category(&mut *defender_boma);
    let attacker_kind = sv_battle_object::kind(attacker_object_id);
    let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if attacker_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if defender_category == *BATTLE_OBJECT_CATEGORY_ITEM {
            if attacker_kind == *WEAPON_KIND_LINK_BOOMERANG {
                WorkModule::set_int(attacker_boma, defender_object_id as i32, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
                LinkModule::remove_model_constraint(defender_boma, true);
                if LinkModule::is_link(defender_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(defender_boma, *ITEM_LINK_NO_HAVE);
                }
                if !LinkModule::is_link(defender_boma, *ITEM_LINK_NO_HAVE) {
                    VisibilityModule::set_whole(defender_boma, true);
                    LinkModule::link(defender_boma, *ITEM_LINK_NO_HAVE, (*(attacker_boma)).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(defender_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
                }
            }
        }
        if defender_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            if defender_object_id == owner_id {
                if owner_kind == *FIGHTER_KIND_LINK {
                    let fuse_item_id = WorkModule::get_int(attacker_boma, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                    let item_boma = smash::app::sv_battle_object::module_accessor(fuse_item_id);
                    if fuse_item_id != *BATTLE_OBJECT_ID_INVALID as u32 && sv_battle_object::is_active(fuse_item_id) {
                        LinkModule::remove_model_constraint(item_boma, true);
                        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
                    }
                }
            }
            if attacker_kind == *WEAPON_KIND_KOOPAJR_CANNONBALL {
                if owner_kind == *FIGHTER_KIND_GEKKOUGA {
                    let owner_frame = MotionModule::frame(owner_boma);
                    let owner_status_kind = StatusModule::status_kind(owner_boma);
                    if owner_frame >= 40.0 {
                        if owner_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                            CancelModule::enable_cancel(owner_boma);
                        }
                    }
                }
            }
        }
    }
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32); //Removes phantoms
	skyline::install_hooks!(
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
        attack_module_set_attack,
        notify_log_event_collision_hit
    );
}