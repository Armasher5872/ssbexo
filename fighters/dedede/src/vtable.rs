use super::*;

const DEDEDE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x903c80; //Dedede only
const DEDEDE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x904520; //Dedede only
const DEDEDE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x904cf0; //Dedede only
const DEDEDE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x904e70; //Dedede only

unsafe extern "C" fn dedede_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
}

//Dedede Startup Initialization
#[skyline::hook(offset = DEDEDE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dedede_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    dedede_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Dedede Reset Initialization
#[skyline::hook(offset = DEDEDE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dedede_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    dedede_var(&mut *boma);
    original!()(vtable, fighter)
}

//Dedede Death Initialization
#[skyline::hook(offset = DEDEDE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dedede_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    dedede_var(&mut *boma);
    original!()(vtable, fighter)
}

//Dedede Once Per Fighter Frame
#[skyline::hook(offset = DEDEDE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn dedede_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    if status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT {
        let obj_id = WorkModule::get_int(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
        let obj_boma = smash::app::sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        let item = WorkModule::get_int(boma, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            let fused_item = if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::get_int(owner_boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM)
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::get_int(owner_boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            }
            else {
                WorkModule::get_int(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            };
            WorkModule::set_int(boma, fused_item, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        }
        if WorkModule::is_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SHOT_OBJECT_SHOOT) && item != *ITEM_KIND_NONE {
            WorkModule::on_flag(boma, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
        }
        if WorkModule::is_flag(boma, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK) && frame >= 7.0 {
            if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
                set_arrow_fuse_params(obj_boma, item, FuseKind::REFUSE, i32::MAX);
            }
            let item_id = WorkModule::get_int(obj_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if !LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                VisibilityModule::set_whole(item_boma, true);
                LinkModule::link(item_boma, *ITEM_LINK_NO_HAVE, obj_id);
                if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
                    LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
                }
                else {
                    LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("have"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    let offset_pos = Vector3f{x: 0.0, y: 9.5, z: 0.0};
                    LinkModule::set_constraint_translate_offset(item_boma, &offset_pos);
                }
            }
            WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            WorkModule::on_flag(obj_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW);
            WorkModule::off_flag(boma, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        dedede_start_initialization,
        dedede_reset_initialization,
        dedede_death_initialization,
        dedede_opff
    );
}