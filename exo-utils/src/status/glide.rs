//Credit to: Liam Estares (LKE Studios), Ben Hall (arthur!), and WuBoytH

pub struct GlideParams {
    pub angle_max_up : f32, //#0 Max Upward Angle
    pub angle_max_down : f32, //#1 Max Downward Angle
    pub v_glide_start : f32, //#2 V Speed added on GlideStart
    pub gravity_start : f32, //#3 Gravity multiplier on GlideStart
    pub speed_mul_start : f32, //#4 H speed multiplier on GlideStart
    pub base_speed : f32, //#5 Base Power/Speed
    pub speed_change : f32, //#6 Power Rate
    pub max_speed : f32, //#7 Maximum Speed
    pub end_speed : f32, //#8 End Speedc
    pub gravity_accel : f32, //#9 Gravity Acceleration
    pub gravity_speed : f32, //#10 Gravity Max Speed
    pub angle_extra : f32, //#11 Angle stuff but unknown what this is for
    pub angle_more_speed : f32, //#12 Angle to gain more speed
    pub down_speed_add : f32, //#13 Max added H speed gained aiming downward
    pub unknown : f32, //#14 Unknown
    pub radial_stick : f32, //#15 Radial Stick Sensitivity
    pub up_angle_accel : f32, //#16 Upward angular acceleration
    pub down_angle_accel : f32, //#17 Downward angular acceleration
    pub max_angle_speed : f32, //#18 Maximum angular speed
    pub add_angle_speed : f32 //#19 Added angular speed for when stick is center
}

impl GlideParams {
    pub fn get() -> GlideParams {
        GlideParams {
            angle_max_up: 80.0,
            angle_max_down: -70.0,
            v_glide_start: 0.75,
            gravity_start: 1.0,
            speed_mul_start: 1.0,
            base_speed: 1.7,
            speed_change: 0.04,
            max_speed: 2.2,
            end_speed: 0.7,
            gravity_accel: 0.03,
            gravity_speed: 0.6,
            angle_extra: 15.0,
            angle_more_speed: -25.0,
            down_speed_add: 0.03,
            unknown: 0.15,
            radial_stick: 0.25,
            up_angle_accel: 0.55,
            down_angle_accel: 0.75,
            max_angle_speed: 7.0,
            add_angle_speed: 1.0
        }
    }
}