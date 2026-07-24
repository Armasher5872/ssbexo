use super::*;

const WARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x12864e0; //Wario only
const WARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12868c0; //Wario only
const WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x1286ae0; //Wario only
const WARIO_VTABLE_ONCE_PER_FIGHTER_FRAME_2_OFFSET: usize = 0x128b0b0; //Wario only
const WARIO_VTABLE_ON_ATTACK_OFFSET: usize = 0x1287320; //Wario only
const WARIO_VTABLE_LINK_EVENT_OFFSET: usize = 0x12876c0; //Wario only
const WARIO_VTABLE_ON_DAMAGE_OFFSET: usize = 0x12887e0; //Wario only

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
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
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
    //Clears effects and sounds from Toot Kamikaze
    if status_kind == *FIGHTER_STATUS_KIND_DEAD {
        if prev_status_kind == *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE {
            STOP_SE(agent, Hash40::new("se_wario_special_s01"));
            STOP_SE(agent, Hash40::new("se_wario_special_s07"));
            STOP_SE(agent, Hash40::new("vc_wario_missfoot01"));
            STOP_SE(agent, Hash40::new("vc_wario_missfoot02"));
            EffectModule::kill_kind(boma, Hash40::new("sys_dead2"), true, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_dead2_ground"), true, true);
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
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_object = get_battle_object_from_id(opponent_object_id);
        let opponent_battle_object_id = (*opponent_object).battle_object_id;
        if [
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_AIR_LOOP, 
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_JUMPSQUAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_LANDING
        ].contains(&status_kind) {
            if collision_kind == 1 {
                if opponent_battle_object_id >> 0x1C == 0 {
                    StatusModule::change_status_request(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_HIT_END, false);
                }
            }
            if collision_kind == 2 {
                if opponent_battle_object_id >> 0x1C == 0 {
                    StatusModule::change_status_request(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WALL_END, false);
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
            StatusModule::change_status_request(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH, false);
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Wario On Damage
#[skyline::hook(offset = WARIO_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn wario_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_WARIO_STATUS_KIND_APPEAL_KAMIKAZE {
        MotionModule::change_motion(boma, Hash40::new("appeal_kamikaze"), 58.0, 1.0, false, 0.0, false, false);
        ArticleModule::generate_article(boma, FIGHTER_WARIO_GENERATE_ARTICLE_KAMIKAZE, false, -1);
    }
    original!()(vtable, fighter, on_damage)
}

pub fn install() {
	skyline::install_hooks!(
        wario_reset_initialization,
        wario_death_initialization,
        wario_opff,
        wario_opff_2,
        wario_on_attack,
        wario_link_event,
        wario_on_damage
    );
}