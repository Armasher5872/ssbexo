#![allow(dead_code)]
use super::*;

const NESS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xdefdf0; //Ness only
const NESS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const NESS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xdf0050; //Ness only
const NESS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xdf0360; //Ness only
const NESS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET: usize = 0xdf0400; //Ness only

unsafe extern "C" fn ness_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
    WorkModule::on_flag(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
    WorkModule::set_int(boma, 0, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
}

//Ness Startup Initialization
#[skyline::hook(offset = NESS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let attack_air_f_data = ShieldDataResource::new(0.0, 6.8, 10.0, 0.0, 6.8, 10.0, 6.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let attack_air_f_2_data = ShieldDataResource::new(0.0, 4.8, 9.6, 0.0, 4.8, 9.6, 10.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let attack_air_f_datas = &mut (ShieldDatas::new().add(attack_air_f_data, 0));
    let attack_air_f_2_datas = &mut (ShieldDatas::new().add(attack_air_f_2_data, 1));
    let resource_attack_air_f = &mut ShieldGroupResource::new(attack_air_f_datas, 1, 0, false, false, false);
    let resource_attack_air_f_2 = &mut ShieldGroupResource::new(attack_air_f_2_datas, 1, 0, false, false, false);
    add_shield_group(boma, resource_attack_air_f, *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_GUARD);
    add_shield_group(boma, resource_attack_air_f_2, *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_2_GUARD);
    common_initialization_variable_reset(&mut *boma);
    ness_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Ness Reset Initialization
#[skyline::hook(offset = NESS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_NESS as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        ness_var(&mut *boma);
        ShieldModule::set_target_property(boma, *COLLISION_PROPERTY_NORMAL, *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_GUARD);
        ShieldModule::set_target_property(boma, *COLLISION_PROPERTY_NORMAL, *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_2_GUARD);
    }
    original!()(vtable, fighter)
}

//Ness Death Initialization
#[skyline::hook(offset = NESS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_death_initialization(param_1: u64, param_2: u64, param_3: u64, param_4: u64, vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    ness_var(&mut *boma);
    original!()(param_1, param_2, param_3, param_4, vtable, fighter)
}

//Ness Once Per Fighter Frame
#[skyline::hook(offset = NESS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ness_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let offense_up_timer = WorkModule::get_int(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    let offense_up_effect_timer = WorkModule::get_int(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
    if WorkModule::is_flag(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        //Timer Mechanics
        if offense_up_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
            if offense_up_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("ness_pkfl_hold"), false, false);
                EffectModule::kill_kind(boma, Hash40::new("sys_status_attack_up"), false, false);
            }
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
            WorkModule::set_int(boma, 0, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
        }
        //Effect Mechanics
        WorkModule::inc_int(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
        if offense_up_effect_timer > 25 {
            WorkModule::set_int(boma, 0, *FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
        }
        if offense_up_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if offense_up_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("ness_pkfl_hold"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_attack_up"), false, false);
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        //Game Mechanics
        AttackModule::set_power_up(boma, 1.1);
        AttackModule::set_reaction_mul(boma, 0.87);
    }
    original!()(vtable, fighter)
}

//Ness Reflect Attack Event
#[skyline::hook(offset = NESS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET)]
unsafe extern "C" fn ness_reflect_attack_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        WorkModule::on_flag(boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);   
    }
    original!()(vtable, fighter, unk)
}

unsafe extern "C" fn ness_pk_fire_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id as u32);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && status_kind == *WEAPON_NESS_PK_FIRE_STATUS_KIND_SHOOT {
        if WorkModule::is_flag(owner_boma, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
            *(weapon as *mut bool).add(0x90) = true;
            StatusModule::change_status_request(boma, *WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR, false);
        }
        else {
            *(weapon as *mut bool).add(0x90) = false;
        }
    }
    normal_weapon_hit_handler(vtable, weapon, log)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51f4c90).data(ness_pk_fire_on_attack as u64);
    skyline::install_hooks!(
        ness_start_initialization,
        ness_reset_initialization,
        //ness_death_initialization
        ness_reflect_attack_event
    );
}