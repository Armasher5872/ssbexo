//Credit to WuBoyTH for translating the OPFF Vtable for Inkling
use super::*;

//Inkling Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_INKLING, 4, false, false))]
unsafe extern "C" fn inkling_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    inkling_var(&mut *boma);
    original!()(vtable, fighter)
}

//Inkling Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_INKLING, 7, false, false))]
unsafe extern "C" fn inkling_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    inkling_var(&mut *boma);
    original!()(vtable, fighter)
}

//Inkling Once Per Fighter Frame
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_INKLING, 13, false, false))]
unsafe extern "C" fn inkling_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let ink_const = FighterSpecializer_Inkling::get_ink_work_id(fighter.battle_object.kind as i32);
    let current_pos = PostureModule::pos_x(boma);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let ink_height = WorkModule::get_float(boma, 0x4D);
    let ink_radius = WorkModule::get_float(boma, 0x4E);
    let ink = WorkModule::get_float(boma, ink_const);
    let fpa2_accessor = (*((singletons::FighterParamAccessor2() as *const u8).add((0x41 as usize)*0x38+0x70) as *const u64)) as *const f32;
    let unk4 = *(fpa2_accessor).add(0xc8/0x4);
    let unk5 = *(fpa2_accessor).add(0xf0/0x4);
    let vtable_slow = *battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0;
    if vtable_slow {
        if !StopModule::is_stop(boma) {
            add_water_damage(boma, 0x100000BF /*FIGHTER_INKLING_INSTANCE_WORK_ID_INT_WATER_FRAME*/);
        }
        inkling_handle_tank_fill(boma, ink_height, ink_radius, ink, unk4, unk5);
    }
    if ArticleModule::is_exist(boma, 0x4) {
        let active_rollerink_count = ArticleModule::get_active_num(boma, 0x4);
        for idx in 0..active_rollerink_count {
            let rollerink_article = get_article_from_no(boma, 0x4, idx as i32);
            let rollerink_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(rollerink_article) as u32;
            let rollerink_boma = sv_battle_object::module_accessor(rollerink_battle_object_id);
            let rollerink_x_pos = WorkModule::get_float(rollerink_boma, 0x3);
            let rollerink_x_pos_max = rollerink_x_pos+8.0;
            let rollerink_x_pos_min = rollerink_x_pos-8.0;
            let is_standing_on_roller_ink = current_pos < rollerink_x_pos_max && current_pos > rollerink_x_pos_min;
            if is_standing_on_roller_ink && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK);
                break;
            }
            else {
                WorkModule::off_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK);
            }
        }
    }
    inkling_spawn_stage_ink(fighter);
}

pub fn install() {
    skyline::install_hooks!(
        inkling_reset_initialization,
        inkling_death_initialization,
        inkling_opff
    );
}