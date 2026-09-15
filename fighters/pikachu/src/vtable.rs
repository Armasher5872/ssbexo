use super::*;

//Pikachu & Pichu Reset Initialization
unsafe extern "C" fn pikachu_pichu_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_PIKACHU {
        pikachu_var(&mut *boma);
    }
    common_reset_variable_reset(&mut *boma);
}

//Pikachu & Pichu Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PIKACHU, 7, false, false))]
unsafe extern "C" fn pikachu_pichu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_PIKACHU {
        pikachu_var(&mut *boma);
    }
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Pikachu & Pichu On Attack
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PIKACHU, 36, false, false))]
unsafe extern "C" fn pikachu_pichu_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let kind = fighter.battle_object.kind as i32;
    if kind == *FIGHTER_KIND_PIKACHU {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        let collision_log = log as *mut CollisionLogScuffed;
        let collision_kind = (*collision_log).collision_kind;
        let opponent_object_id = (*collision_log).opponent_object_id;
        if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            let opponent_object = get_battle_object_from_id(opponent_object_id);
            let opponent_battle_object_id = (*opponent_object).battle_object_id;
            if [1, 2].contains(&collision_kind) {
                if opponent_battle_object_id >> 0x1C == 0 {
                    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_LAST_ATTACK_HITBOX_ID) == 1 {
                            WorkModule::on_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_WEAK);
                        }
                    }
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_PIKACHU, 4, false, true)).data(pikachu_pichu_reset_initialization as *const () as u64);
    skyline::install_hooks!(
        pikachu_pichu_death_initialization,
        pikachu_pichu_on_attack
    );
}