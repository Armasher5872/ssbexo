use super::*;

const GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xadaf50; //Greninja only
const GEKKOUGA_VTABLE_ON_ATTACK_OFFSET: usize = 0xadb1a0; //Greninja only

//Greninja Reset Initialization
unsafe extern "C" fn gekkouga_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
}

//Greninja Death Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
    original!()(vtable, fighter, param_3)
}

//Greninja On Attack Event
#[skyline::hook(offset = GEKKOUGA_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn gekkouga_on_attack(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let attack_air_motion_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if attack_air_motion_kind == hash40("attack_air_lw") {
            WorkModule::on_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_BOUND);
            WorkModule::set_int(boma, jump_count, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_JUMP_COUNT);
            if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let opponent_object = get_battle_object_from_id(opponent_object_id);
                let opponent_battle_object_id = (*opponent_object).battle_object_id;
                if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                    let opponent_boma = sv_battle_object::module_accessor(opponent_battle_object_id);
                    StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
                }
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_THROW {
        if motion_kind == hash40("throw_lw") {
            if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let opponent_object = get_battle_object_from_id(opponent_object_id);
                let opponent_battle_object_id = (*opponent_object).battle_object_id;
                if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
                    let opponent_boma = sv_battle_object::module_accessor(opponent_battle_object_id);
                    StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
                }
            }
        }
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fbf2d8).data(gekkouga_reset_initialization as *const () as *const u64);
    skyline::install_hooks!(
        gekkouga_death_initialization,
        gekkouga_on_attack
    );
}