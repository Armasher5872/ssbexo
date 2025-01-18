use super::*;

const LUCAS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc73040; //Lucas only
const LUCAS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const LUCAS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc732c0; //Lucas only
const LUCAS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc73570; //Lucas only
const LUCAS_VTABLE_LINK_EVENT_OFFSET: usize = 0xc73e00; //Lucas only
const LUCAS_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET: usize = 0x68d8c0; //Shared
const LUCAS_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET: usize = 0x68d8d0; //Shared
const LUCAS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET: usize = 0xc73640; //Lucas only

unsafe extern "C" fn lucas_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
    WorkModule::off_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
    WorkModule::off_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
    WorkModule::off_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
    WorkModule::off_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN);
    WorkModule::set_int(boma, 0, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
}

//Lucas Startup Initialization
#[skyline::hook(offset = LUCAS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let shield_data = ShieldDataResource::new(0.0, 6.5, 0.0, 0.0, 6.5, 11.5, 13.0, Hash40::new("hip"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
    common_initialization_variable_reset(&mut *boma);
    lucas_var(&mut *boma);
    add_shield_group(boma, resource, *FIGHTER_LUCAS_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Lucas Reset Initialization
#[skyline::hook(offset = LUCAS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        lucas_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Lucas Death Initialization
#[skyline::hook(offset = LUCAS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    lucas_var(&mut *boma);
    original!()(vtable, fighter)
}

//Lucas Once Per Fighter Frame
#[skyline::hook(offset = LUCAS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn lucas_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let defense_up_timer = WorkModule::get_int(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    let defense_up_effect_timer = WorkModule::get_int(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
    if WorkModule::is_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        //Timer Mechanics
        if defense_up_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
            if defense_up_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("lucas_pkfr_hold"), false, false);
                EffectModule::kill_kind(boma, Hash40::new("sys_status_defense_up"), false, false);
            }
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
            WorkModule::set_int(boma, 0, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
        }
        //Effect Mechanics
        WorkModule::inc_int(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
        if defense_up_effect_timer > 25 {
            WorkModule::set_int(boma, 0, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_EFFECT_TIMER);
        }
        if defense_up_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if defense_up_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("lucas_pkfr_hold"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_defense_up"), false, false);
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        //Game Mechanics
        DamageModule::set_damage_mul(boma, 0.9);
    }
    println!("Full Smash Attack: {}", WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK));
    original!()(vtable, fighter)
}

//Lucas Link Event
#[skyline::hook(offset = LUCAS_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn lucas_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        let boma = fighter.battle_object.module_accessor;
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if StatusModule::status_kind(boma) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH && capture_event.status == *FIGHTER_STATUS_KIND_THROWN {
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            fighter.battle_object.change_status_req(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_THROW.into(), false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Lucas Shield Attack Detection Event
#[skyline::hook(offset = LUCAS_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET)]
unsafe extern "C" fn lucas_shield_attack_detection_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
        let boma = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            WorkModule::on_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
        }
    }
    original!()(vtable, fighter, unk)
}

//Lucas Shield Attack Transition Event
#[skyline::hook(offset = LUCAS_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET)]
unsafe extern "C" fn lucas_shield_attack_transition_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
        let boma = fighter.battle_object.module_accessor;
        if WorkModule::is_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, false);
        }
    }
    original!()(vtable, fighter, unk)
}

//Lucas Reflect Attack Event
#[skyline::hook(offset = LUCAS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET)]
unsafe extern "C" fn lucas_reflect_attack_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
        WorkModule::on_flag(boma, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);   
    }
    original!()(vtable, fighter, unk)
}

pub fn install() {
	skyline::install_hooks!(
        lucas_start_initialization,
        lucas_reset_initialization,
        lucas_death_initialization,
        lucas_opff,
        lucas_link_event,
        lucas_shield_attack_detection_event,
        lucas_shield_attack_transition_event,
        lucas_reflect_attack_event
    );
}