use super::*;

unsafe extern "C" fn dedede_frame(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status_kind)
    && fighter.global_table[CURRENT_FRAME].get_f32() < 1.0
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_NEWDEDEDEHAMMER, true, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_NEWDEDEDEHAMMER, true, ArticleOperationTarget(0));
        VisibilityModule::set_int64(fighter.module_accessor, hash40("hammer") as i64, hash40("hammer_disp_off") as i64);
    }
    if status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT {
        let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
        let obj_boma = smash::app::sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        let item = WorkModule::get_int(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            let fused_item = if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::get_int(owner_boma, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM)
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::get_int(owner_boma, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            }
            else {
                WorkModule::get_int(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
            };
            WorkModule::set_int(fighter.module_accessor, fused_item, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        }
        else if obj_kind == *WEAPON_KIND_LINK_BOOMERANG {
            let fused_item = if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::get_int(owner_boma, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM)
            }
            else {
                WorkModule::get_int(owner_boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE)
            };
            WorkModule::set_int(fighter.module_accessor, fused_item, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SHOT_OBJECT_SHOOT) && item != *ITEM_KIND_NONE {
            WorkModule::on_flag(fighter.module_accessor,FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
        }
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK) && MotionModule::frame(fighter.module_accessor) >= 7.0 {
            if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
                set_arrow_fuse_params(obj_boma, item, FuseKind::REFUSE, i32::MAX);
            }
            else {
                WorkModule::on_flag(obj_boma, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
                set_boomerang_fuse_params(obj_boma, item, FuseKind::REFUSE, i32::MAX);
            }
            let item_id = WorkModule::get_int(obj_boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
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
            WorkModule::set_int(fighter.module_accessor, *ITEM_KIND_NONE, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            WorkModule::on_flag(obj_boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
        }
    }
}

unsafe extern "C" fn dedede_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, smash::app::sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    //Dedede
    WorkModule::set_flag(boma, false, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
    WorkModule::set_flag(boma, false, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
    WorkModule::set_int(boma, 0, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
}

pub fn install() {
    Agent::new("dedede")
    .on_start(dedede_init)
    .on_line(Main, dedede_frame)
    .install()
    ;
}