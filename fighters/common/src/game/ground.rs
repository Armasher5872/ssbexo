//The following section is credited to HewDraw Remix, and is used to get ecb points to make certain mechanics (like wavelanding) smoother
use super::*;

/* 
The following function calculates the various ecb point offsets relative to your top bone

map_coll_data is a pointer to the characters vl.prc's map_coll_data bones
ecb_point_l is the left horizontal offset, and always returns a negative number
ecb_point_b is the bottom vertical offset, and is 0.0 in vanilla
ecb_point_r is the right horizontal offset, and always returns a positive number
ecb_point_u is the top vertical offset, and always returns a positive number
lock_ecb_b is a param that appears to lock the bottom most ecb point to 0.0. This value is defaulted to 0 (likely a bool) in vanilla for fighters, but if set to 1, it'll calculate it normally
*/
#[skyline::hook(offset = 0x45f440)]
unsafe fn groundmodule_ecb_point_calculation(ground_module: u64, map_coll_data: *mut *mut Hash40, ecb_point_l: *mut f32, ecb_point_b: *mut f32, ecb_point_r: *mut f32, ecp_point_u: *mut f32, lock_ecb_b: u32) {
    let boma = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    let category = (*boma).battle_object_id >> 0x1C;
    if category == 0 {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CALCULATING_ECB);
    }
    call_original!(ground_module, map_coll_data, ecb_point_l, ecb_point_b, ecb_point_r, ecp_point_u, lock_ecb_b);
    if category == 0 {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CALCULATING_ECB);
        WorkModule::set_float(boma, *ecb_point_b, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ECB_OFFSET_B);
        WorkModule::set_float(boma, ((*ecp_point_u-*ecb_point_b)/2.0)+*ecb_point_b, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ECB_OFFSET_C);
    }
}

//FighterUtil::get_ground_correct_kind_air_trans, Aerial ECB fixes for Link, Captain, Simon, Richter
#[skyline::hook(replace = FighterUtil::get_ground_correct_kind_air_trans)]
unsafe fn get_ground_correct_kind_air_trans_hook(_boma: &mut BattleObjectModuleAccessor, _something: i32) -> i32 {
    *GROUND_CORRECT_KIND_AIR
}

pub fn install() {
    skyline::install_hooks!(
        groundmodule_ecb_point_calculation,
        get_ground_correct_kind_air_trans_hook
    );
}