use super::*;

const MIIFIGHTER_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xd566e0; //Mii Brawler only
const MIIFIGHTER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xd56780; //Mii Brawler only
const MIIFIGHTER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd56e70; //Mii Brawler only
const MIIFIGHTER_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xd592c0; //Mii Brawler only
const MIIFIGHTER_VTABLE_ON_SEARCH_OFFSET: usize = 0xd59650; //Mii Brawler only
const MIIFIGHTER_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Shared

//Set Move Customizer is accredited to WuBor Patch
unsafe extern "C" fn miifighter_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let waza_customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_n3_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_RISE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_rise_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_RISE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_rise_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_dive_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_n3_dive_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_dive_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_LAND.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_land_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_LAND.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_land_main_status as *const ()));
        0.into()
    }
    else if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_s1_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_s1_end_main_status as *const ()));
        0.into()
    }
    else if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_lw1_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_lw1_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_lw1_charge_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_charge_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_lw1_charge_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_lw1_attack_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_attack_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_lw1_attack_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miifighter_special_lw1_attack_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_attack_exit_status as *const ()));
        0.into()
    }
    else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } 
    else {
        0.into()
    }
}

unsafe extern "C" fn miifighter_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn miifighter_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
    WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
    WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    WorkModule::set_float(boma, 10.0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_ARMOR);
    WorkModule::set_float(boma, 12.0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    WorkModule::set_int(boma, 0, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
}

//Mii Brawler Startup Initialization
#[skyline::hook(offset = MIIFIGHTER_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miifighter_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    miifighter_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(miifighter_end_control as *const () as _));
    set_move_customizer(agent, miifighter_waza_customize);
    miifighter_waza_customize(agent);
    original!()(vtable, fighter)
}

//Mii Brawler Reset Initialization
#[skyline::hook(offset = MIIFIGHTER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miifighter_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    miifighter_var(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Brawler Death Initialization
#[skyline::hook(offset = MIIFIGHTER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miifighter_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    miifighter_var(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Brawler Once Per Fighter Frame
#[skyline::hook(offset = MIIFIGHTER_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn miifighter_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_miifighter_final"), -1);
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(agent);
        }
        if counter == 10 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
    original!()(vtable, fighter)
}

//Mii Brawler On Search
#[skyline::hook(offset = MIIFIGHTER_VTABLE_ON_SEARCH_OFFSET)]
unsafe extern "C" fn miifighter_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let status_kind = StatusModule::status_kind(boma);
    let customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let opponent_id = (*collision_log).opponent_battle_object_id;
            WorkModule::set_int(boma, opponent_id as i32, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
            WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
            if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let counter_throw_boma = sv_battle_object::module_accessor(opponent_id as u32);
                    LinkModule::remove_model_constraint(counter_throw_boma, true);
                    if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                        LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                    }
                    if !LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                        VisibilityModule::set_whole(counter_throw_boma, true);
                        LinkModule::link(counter_throw_boma, *LINK_NO_ARTICLE, (*boma).battle_object_id);
                        LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *LINK_NO_ARTICLE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                        LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                    }
                    GroundModule::set_ignore_boss(counter_throw_boma, true);
                    GroundModule::set_passable_check(counter_throw_boma, false);
                    GroundModule::set_collidable(counter_throw_boma, false);
                    JostleModule::set_status(counter_throw_boma, false);
                    WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, false);
                }
                if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_ITEM {
                    let counter_throw_boma = sv_battle_object::module_accessor(opponent_id as u32);
                    LinkModule::remove_model_constraint(counter_throw_boma, true);
                    if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                        LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                    }
                    if !LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                        VisibilityModule::set_whole(counter_throw_boma, true);
                        LinkModule::link(counter_throw_boma, *ITEM_LINK_NO_HAVE, (*boma).battle_object_id);
                        LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                        LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                    }
                    GroundModule::set_ignore_boss(counter_throw_boma, true);
                    GroundModule::set_passable_check(counter_throw_boma, false);
                    GroundModule::set_collidable(counter_throw_boma, false);
                    JostleModule::set_status(counter_throw_boma, false);
                    WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, false);
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Mii Brawler On Damage
#[skyline::hook(offset = MIIFIGHTER_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn miifighter_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MIIFIGHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = fighter.battle_object.status();
        let customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
        if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
            if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE].contains(&status_kind) {
                DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
                WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
            }
        }
    }
    original!()(vtable, fighter, on_damage)
}

pub fn install() {
	skyline::install_hooks!(
        miifighter_start_initialization,
        miifighter_reset_initialization,
        miifighter_death_initialization,
        miifighter_opff,
        miifighter_on_search,
        miifighter_on_damage
    );
}