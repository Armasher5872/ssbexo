use super::*;

const WARIO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1285a70; //Wario only
const WARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x12864e0; //Wario only
const WARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12868c0; //Wario only
const WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x1286ae0; //Wario only
const WARIO_VTABLE_ON_ATTACK_OFFSET: usize = 0x1287320; //Wario only
const WARIO_VTABLE_LINK_EVENT_OFFSET: usize = 0x12876c0; //Wario only

unsafe extern "C" fn wario_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn wario_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_HIT);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SWING_DING_MOVE);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_UP_SPECIAL_TIMER);
}

//Wario Startup Initialization
#[skyline::hook(offset = WARIO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wario_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    wario_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(wario_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Wario Reset Initialization
#[skyline::hook(offset = WARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wario_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    wario_var(&mut *boma);
    original!()(vtable, fighter)
}

//Wario Death Initialization
#[skyline::hook(offset = WARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    wario_var(&mut *boma);
    original!()(vtable, fighter)
}

//Wario Once Per Fighter Frame
#[skyline::hook(offset = WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn wario_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let head_scale = &Vector3f{x: 0.9, y: 0.9, z: 0.9};
    let foot_scale = &Vector3f{x: 0.9, y: 0.9, z: 0.9};
    let clavicle_scale = &Vector3f{x: 1.2, y: 1.2, z: 1.2};
    let arm_scale = &Vector3f{x: 0.92, y: 0.92, z: 0.92};
    let leg_scale = &Vector3f{x: 1.05, y: 1.05, z: 1.05};
    //Wario Scaling
    ModelModule::set_joint_scale(boma, Hash40::new("neck"), head_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("clavicler"), clavicle_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("claviclel"), clavicle_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), arm_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("shoulderl"), arm_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("legr"), leg_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("legl"), leg_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("footr"), foot_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("footl"), foot_scale);
    original!()(vtable, fighter)
}

//Wario On Attack
#[skyline::hook(offset = WARIO_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn wario_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if [
        *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_WARIO_STATUS_KIND_ATTACK_DASH_LOOP, *FIGHTER_WARIO_STATUS_KIND_ATTACK_DASH_JUMP_SQUAT, *FIGHTER_WARIO_STATUS_KIND_ATTACK_DASH_AIR_LOOP, *FIGHTER_WARIO_STATUS_KIND_ATTACK_DASH_LANDING,
        *FIGHTER_WARIO_STATUS_KIND_ATTACK_DASH_END
    ].contains(&status_kind) {
        WorkModule::on_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_HIT);
    }
    call_original!(vtable, fighter, log)
}

//Wario Link Event
#[skyline::hook(offset = WARIO_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn wario_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let ret = original!()(vtable, fighter, event);
    if event.link_event_kind.0 == hash40("capture") {
        let boma = fighter.battle_object.module_accessor;
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if StatusModule::status_kind(boma) == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW, false);
            return 0;
        }
        return 1;
    }
    ret
}

pub fn install() {
	skyline::install_hooks!(
        wario_start_initialization,
        wario_reset_initialization,
        wario_death_initialization,
        wario_opff,
        wario_on_attack,
        wario_link_event
    );
}