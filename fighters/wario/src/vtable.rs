use super::*;

const WARIO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1285a70; //Wario only
const WARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x12864e0; //Wario only
const WARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12868c0; //Wario only
const WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x1286ae0; //Wario only
const WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_2_OFFSET: usize = 0x128b0b0; //Wario only
const WARIO_VTABLE_ON_ATTACK_OFFSET: usize = 0x1287320; //Wario only
const WARIO_VTABLE_LINK_EVENT_OFFSET: usize = 0x12876c0; //Wario only
const WARIO_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x12881c0; //Wario only

unsafe extern "C" fn wario_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn wario_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 1.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_PILEDRIVER_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
}

//Wario Startup Initialization
#[skyline::hook(offset = WARIO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wario_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    wario_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
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
unsafe extern "C" fn wario_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let head_scale = &Vector3f{x: 0.91, y: 0.91, z: 0.91};
    let foot_scale = &Vector3f{x: 0.9, y: 0.9, z: 0.9};
    let clavicle_scale = &Vector3f{x: 1.19, y: 1.19, z: 1.19};
    let arm_scale = &Vector3f{x: 0.92, y: 0.92, z: 0.92};
    let leg_scale = &Vector3f{x: 1.05, y: 1.05, z: 1.05};
    //Wario Scaling
    ModelModule::set_joint_scale(boma, Hash40::new("neck"), head_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("legr"), leg_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("legl"), leg_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("footr"), foot_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("footl"), foot_scale);
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_END, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_HIT_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END
    ].contains(&status_kind) {
        ModelModule::set_joint_scale(boma, Hash40::new("clavicler"), clavicle_scale);
        ModelModule::set_joint_scale(boma, Hash40::new("claviclel"), clavicle_scale);
        ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), arm_scale);
        ModelModule::set_joint_scale(boma, Hash40::new("shoulderl"), arm_scale);
    }
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) && (8.0..=49.0).contains(&frame) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_APPEAL_GAS, false);
            }
        }
    }
}

//Wario Once Per Fighter Frame 2
#[skyline::hook(offset = WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_2_OFFSET)]
unsafe extern "C" fn wario_opff_2(_vtable: u64, _boma: &mut BattleObjectModuleAccessor) {

}

//Wario On Attack
#[skyline::hook(offset = WARIO_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn wario_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 && opponent_object_id != 0 {
        let opponent_object = get_battle_object_from_id(opponent_object_id);
        let opponent_battle_object_id = (*opponent_object).battle_object_id;
        if [
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, 
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING
        ].contains(&status_kind) && !WorkModule::is_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION) {
            if collision_kind == 1 {
                if opponent_battle_object_id >> 0x1C == 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_HIT_END, false);
                }
            }
            if collision_kind == 2 {
                if opponent_battle_object_id >> 0x1C == 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END, false);
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Wario Link Event
#[skyline::hook(offset = WARIO_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn wario_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        if StatusModule::status_kind(boma) == FIGHTER_STATUS_KIND_SPECIAL_N && capture_event.status == *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START {
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.constraint = true;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, false);
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Wario On Search
#[skyline::hook(offset = WARIO_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn wario_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let status_kind = StatusModule::status_kind(boma);
    if [
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, 
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING
    ].contains(&status_kind) {
        let opponent_id = (*collision_log).opponent_battle_object_id;
        if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                let opponent_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
                let opponent_kind = utility::get_kind(&mut *opponent_boma);
                let opponent_status_kind = StatusModule::status_kind(opponent_boma);
                let donkey_check = opponent_kind == *FIGHTER_KIND_DONKEY && opponent_status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO;
                let yoshi_check = opponent_kind == *FIGHTER_KIND_YOSHI && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2].contains(&opponent_status_kind);
                let kirby_check = opponent_kind == *FIGHTER_KIND_KIRBY && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP].contains(&opponent_status_kind);
                let koopa_check = opponent_kind == *FIGHTER_KIND_KOOPA && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let ganon_check = opponent_kind == *FIGHTER_KIND_GANON && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let mewtwo_check = opponent_kind == *FIGHTER_KIND_MEWTWO && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let wario_check = opponent_kind == *FIGHTER_KIND_WARIO && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N;
                let diddy_check = opponent_kind == *FIGHTER_KIND_DIDDY && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let dedede_check = opponent_kind == *FIGHTER_KIND_DEDEDE && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_LOOP].contains(&opponent_status_kind);
                let lucario_check = opponent_kind == *FIGHTER_KIND_LUCARIO && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let miifighter_check = opponent_kind == *FIGHTER_KIND_MIIFIGHTER && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH].contains(&opponent_status_kind);
                let reflet_check = opponent_kind == *FIGHTER_KIND_REFLET && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW;
                let ridley_check = opponent_kind == *FIGHTER_KIND_RIDLEY && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let shizue_check = opponent_kind == *FIGHTER_KIND_SHIZUE && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START].contains(&opponent_status_kind);
                let gaogaen_check = opponent_kind == *FIGHTER_KIND_GAOGAEN && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S;
                let jack_check = opponent_kind == *FIGHTER_KIND_JACK && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI;
                let demon_check = opponent_kind == *FIGHTER_KIND_DEMON && opponent_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW;
                if [*FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&opponent_status_kind)
                || donkey_check || yoshi_check || kirby_check || koopa_check || ganon_check || mewtwo_check || wario_check || diddy_check || dedede_check || lucario_check || miifighter_check || reflet_check || ridley_check || shizue_check || gaogaen_check 
                || jack_check || demon_check {
                    WorkModule::on_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_INVALID_TRANSITION);
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        wario_start_initialization,
        wario_reset_initialization,
        wario_death_initialization,
        wario_opff,
        wario_opff_2,
        wario_on_attack,
        wario_link_event,
        wario_on_search
    );
}