use super::*;

#[derive(Default, Copy, Clone)]
pub struct SonicMeter {
    pub base_bar: u64,
    pub bar1: u64,
    pub bar2: u64,
    pub original_bar_width: f32,
    pub original_bar_height: f32,
    pub original_bar2_width: f32,
    pub original_bar2_height: f32,
    pub percent: f32,
    pub enabled: bool,
}

impl SonicMeter {
    pub fn new(layout_data: u64) -> Self {
        let base_bar = get_pane_from_layout(layout_data, "sonic_meter_base\0").expect("Could not find base meter!");
        let bar1 = get_pane_from_layout(layout_data, "sonic_bar1\0").expect("Could not find first bar!");
        let bar2 = get_pane_from_layout(layout_data, "sonic_bar2\0").expect("Could not find second bar!");
        return Self {
            base_bar: base_bar,
            bar1: bar1,
            bar2: bar2,
            original_bar_width: -1.0,
            original_bar_height: -1.0,
            original_bar2_width: -1.0,
            original_bar2_height: -1.0,
            percent: -1.0,
            enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.base_bar, true);
        set_pane_visible(self.bar1, true);
        set_pane_visible(self.bar2, true);
        if self.original_bar_height < 0.0 {
            self.original_bar_width = get_width_height(self.bar1).0;
            self.original_bar_height = get_width_height(self.bar1).1;
            self.original_bar2_width = get_width_height(self.bar2).0;
            self.original_bar2_height = get_width_height(self.bar2).1;
        }
        self.percent = 0.0;
    }
    pub fn set_percent(&mut self, gauge: f32) {
        let percent = gauge/100.0;
        self.percent = percent;
    }
    pub fn update_number(&mut self) {
        let percentage = 0.38+(self.percent*0.52);
        set_tex_coords(self.bar2, [0.0, 0.0, percentage, 0.0, 0.0, 1.0, percentage, 1.0]);
        set_width_height(self.bar2, self.original_bar2_width*percentage, self.original_bar2_height);
    }
}

pub unsafe extern "C" fn fun_7100015020(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let max_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let mut ret = 0;
    if situation_kind == *SITUATION_KIND_GROUND {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
        }
        else {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
            }
            else {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
                    if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
                        ret = 1;
                    }
                }
            }
        }
    }
    else {
        if !fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                if jump_count < max_jump_count {
                    fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                    ret = 1;
                }
            }
            else {
                ret = 0;
            }
        }
    }
    ret.into()
}

//Credited to WuBoyTH
#[no_mangle]
pub unsafe extern "C" fn sonic_set_lightweight(boma: *mut BattleObjectModuleAccessor, new_stats: bool) {
    if new_stats {
        let mut changes = Vec::new();
        changes.push(StatChange::new(hash40("walk_accel_mul"), 1.1));
        changes.push(StatChange::new(hash40("walk_accel_add"), 1.1));
        changes.push(StatChange::new(hash40("walk_speed_max"), 1.1));
        changes.push(StatChange::new(hash40("dash_speed"), 1.1));
        changes.push(StatChange::new(hash40("run_accel_mul"), 1.1));
        changes.push(StatChange::new(hash40("run_accel_add"), 1.1));
        changes.push(StatChange::new(hash40("run_speed_max"), 1.1));
        changes.push(StatChange::new(hash40("air_accel_x_mul"), 1.1));
        changes.push(StatChange::new(hash40("air_accel_x_add"), 1.1));
        changes.push(StatChange::new(hash40("air_speed_x_stable"), 1.1));
        set_lightweight(boma, changes);
    }
    else {
        disable_lightweight(boma);
    }
}