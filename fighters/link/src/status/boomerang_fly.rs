use super::*;

unsafe extern "C" fn link_boomerang_fly_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let status_kind = weapon.global_table[STATUS_KIND].get_i32();
    let fuse_item_id = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
    if ![*WN_LINK_BOOMERANG_STATUS_KIND_FLY, *WN_LINK_BOOMERANG_STATUS_KIND_HOP, *WN_LINK_BOOMERANG_STATUS_KIND_TURN, *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED].contains(&status_kind) {
        if fuse_item_id != *BATTLE_OBJECT_ID_INVALID as u32 && fuse_item_id != 0 && sv_battle_object::is_active(fuse_item_id) {
            let item_boma = smash::app::sv_battle_object::module_accessor(fuse_item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("link_boomerang")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_FLY, link_boomerang_fly_end_status)
    .install()
    ;
}