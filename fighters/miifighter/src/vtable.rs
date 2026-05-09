use super::*;

const MIIFIGHTER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xd56780; //Mii Brawler only
const MIIFIGHTER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd56e70; //Mii Brawler only
const MIIFIGHTER_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xd592c0; //Mii Brawler only
const MIIFIGHTER_VTABLE_ON_SEARCH_OFFSET: usize = 0xd59650; //Mii Brawler only
const MIIFIGHTER_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Shared

//Mii Brawler Reset Initialization
#[skyline::hook(offset = MIIFIGHTER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miifighter_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    miifighter_var(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Brawler Death Initialization
#[skyline::hook(offset = MIIFIGHTER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miifighter_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    miifighter_var(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Brawler OPFF
#[skyline::hook(offset = MIIFIGHTER_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn miifighter_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let sticky = ControlModule::get_stick_y(boma);
    let status_kind = StatusModule::status_kind(boma);
    let customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1 {
        if ArticleModule::is_exist(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL) {
            param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, vec![-1].clone(), (hash40("param_special_n"), hash40("n1_throw_angle"), (90.0*sticky).abs().clamp(0.0, 90.0)));
        }
    }
    original!()(vtable, fighter)
}

//Mii Brawler On Search
#[skyline::hook(offset = MIIFIGHTER_VTABLE_ON_SEARCH_OFFSET)]
unsafe extern "C" fn miifighter_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let status_kind = StatusModule::status_kind(boma);
    let customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let opponent_id = (*collision_log).opponent_battle_object_id;
            WorkModule::set_int(boma, opponent_id as i32, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
            WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
            if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let counter_throw_boma = sv_battle_object::module_accessor(opponent_id as u32);
                    LinkModule::remove_model_constraint(counter_throw_boma, true);
                    if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                        LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                    }
                    if !LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                        VisibilityModule::set_whole(counter_throw_boma, true);
                        LinkModule::link(counter_throw_boma, *LINK_NO_ARTICLE, (*boma).battle_object_id);
                        LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *LINK_NO_ARTICLE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                        LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                    }
                    GroundModule::set_ignore_boss(counter_throw_boma, true);
                    GroundModule::set_passable_check(counter_throw_boma, false);
                    GroundModule::set_collidable(counter_throw_boma, false);
                    JostleModule::set_status(counter_throw_boma, false);
                    WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, false);
                }
                if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_ITEM {
                    let counter_throw_boma = sv_battle_object::module_accessor(opponent_id as u32);
                    LinkModule::remove_model_constraint(counter_throw_boma, true);
                    if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                        LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                    }
                    if !LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                        VisibilityModule::set_whole(counter_throw_boma, true);
                        LinkModule::link(counter_throw_boma, *ITEM_LINK_NO_HAVE, (*boma).battle_object_id);
                        LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                        LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                    }
                    GroundModule::set_ignore_boss(counter_throw_boma, true);
                    GroundModule::set_passable_check(counter_throw_boma, false);
                    GroundModule::set_collidable(counter_throw_boma, false);
                    JostleModule::set_status(counter_throw_boma, false);
                    WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, false);
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Mii Brawler On Damage
#[skyline::hook(offset = MIIFIGHTER_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn miifighter_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MIIFIGHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        let customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
        if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
            if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE].contains(&status_kind) {
                DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
                WorkModule::off_flag(boma, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
            }
        }
    }
    original!()(vtable, fighter, on_damage)
}

pub fn install() {
	skyline::install_hooks!(
        miifighter_reset_initialization,
        miifighter_death_initialization,
        miifighter_opff,
        miifighter_on_search,
        miifighter_on_damage
    );
}