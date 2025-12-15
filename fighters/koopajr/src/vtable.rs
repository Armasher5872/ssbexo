use super::*;

const KOOPAJR_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xbe37b0; //Bowser Jr only
const KOOPAJR_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xbe37e0; //Bowser Jr only
const KOOPAJR_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xbe3830; //Bowser Jr only
const KOOPAJR_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xbe3d20; //Bowser Jr only
const KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET: usize = 0x3425870;
const KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET: usize = 0x34257f0;

unsafe extern "C" fn koopajr_check_air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_aerial = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) 
    || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT) {
            let mut allow_float = false;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
                    allow_float = !is_aerial;
                }
            }
            if allow_float {
                fighter.change_status(FIGHTER_KOOPAJR_STATUS_KIND_FLOAT.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn koopajr_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn koopajr_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
    WorkModule::set_int(boma, 0, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
}

//Bowser Jr Startup Initialization
#[skyline::hook(offset = KOOPAJR_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopajr_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    koopajr_var(&mut *boma);
    agent.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&L2CValue::Ptr(koopajr_check_air_jump_aerial_uniq as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(koopajr_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Bowser Jr Reset Initialization
#[skyline::hook(offset = KOOPAJR_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopajr_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    koopajr_var(&mut *boma);
    original!()(vtable, fighter)
}

//Bowser Jr Death Initialization
#[skyline::hook(offset = KOOPAJR_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopajr_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    koopajr_var(&mut *boma);
    original!()(vtable, fighter)
}

//Bowser Jr Once Per Fighter Frame
#[skyline::hook(offset = KOOPAJR_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn koopajr_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) && WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, false);
        }
    }
    original!()(vtable, fighter)
}

//Bowser Jr Cannonball Initialization Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialization_event(vtable: u64, weapon: *mut smash::app::Weapon, param_3: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = *(param_3 as *mut u32).add(0x2c/4);
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let ptr = get_module_vtable_func(boma, 0x108, 0x60);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
    let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
    if owner_kind == *FIGHTER_KIND_DONKEY {
        let shield_data = ShieldData::new(0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 8.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_JUST_SHIELD_REFLECTOR as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
        set_shield_group2(reflector_module, resource, *WEAPON_DONKEY_BARREL_SHIELD_KIND_BODY);
        ReflectorModule::set_status(boma, *WEAPON_DONKEY_BARREL_SHIELD_KIND_BODY, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    if owner_kind == *FIGHTER_KIND_GANON {
        let shield_data = ShieldData::new(6.0, 0.0, 0.0, 6.0, 0.0, 0.0, 8.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_JUST_SHIELD_REFLECTOR as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
        set_shield_group2(reflector_module, resource, *WEAPON_GANON_VOLLEY_SHIELD_KIND_BODY);
        ReflectorModule::set_status(boma, *WEAPON_GANON_VOLLEY_SHIELD_KIND_BODY, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    if owner_kind == *FIGHTER_KIND_GEKKOUGA {
        let shield_data = ShieldData::new(0.0, 0.0, 0.0, 30.0, 0.0, 0.0, 3.0, Hash40::new("tatami1"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_JUST_SHIELD_REFLECTOR as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
        set_shield_group2(reflector_module, resource, *WEAPON_GEKKOUGA_MAT_SHIELD_KIND_BODY);
        ReflectorModule::set_status(boma, *WEAPON_GEKKOUGA_MAT_SHIELD_KIND_BODY, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    call_original!(vtable, weapon, param_3)
}

//Bowser Jr Cannonball Reflector Clean Event Offset
unsafe extern "C" fn koopajr_cannonball_reflector_clean_event(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GANON, *FIGHTER_KIND_GEKKOUGA].contains(&owner_kind) {
        ReflectorModule::clean(boma);
    }
}

//Bowser Jr Cannonball On Attack Offset
unsafe extern "C" fn koopajr_cannonball_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let pos = *PostureModule::pos(boma);
    if owner_kind == *FIGHTER_KIND_DONKEY {
        *(weapon as *mut bool).add(0x90) = true;
    }
    if owner_kind == *FIGHTER_KIND_GANON {
        if !WorkModule::is_flag(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLAG_CHARGED) {
            WorkModule::off_flag(owner_boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_HAS_ACTIVE_VOLLEY);
            EffectModule::kill_kind(boma, Hash40::new("ganon_volley"), true, true);
            EffectModule::req(boma, Hash40::new("ganon_appeal_aura"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
            *(weapon as *mut bool).add(0x90) = false;
        }
        else {
            WorkModule::inc_int(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_TOTAL_HIT_COUNT);
            AttackModule::clear_all(boma);
            let total_hit_count = WorkModule::get_int(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_TOTAL_HIT_COUNT);
            if total_hit_count == 1 {
                WorkModule::set_int(boma, LAST_ATTACK_HITBOX_ID, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FIRST_HIT_ID);
            }
            if total_hit_count == 2 {
                WorkModule::set_int(boma, LAST_ATTACK_HITBOX_ID, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_SECOND_HIT_ID);
            }
            if total_hit_count == 3 {
                WorkModule::set_int(boma, LAST_ATTACK_HITBOX_ID, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_THIRD_HIT_ID);   
            }
            if total_hit_count == 4 {
                WorkModule::set_int(boma, LAST_ATTACK_HITBOX_ID, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FOURTH_HIT_ID);   
            }
            volley_hitbox_check(
                weapon, 
                agent,
                total_hit_count,
                WorkModule::get_int(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FIRST_HIT_ID), 
                WorkModule::get_int(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_SECOND_HIT_ID), 
                WorkModule::get_int(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_THIRD_HIT_ID), 
                WorkModule::get_int(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_INT_FOURTH_HIT_ID)
            );
        }
    }
    if owner_kind == *FIGHTER_KIND_IKE {
        EffectModule::req(boma, Hash40::new("ike_counter_attack"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        *(weapon as *mut bool).add(0x90) = false;
    }
    if owner_kind == *FIGHTER_KIND_PFUSHIGISOU {
        EffectModule::req(boma, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
        *(weapon as *mut bool).add(0x90) = false;
    }
    if owner_kind == *FIGHTER_KIND_GEKKOUGA {
        *(weapon as *mut bool).add(0x90) = true;
    }
    if owner_kind == *FIGHTER_KIND_KOOPAJR {
        agent.clear_lua_stack();
        lua_args!(agent, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_bomb_b"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        sv_module_access::effect(agent.lua_state_agent);
        *(weapon as *mut bool).add(0x90) = false;
    }
    normal_weapon_hit_handler(vtable, weapon, log)
}

//Bowser Jr Cannonball On Reflect Event Offset
unsafe extern "C" fn koopajr_cannonball_on_reflect_event(_vtable: u64, battle_object: *mut BattleObject) {
    let boma = (*battle_object).module_accessor;
    let agent = get_weapon_common_from_accessor(&mut *boma);
    let reflect_team_no = ReflectModule::team_no(boma);
    let reflect_object_id = ReflectModule::object_id(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    PostureModule::reverse_lr(boma);
    PostureModule::update_rot_y_lr(boma);
    if owner_kind != *FIGHTER_KIND_GANON {
        TeamModule::set_team(boma, reflect_team_no as i32, true);
        TeamModule::set_hit_team(boma, reflect_team_no as i32);
        TeamModule::set_team_owner_id(boma, reflect_object_id as u32);
    }
    else {
        let scale = WorkModule::get_float(boma, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_SCALE);
        TeamModule::set_team(boma, *TEAM_NONE, false);
        TeamModule::set_hit_team(boma, *TEAM_NONE);
        TeamModule::set_team_owner_id(boma, *BATTLE_OBJECT_ID_INVALID as u32);
        EffectModule::kill_kind(boma, Hash40::new("ganon_volley"), true, true);
        if scale > 1.0 {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_volley"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 3.8+scale, true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_volley"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 3.8, true);
        }
    }
}

//Bowser Jr Cannonball On Reflection Event Offset
unsafe extern "C" fn koopajr_cannonball_on_reflection_event(_vtable: u64, weapon: *mut smash::app::Weapon, log: *mut ShieldAttackCollisionEvent) {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let agent = get_weapon_common_from_accessor(&mut *boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let opponent_id = (*(*log).collision_log).opponent_object_id;
    let opponent_battle_object = get_battle_object_from_id(opponent_id);
    let opponent_boma = (*opponent_battle_object).module_accessor;
    let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
    let opponent_status_kind = opponent_agent.global_table[STATUS_KIND].get_i32();
    let opponent_category = utility::get_category(&mut *opponent_boma);
    let attack_data = *AttackModule::attack_data(owner_boma, (*(*log).collision_log).collider_id as i32, (*(*log).collision_log).x35);
    let power = attack_data.power;
    let attr = attack_data.attr;
    EffectModule::kill_kind(boma, Hash40::new("sys_reflection"), true, true);
    spawn_hit_effects(agent, attr);
    if owner_kind == *FIGHTER_KIND_DONKEY {
        WorkModule::sub_float(boma, power, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
        if WorkModule::get_float(boma, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP) <= 0.0 && status_kind != *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK {
            StatusModule::change_status_request_from_script(boma, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, false);
        }
    }
    if owner_kind == *FIGHTER_KIND_GANON {
        let life = WorkModule::get_param_int(boma, hash40("param_volley"), hash40("life"));
        let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let normals_status = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&opponent_status_kind);
        let smashes_status = [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&opponent_status_kind);
        let specials_status = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_FINAL].contains(&opponent_status_kind);
        let speed_multiplier: f32 = if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            if normals_status {
                1.75
            }
            else if smashes_status {
                3.0
            }
            else if specials_status {
                2.0
            }
            else {
                1.1
            }
        }
        else {
            -1.0
        };
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
        ReflectorModule::set_no_team(boma, true);
        TeamModule::set_team(boma, *TEAM_NONE, false);
        TeamModule::set_hit_team(boma, *TEAM_NONE);
        TeamModule::set_team_owner_id(boma, *BATTLE_OBJECT_ID_INVALID as u32);
        WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        let reflect = SoundModule::play_se(boma, Hash40::new("se_ganon_special_n06"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, reflect as i32, 6.0, 0);
        sv_kinetic_energy!(set_speed, agent, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -get_sum_speed_x*speed_multiplier, 0.0);
        sv_kinetic_energy!(set_stable_speed, agent, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -get_sum_speed_x*speed_multiplier, 0.0);
    }
}

//Bowser Jr Cannonball Initialize Weapon Module Accessor Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialize_weapon_module_accessor(vtable: u64, boma: *mut BattleObjectModuleAccessor, param_3: u64) -> u64 {
    *(param_3 as *mut i32).add(0x288/4) = *COLLISION_KIND_SHIELD;
    call_original!(vtable, boma, param_3)
}

pub fn install() {
    //Fuck it we ball type code (Patches the initialization of Bowser Jr's Cannonball modules to instead use Palutena's Reflection Board Module Initialization so that the former can call to ReflectorModule functions correctly)
    let initialize_reflectormodule = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+0x33b9830};
    let _ = skyline::patching::Patch::in_text(0x519ab68).data(initialize_reflectormodule);
    let _ = skyline::patching::Patch::in_text(0x51d8348).data(koopajr_cannonball_reflector_clean_event as u64);
    let _ = skyline::patching::Patch::in_text(0x51d83e8).data(koopajr_cannonball_on_attack as u64);
    let _ = skyline::patching::Patch::in_text(0x51d8400).data(koopajr_cannonball_on_reflect_event as u64);
    let _ = skyline::patching::Patch::in_text(0x51d8468).data(koopajr_cannonball_on_reflection_event as u64);
	skyline::install_hooks!(
        koopajr_start_initialization,
        koopajr_reset_initialization,
        koopajr_death_initialization,
        koopajr_opff,
        koopajr_cannonball_initialization_event,
        koopajr_cannonball_initialize_weapon_module_accessor
    );
}