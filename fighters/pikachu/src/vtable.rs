use super::*;

const PIKACHU_PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf2a530; //Shared
const PIKACHU_PICHU_VTABLE_ON_ATTACK_OFFSET: usize = 0xf2ae00; //Shared

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
#[skyline::hook(offset = PIKACHU_PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
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
#[skyline::hook(offset = PIKACHU_PICHU_VTABLE_ON_ATTACK_OFFSET)]
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
                        if LAST_ATTACK_HITBOX_ID == 1 {
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
    let _ = skyline::patching::Patch::in_text(0x5012668).data(pikachu_pichu_reset_initialization as *const () as u64);
    skyline::install_hooks!(
        pikachu_pichu_death_initialization,
        pikachu_pichu_on_attack
    );
}