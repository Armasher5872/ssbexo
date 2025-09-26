use super::*;

unsafe extern "C" fn link_bowarrow_haved_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_scale(weapon.module_accessor, 1.0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_haved_main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_haved_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let owner_boma = get_owner_boma(weapon);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let arrow_type = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    let bow_step = WorkModule::get_int(owner_boma, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
        if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
            if !WorkModule::is_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_INIT_FUSE) {
                if ItemModule::is_have_item(owner_boma, 0) {
                    set_arrow_fuse_params(weapon.module_accessor, ItemModule::get_have_item_kind(owner_boma, 0), FuseKind::FUSE, ItemModule::get_have_item_trait(owner_boma, 0) as i32);
                }
                else if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                    let kind = WorkModule::get_int(owner_boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
                    if kind != *ITEM_KIND_NONE {
                        set_arrow_fuse_params(weapon.module_accessor, kind, FuseKind::REFUSE, i32::MAX);
                    }
                }
                else {
                    if owner_kind == *FIGHTER_KIND_LINK {
                        WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
                    }
                    else if owner_kind == *FIGHTER_KIND_KIRBY {
                        WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
                    }
                }
                if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
                    let item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                    let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
                    LinkModule::remove_model_constraint(item_boma, true);
                    if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                        LinkModule::unlink(item_boma, *ITEM_LINK_NO_HAVE);
                    }
                    if !LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                        VisibilityModule::set_whole(item_boma, true);
                        LinkModule::link(item_boma, *ITEM_LINK_NO_HAVE, (*(weapon.module_accessor)).battle_object_id);
                        LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
                    }
                }
                WorkModule::on_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_INIT_FUSE);
            }
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
        if frame % 15.0 == 0.0 {
            EFFECT_FOLLOW(weapon, Hash40::new("link_light_arrow_charge"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, link_bowarrow_haved_main_status)
    .install()
    ;
}