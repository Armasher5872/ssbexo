use super::*;

const KOOPAJR_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xbe37b0; //Bowser Jr only
const KOOPAJR_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xbe37e0; //Bowser Jr only
const KOOPAJR_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xbe3830; //Bowser Jr only
const KOOPAJR_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xbe3d20; //Bowser Jr only
const KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET: usize = 0x3425ae0;
const KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET: usize = 0x3425a60;

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
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
    let ptr = get_module_vtable_func(boma, 0x108, 0x60);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
    let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
    if owner_kind == *FIGHTER_KIND_DONKEY {
        let shield_data = ShieldData::new(0.0, 6.0, 0.0, 0.0, 3.0, 0.0, 8.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_JUST_SHIELD_REFLECTOR as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
        set_shield_group2(reflector_module, resource, *WEAPON_DONKEY_BARREL_SHIELD_KIND_BODY);
        ReflectorModule::set_status_all(boma, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    if owner_kind == *FIGHTER_KIND_IKE {
        let shield_data = ShieldData::new(0.0, 11000.0, 0.0, 0.0, -7000.0, 0.0, 4000.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_JUST_SHIELD_REFLECTOR as u8);
        let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
        let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.5, 1.5, 50.0, 1.0, false, 0);
        set_shield_group2(reflector_module, resource, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR);
        ReflectorModule::set_status(boma, *WEAPON_IKE_SLASH_REFLECTOR_KIND_REFLECTOR, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    call_original!(vtable, weapon, param_3)
}

//Bowser Jr Cannonball Reflector Clean Event Offset
unsafe extern "C" fn koopajr_cannonball_reflector_clean_event(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_IKE].contains(&owner_kind) {
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
        WorkModule::on_flag(boma, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_DID_ATTACK);
    }
    if owner_kind == *FIGHTER_KIND_PURIN {
        EffectModule::req(boma, Hash40::new("sys_flash"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("poke_meloetta_bullet"), false, false);
        EffectModule::kill_kind(boma, Hash40::new("rosetta_ring_erase"), false, false);
        *(weapon as *mut bool).add(0x90) = false;
    }
    if [*FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_IKE].contains(&owner_kind) {
        EffectModule::req(boma, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
        *(weapon as *mut bool).add(0x90) = false;
    }
    if owner_kind == *FIGHTER_KIND_PFUSHIGISOU {
        EffectModule::req(boma, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        EffectModule::kill_kind(boma, Hash40::new("packun_poison_gas"), false, false);
        *(weapon as *mut bool).add(0x90) = false;
    }
    if owner_kind == *FIGHTER_KIND_KOOPAJR {
        agent.clear_lua_stack();
        lua_args!(agent, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_bomb_b"), hash40("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
        sv_module_access::effect(agent.lua_state_agent);
        *(weapon as *mut bool).add(0x90) = false;
    }
    normal_weapon_hit_handler(vtable, weapon, log)
}

//Bowser Jr Cannonball On Reflection Event Offset
unsafe extern "C" fn koopajr_cannonball_on_reflection_event(_vtable: u64, weapon: *mut smash::app::Weapon, log: *mut ShieldAttackCollisionEvent) {
    let boma = (*weapon).battle_object.module_accessor;
    let agent = get_weapon_common_from_accessor(&mut *boma);
    let lr = PostureModule::lr(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let opponent_id = (*(*log).collision_log).opponent_object_id;
    let opponent_category = (*(*log).collision_log).opponent_object_category as i32;
    let opponent_power = (*log).real_power;
    let opponent_attack_data = AttackModule::attack_data(owner_boma, (*(*log).collision_log).collider_id as i32, (*(*log).collision_log).x35);
    let opponent_angle = (*opponent_attack_data).vector;
    let opponent_battle_object = get_battle_object_from_id(opponent_id);
    let opponent_boma = (*opponent_battle_object).module_accessor;
    let opponent_agent = get_weapon_common_from_accessor(&mut *opponent_boma);
    let opponent_speed = KineticModule::get_sum_speed_x(opponent_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if owner_kind == *FIGHTER_KIND_DONKEY {
        let mut angle = opponent_angle;
        if opponent_angle > 360 {
            angle = 32;
        }
        let hit_radians = (angle as f32).to_radians();
        let hit_sin = hit_radians.sin();
        let hit_cos = hit_radians.cos();
        let speed_max = WorkModule::get_param_float(boma, hash40("param_barrel"), hash40("speed_max"));
        let speed_x = hit_sin*speed_max;
        let speed_y = hit_cos*speed_max;
        let opponent_kind = utility::get_kind(&mut *opponent_boma);
        if opponent_kind == owner_kind {
            WorkModule::sub_float(boma, (opponent_power/10.0).clamp(0.0, 10.0), *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
        }
        else {
            WorkModule::sub_float(boma, opponent_power.clamp(0.0, 10.0), *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
        }
        StopModule::set_hit_stop_frame(opponent_boma, 20, false);
        StopModule::set_hit_stop_frame(boma, 20, false);
        sv_kinetic_energy!(set_speed, agent, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x*lr, speed_y);
        StatusModule::change_status_request_from_script(boma, *WEAPON_DONKEY_BARREL_STATUS_KIND_ROLL, false);
    }
    if owner_kind == *FIGHTER_KIND_IKE {
        if [*BATTLE_OBJECT_CATEGORY_WEAPON, *BATTLE_OBJECT_CATEGORY_ITEM].contains(&opponent_category) {
            if opponent_category == *BATTLE_OBJECT_CATEGORY_ITEM {
                let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
                if smash::app::sv_battle_object::is_active(opponent_id) {
                    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, opponent_id);
                }
            }
            notify_event_msc_cmd!(opponent_agent, Hash40::new_raw(0x199c462b5d));
            WorkModule::set_float(boma, opponent_speed, *WEAPON_IKE_SLASH_INSTANCE_WORK_ID_FLOAT_ABSORB_SPEED);
            WorkModule::set_float(boma, opponent_power, *WEAPON_IKE_SLASH_INSTANCE_WORK_ID_FLOAT_ABSORB_POWER);
            WorkModule::on_flag(boma, *WEAPON_IKE_SLASH_INSTANCE_WORK_ID_FLAG_ABSORBED_WEAPON);
        }
    }
}

//Bowser Jr Cannonball Initialize Weapon Module Accessor Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialize_weapon_module_accessor(vtable: u64, boma: *mut smash::app::BattleObjectModuleAccessor, param_3: u64) -> u64 {
    *(param_3 as *mut i32).add(0x288/4) = *COLLISION_KIND_SHIELD;
    call_original!(vtable, boma, param_3)
}

pub fn install() {
    //Fuck it ball type code (Patches the initialization of Bowser Jr's Cannonball modules to instead use Palutena's Reflection Board Module Initialization so that the former can call to ReflectorModule functions correctly)
    let initialize_reflectormodule = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+0x33b9aa0};
    let _ = skyline::patching::Patch::in_text(0x519bb68).data(initialize_reflectormodule);
    let _ = skyline::patching::Patch::in_text(0x51d9348).data(koopajr_cannonball_reflector_clean_event as u64);
    let _ = skyline::patching::Patch::in_text(0x51d93e8).data(koopajr_cannonball_on_attack as u64);
    let _ = skyline::patching::Patch::in_text(0x51d9468).data(koopajr_cannonball_on_reflection_event as u64);
	skyline::install_hooks!(
        koopajr_start_initialization,
        koopajr_reset_initialization,
        koopajr_death_initialization,
        koopajr_opff,
        koopajr_cannonball_initialization_event,
        koopajr_cannonball_initialize_weapon_module_accessor
    );
}