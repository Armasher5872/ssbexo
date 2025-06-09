use super::*;

#[repr(C)]
pub struct FlyData {
    pub turn_stick_x: f32,
    pub init_speed_x_mul: f32,
    pub speed_x_mul: f32,
    pub speed_x_max_mul: f32,
    pub speed_y_table_start: *const f32,
    pub speed_y_table_end: *const f32,
    pub speed_y_table_eos: *const f32,
    pub turn_param_start: *const i32,
    pub turn_param_end: *const i32,
    pub turn_param_eos: *const i32,
    pub shoot_fly_next_frame: i32
}

impl FlyData {
    pub fn get_from_fighter_kind(kind: i32) -> Option<&'static Self> {
        #[repr(C)]
        struct FlyDataResult {
            vtable: *const *const (),
            data: *const *const FlyData
        }
        unsafe {
            let accessor = *((singletons::FighterParamAccessor2() as *const u8).add((kind as usize) * 0x38 + 0x70) as *const u64);
            let function: extern "C" fn(u64, u64) -> FlyDataResult = std::mem::transmute(*(*(accessor as *const *const u64)).add(0x2));
            let result = function(accessor, hash40("fly_data"));
            if (*result.data).is_null() {
                return None;
            } 
            else {
                return Some(&**result.data);
            }
        }
    }
}