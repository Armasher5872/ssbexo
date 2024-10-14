use super::*;

//Hit Module Handle Attack Event, determines where you hit and with what hitbox id
#[skyline::hook(offset = 0x46ae84, inline)]
unsafe fn hit_module_handle_attack_event(ctx: &InlineCtx) {
    let data = *ctx.registers[1].x.as_ref() as *mut u32;
    let attacker_id = *data;
    let collision_id = *data.add(1);
    let battle_object = &mut *crate::functions::hook::misc::get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }
    let collision_data = *ctx.registers[27].x.as_ref() as *mut f32;
    let loc_x = *collision_data.add(4);
    let loc_y = *collision_data.add(5);
    let loc_z = *collision_data.add(6);
    LAST_ATTACK_HITBOX_ID = collision_id as i32;
    LAST_ATTACK_HITBOX_LOCATION_X = loc_x;
    LAST_ATTACK_HITBOX_LOCATION_Y = loc_y;
    LAST_ATTACK_HITBOX_LOCATION_Z = loc_z;
}

//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield. Also dictates hard shield breaks
#[skyline::hook(offset = 0x4c7080)]
unsafe fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    let defender_boma = *(shield_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let defender_status_kind = StatusModule::status_kind(defender_boma);
    let attacker_id = *(collision.add(0x24) as *const u32);
	let attacker_battle_object = &mut *crate::functions::hook::misc::get_battle_object_from_id(attacker_id);
    let attacker_boma = attacker_battle_object.module_accessor;
    let attacker_shield_damage = WorkModule::get_int(attacker_boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE) as f32;
    let defender_shield_hp = WorkModule::get_float(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let motion_rate: f32;
    let damage: f32;
    if !attacker_battle_object.is_fighter() && !attacker_battle_object.is_weapon() {
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
    damage = ((30.0/(defender_shield_hp*1.4))+(defender_shield_hp*1.65)).clamp(15.0, 30.0);
    if (real_power+attacker_shield_damage) >= damage && [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD].contains(&defender_status_kind) {
        StatusModule::change_status_request_from_script(defender_boma, *FIGHTER_STATUS_KIND_FURAFURA, false);
    }
    if real_power == 0.0 {
        motion_rate = 1.0;
    } 
    else {
        motion_rate = (1.0-(0.02*real_power)).clamp(0.5, 1.0);
    };
    if defender_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF
    && FighterUtil::is_valid_just_shield(defender_boma) {
        WorkModule::set_float(attacker_boma, motion_rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
        if attacker_battle_object.is_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND_JUMP, false);
        }
        else {
            StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
        }
    }
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
}

//Attack Module Set Attack, makes it so random tripping doesn't happen if the move doesn't have a 100% trip chance (Credit to HDR)
#[skyline::hook(offset = 0x3dc180)]
unsafe fn attack_module_set_attack(module: u64, id: i32, group: i32, data: &mut smash2::app::AttackData) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);
    if data.slip < 1.0 {
        data.slip = -1.0;
    }
    if (data.sub_shield as f32) < 0.0 || (data.sub_shield as f32) > 50.0 {
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    }
    else {
        WorkModule::set_int(boma, data.sub_shield as i32, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    }
    WorkModule::set_int(boma, data.vector, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
    call_original!(module, id, group, data);
}

//Used for reverse knockback
#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    if SPECIAL_SMASH_STATUS == 1 {
        let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for (i, x) in hitbox_params.iter_mut().enumerate().take(36) {
            if i == 4 {
                if i < 362 {
                    let positive_angle = (i+180) as f32;
                    let negative_angle = (i-180) as f32;
                    if i < 180 {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(positive_angle));
                    }
                    else {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(negative_angle));
                    }
                }
            }
            else {
                l2c_agent.push_lua_stack(x);
            }
        }
    }
	original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::ATTACK_ABS)]
unsafe fn attack_abs_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    if SPECIAL_SMASH_STATUS == 1 {
        let mut hitbox_params: Vec<L2CValue> = (0..15).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for (i, x) in hitbox_params.iter_mut().enumerate().take(15) {
            if i == 3 {
                if i < 362 {
                    let positive_angle = (i+180) as f32;
                    let negative_angle = (i-180) as f32;
                    if i < 180 {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(positive_angle));
                    }
                    else {
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(negative_angle));
                    }
                }
            }
            else {
                l2c_agent.push_lua_stack(x);
            }
        }
    }
    original!()(lua_state);
}

//Notify Log Event Collision Hit, dictates several things when you've hit the opponent
#[skyline::hook(offset=0x67a7b0)]
unsafe fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
	let attacker_boma = &mut *smash::app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *smash::app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_kind = smash::app::utility::get_kind(attacker_boma);
	let defender_kind = smash::app::utility::get_kind(defender_boma);
	let attacker_status_kind = StatusModule::status_kind(attacker_boma);
	//Turbo
	if SPECIAL_SMASH_HEAD == 1 {
		CancelModule::enable_cancel(attacker_boma);
	}
	//Ball
	if attacker_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(defender_boma); //If the ball hits someone and then goes out of bounds, the team that got hit loses the stock
	}
	if defender_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(attacker_boma);
        WorkModule::set_flag(attacker_boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
	}
	//GGST COUNTER!/Little Mac Star Punch Counterhit Detection
	if COUNTERHIT_CHECK[get_player_number(defender_boma)]
	&& attacker_boma.is_fighter() {
		if attacker_kind != *FIGHTER_KIND_LITTLEMAC 
		&& [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&attacker_status_kind) {
			//Counterhit Detection
			COUNTERHIT_SUCCESS[get_player_number(attacker_boma)] = true;
			COUNTERHIT_CHECK[get_player_number(defender_boma)] = false;
		}
	}
	LAST_DAMAGE[get_player_number(defender_boma)] = DamageModule::damage(defender_boma, 0);
    if attacker_status_kind == WEAPON_PURIN_DISARMING_VOICE_STATUS_KIND_SHOOT {
        ItemModule::drop_item(defender_boma, 0.0, 0.0, 0);
    }
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

//The following hooks are used to ignore setting the team of several items, which are used to resolve issues regarding items being able to hit the user, or them not being hittable themselves.

#[skyline::hook(replace=TeamModule::set_hit_team)]
unsafe fn set_hit_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    let kind = boma.kind();
    if [*ITEM_KIND_BARREL, *ITEM_KIND_SNAKEGRENADE].contains(&kind) {
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_hit_team_second)]
unsafe fn set_hit_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    let kind = boma.kind();
    if (boma.is_item() && [*ITEM_KIND_BARREL, *ITEM_KIND_SNAKEGRENADE].contains(&kind)) {
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_team)]
unsafe fn set_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32, arg3: bool) {
    let kind = boma.kind();
    if (boma.is_item() && [*ITEM_KIND_BARREL, *ITEM_KIND_SNAKEGRENADE].contains(&kind)) {} 
    else {
        original!()(boma, arg2, arg3);
    }
}

#[skyline::hook(replace=TeamModule::set_team_second)]
unsafe fn set_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    let kind = boma.kind();
    if (boma.is_item() && [*ITEM_KIND_BARREL, *ITEM_KIND_SNAKEGRENADE].contains(&kind)) {
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_team_owner_id)]
unsafe fn set_team_owner_id_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    let kind = boma.kind();
    if (boma.is_item() && [*ITEM_KIND_BARREL, *ITEM_KIND_SNAKEGRENADE].contains(&kind)) {
        return;
    }
}

//Calls the Special Zoom function
#[skyline::from_offset(0x696720)]
pub fn call_special_zoom(boma: *mut BattleObjectModuleAccessor, collision_log: u64, fighter_kind: i32, vl_params: u64, param_5: i32, param_6: i32, param_7: i32, param_8: i32, param_9: i32) -> u64;

//Related to the updated finishing zoom function
#[skyline::hook(offset = 0x402f00, inline)]
unsafe fn calculate_knockback(ctx: &InlineCtx) {
    let damage_module = *ctx.registers[19].x.as_ref();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = *ctx.registers[20].x.as_ref() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe fn process_knockback(ctx: &InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = *ctx.registers[20].x.as_ref() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            calculate_finishing_hit(defender, attacker, *ctx.registers[19].x.as_ref() as *const f32);
        }
    }
}

pub fn install() {
	unsafe {
        //Removes the vanilla kill zoom in favor of the updated function
        const KILL_ZOOM_DATA: u32 = 0xD503201Fu32;
        skyline::patching::Patch::in_text(0x633de0).nop();
        skyline::patching::Patch::in_text(0x6373a4).data(KILL_ZOOM_DATA);
        //Removes phantoms
        skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32);
    }
    skyline::install_hook!(attack_replace);
	skyline::install_hook!(attack_abs_replace);
	skyline::install_hook!(notify_log_event_collision_hit);
	skyline::install_hooks!(
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
        attack_module_set_attack,
        set_hit_team_hook,
        set_hit_team_second_hook,
        set_team_second_hook,
        set_team_hook,
        set_team_owner_id_hook,
        process_knockback,
        calculate_knockback
    );
}