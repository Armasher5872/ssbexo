//Credit to Hewdraw Remix
use super::*;

pub unsafe extern "C" fn check_waveland_validity(fighter: &mut L2CFighterCommon) -> bool {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let pos = *PostureModule::pos(boma);
    let dir_y = WorkModule::get_float(boma, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let upper_bound_offset_y = if StatusModule::is_changing(boma) && prev_status_kind != *FIGHTER_STATUS_KIND_PASS {
        WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ECB_OFFSET_C)
    } 
    else {
        WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ECB_OFFSET_B)
    };
    let upper_bound_y = pos.y+upper_bound_offset_y;
    let lower_bound = Vector2f::new(pos.x, pos.y-6.0);
    let ground_pos_any = &mut Vector2f::zero();
    let ground_pos_stage = &mut Vector2f::zero();
    let is_touch_any = GroundModule::line_segment_check(boma, &Vector2f::new(pos.x, upper_bound_y), &lower_bound, &Vector2f::zero(), ground_pos_any, true);
    let is_touch_stage = GroundModule::line_segment_check(boma, &Vector2f::new(pos.x, upper_bound_y), &lower_bound, &Vector2f::zero(), ground_pos_stage, false);
    let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage == 0 as *const *const u64 && dir_y > 0.0));
    if prev_status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        return false;
    }
    if can_snap {
        PostureModule::set_pos(boma, &Vector3f::new(pos.x, ground_pos_any.y+0.1, pos.z));
        return true;
    }
    else {
        return false;
    }
}