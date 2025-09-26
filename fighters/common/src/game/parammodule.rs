use super::*;

#[skyline::hook(offset=FLOAT_OFFSET)]
unsafe extern "C" fn get_param_float_impl_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let category = smash::app::utility::get_category(boma_reference);
    let fighter_kind = smash::app::utility::get_kind(boma_reference);
	let sticky = ControlModule::get_stick_y(boma);
	if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if fighter_kind == *FIGHTER_KIND_INKLING {
			if param_type == hash40("dash_speed") {
				if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK) {
					return original!()(module_accessor, param_type, param_hash);
				}
				else {
					return 1.5;
				}
			}
			if param_type == hash40("run_speed_max") {
				if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK) {
					return original!()(module_accessor, param_type, param_hash);
				}
				else {
					return 1.6;
				}
			}
		}
	}
	original!()(module_accessor, param_type, param_hash)
}

pub fn install() {
	skyline::install_hook!(get_param_float_impl_hook);
}
/*
lua2cpp::L2CFighterCommon::calc_walk_speed(L2CFighterCommon *this,L2CValue param_1,L2CValue param_2,L2CValue param_3,L2CValue param_4,L2CValue param_5,L2CValue param_6,L2CValue param_7,L2CValue param_8,L2CValue param_9) {
	pLVar8 = (L2CValue *)(ulong)param_2;
	this_00 = (L2CAgent *)(ulong)param_1;
	fVar9 = (float)app::lua_bind::ControlModule__get_stick_x_impl(this->moduleAccessor);
	lib::L2CValue::L2CValue(&LStack_80,fVar9);
	lib::L2CValue::operator/((L2CValue *)(ulong)param_3,pLVar8);
	lib::L2CValue::operator/((L2CValue *)(ulong)param_4,pLVar8);
	lib::L2CValue::operator*(&LStack_80,(L2CValue *)&LStack_90);
	pLVar1 = (L2CValue *)CONCAT44(param_9,param_8);
	lib::L2CValue::operator*(&LStack_70,pLVar1);
	lib::L2CValue::~L2CValue(&LStack_70);
	lib::L2CValue::L2CValue(&LStack_70,0.0);
	uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)&LStack_80);
	lib::L2CValue::~L2CValue(&LStack_70);
	if ((uVar5 & 1) == 0) {
		lib::L2CValue::operator-(&LStack_a0);
		lib::L2CValue::operator*((L2CValue *)(auStack_e0 + 0x10),pLVar1);
		lib::L2CValue::operator+(&LStack_b0,(L2CValue *)(auStack_e0 + 0x20));
		lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
		lib::L2CValue::~L2CValue(&LStack_70);
		lib::L2CValue::~L2CValue((L2CValue *)(auStack_e0 + 0x20));
		pLVar7 = (L2CValue *)(auStack_e0 + 0x10);
	}
	else {
		lib::L2CValue::operator*(&LStack_a0,pLVar1);
		lib::L2CValue::operator+(&LStack_b0,(L2CValue *)(auStack_e0 + 0x20));
		lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
		lib::L2CValue::~L2CValue(&LStack_70);
		pLVar7 = (L2CValue *)(auStack_e0 + 0x20);
	}
	lib::L2CValue::~L2CValue(pLVar7);
	lib::L2CValue::operator*(&LStack_80,pLVar1);
	lib::L2CValue::L2CValue(&LStack_70,0x6e5ec7051);
	lib::L2CValue::L2CValue((L2CValue *)auStack_e0,0x14cf046ee5);
	uVar5 = lib::L2CValue::as_integer(&LStack_70);
	uVar6 = lib::L2CValue::as_integer((L2CValue *)auStack_e0);
	fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar5,uVar6);
	lib::L2CValue::L2CValue((L2CValue *)(auStack_e0 + 0x10),fVar9);
	lib::L2CValue::~L2CValue((L2CValue *)auStack_e0);
	lib::L2CValue::~L2CValue(&LStack_70);
	uVar5 = lib::L2CValue::operator<((L2CValue *)(auStack_e0 + 0x10),(L2CValue *)(auStack_e0 + 0x20));
	if ((uVar5 & 1) != 0) {
		lib::L2CValue::operator=((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)(auStack_e0 + 0x10));
	}
	lib::L2CValue::L2CValue(&LStack_70,true);
	uVar5 = lib::L2CValue::operator==(in_stack_00000008,(L2CValue *)&LStack_70);
	lib::L2CValue::~L2CValue(&LStack_70);
	if ((uVar5 & 1) != 0) {
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
			lib::L2CValue::L2CValue(&LStack_110,0x112d83771b);
			lib::L2CValue::L2CValue(&LStack_120,0);
			uVar5 = lib::L2CValue::as_integer(&LStack_110);
			uVar6 = lib::L2CValue::as_integer(&LStack_120);
			fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar5,uVar6);
			lib::L2CValue::L2CValue(&LStack_100,fVar9);
			lib::L2CValue::operator/(&LStack_100,pLVar1);
			lib::L2CValue::L2CValue(&LStack_70,0.001);
			lib::L2CValue::operator-(&LStack_f0,(L2CValue *)&LStack_70);
			lib::L2CValue::~L2CValue(&LStack_70);
			lib::L2CValue::~L2CValue(&LStack_f0);
			lib::L2CValue::~L2CValue(&LStack_100);
			lib::L2CValue::~L2CValue(&LStack_120);
			lib::L2CValue::~L2CValue(&LStack_110);
			uVar5 = lib::L2CValue::operator<((L2CValue *)auStack_e0,(L2CValue *)(auStack_e0 + 0x20));
			if ((uVar5 & 1) == 0) {
				lib::L2CValue::operator-((L2CValue *)auStack_e0);
				uVar5 = lib::L2CValue::operator<((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)&LStack_70);
				lib::L2CValue::~L2CValue(&LStack_70);
				if ((uVar5 & 1) != 0) {
					lib::L2CValue::operator-((L2CValue *)auStack_e0);
					lib::L2CValue::operator=((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)&LStack_70);
					lib::L2CValue::~L2CValue(&LStack_70);
				}
			}
			else {
				lib::L2CValue::operator=((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)auStack_e0);
			}
			lib::L2CValue::~L2CValue((L2CValue *)auStack_e0);
		}
	}
	lib::L2CValue::L2CValue(&LStack_70,1e-05);
	uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)(auStack_e0 + 0x20));
	lib::L2CValue::~L2CValue(&LStack_70);
	if ((uVar5 & 1) == 0) {
		lib::L2CValue::L2CValue(&LStack_70,-1e-05);
		uVar5 = lib::L2CValue::operator<((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)&LStack_70);
		lib::L2CValue::~L2CValue(&LStack_70);
		if ((uVar5 & 1) == 0) {
			lib::L2CValue::L2CValue(&LStack_70,0.0);
			lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
			pLVar7 = &LStack_70;
		}
	}
	else {
		LAB_71001aa48c:
		lib::L2CValue::operator/((L2CValue *)this_00,(L2CValue *)(auStack_e0 + 0x20));
		lib::L2CValue::L2CValue(&LStack_70,0.0);
		uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)auStack_e0);
		lib::L2CValue::~L2CValue(&LStack_70);
		if ((uVar5 & 1) != 0) {
			lib::L2CValue::L2CValue(&LStack_70,1.0);
			uVar5 = lib::L2CValue::operator<((L2CValue *)auStack_e0,(L2CValue *)&LStack_70);
			lib::L2CValue::~L2CValue(&LStack_70);
			if ((uVar5 & 1) != 0) {
				lib::L2CValue::L2CValue(&LStack_70,1.0);
				lib::L2CValue::operator-(&LStack_70,(L2CValue *)auStack_e0);
				lib::L2CValue::~L2CValue(&LStack_70);
				lib::L2CValue::operator=((L2CValue *)auStack_e0,(L2CValue *)&LStack_f0);
				lib::L2CValue::~L2CValue(&LStack_f0);
				uVar5 = lib::L2CValue::operator<((L2CValue *)auStack_e0,(L2CValue *)(ulong)param_5);
				if ((uVar5 & 1) != 0) {
					lib::L2CValue::operator=((L2CValue *)auStack_e0,(L2CValue *)(ulong)param_5);
				}
				lib::L2CValue::operator*((L2CValue *)auStack_e0,(L2CValue *)(ulong)param_6);
				lib::L2CValue::operator*(&LStack_b0,(L2CValue *)&LStack_f0);
				lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
				lib::L2CValue::~L2CValue(&LStack_70);
				lib::L2CValue::~L2CValue(&LStack_f0);
			}
		}
		pLVar7 = (L2CValue *)auStack_e0;
	}
	lib::L2CValue::~L2CValue(pLVar7);
	lib::L2CValue::L2CValue(&LStack_f0,0xc8cc9db76);
	lib::L2CValue::L2CValue(&LStack_100,0);
	uVar5 = lib::L2CValue::as_integer(&LStack_f0);
	uVar6 = lib::L2CValue::as_integer(&LStack_100);
	fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar5,uVar6);
	lib::L2CValue::L2CValue(&LStack_70,fVar9);
	lib::L2CValue::operator/(&LStack_70,pLVar8);
	lib::L2CValue::~L2CValue(&LStack_70);
	lib::L2CValue::~L2CValue(&LStack_100);
	lib::L2CValue::~L2CValue(&LStack_f0);
	lib::L2CValue::L2CValue(&LStack_70,0.0);
	uVar5 = lib::L2CValue::operator==((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)&LStack_70);
	lib::L2CValue::~L2CValue(&LStack_70);
	if ((uVar5 & 1) == 0) {
		lib::L2CValue::L2CValue(&LStack_70,0.0);
		uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)&LStack_b0);
		lib::L2CValue::~L2CValue(&LStack_70);
		if ((uVar5 & 1) == 0) {
			lib::L2CValue::operator+((L2CValue *)this_00,(L2CValue *)&LStack_b0);
			uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)(auStack_e0 + 0x20));
			lib::L2CValue::~L2CValue(&LStack_70);
			if ((uVar5 & 1) != 0) {
				lib::L2CValue::operator=(&LStack_b0,(L2CValue *)auStack_e0);
				lib::L2CValue::operator+((L2CValue *)this_00,(L2CValue *)&LStack_b0);
				uVar5 = lib::L2CValue::operator<((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)&LStack_70);
				lib::L2CValue::~L2CValue(&LStack_70);
				if ((uVar5 & 1) != 0) {
					lib::L2CValue::operator-((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)this_00);
					lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
				}
			}
		}
		else {
			lib::L2CValue::operator+((L2CValue *)this_00,(L2CValue *)&LStack_b0);
			uVar5 = lib::L2CValue::operator<((L2CValue *)(auStack_e0 + 0x20),(L2CValue *)&LStack_70);
			lib::L2CValue::~L2CValue(&LStack_70);
			if ((uVar5 & 1) != 0) {
				lib::L2CValue::operator-((L2CValue *)auStack_e0);
				lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
				lib::L2CValue::~L2CValue(&LStack_70);
				lib::L2CValue::operator+((L2CValue *)this_00,(L2CValue *)&LStack_b0);
				uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)(auStack_e0 + 0x20));
				lib::L2CValue::~L2CValue(&LStack_70);
				if ((uVar5 & 1) != 0) {
					lstack_b0 = this_00 - (auStack_e0 + 0x20);
				}
			}
		}
	}
	else {
		lib::L2CAgent::math_abs((L2CAgent *)auStack_e0,extraout_x1);
		lib::L2CAgent::math_abs(this_00,extraout_x1_00);
		uVar5 = lib::L2CValue::operator<(&LStack_f0,(L2CValue *)&LStack_70);
		lib::L2CValue::~L2CValue(&LStack_f0);
		lib::L2CValue::~L2CValue(&LStack_70);
		if ((uVar5 & 1) == 0) {
			lib::L2CValue::L2CValue(&LStack_70,0.0);
			uVar5 = lib::L2CValue::operator<(&LStack_70,(L2CValue *)this_00);
			lib::L2CValue::~L2CValue(&LStack_70);
			if ((uVar5 & 1) == 0) {
				lib::L2CValue::operator=(&LStack_b0,(L2CValue *)auStack_e0);
			}
			lib::L2CValue::operator-((L2CValue *)auStack_e0);
			lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
		}
		else {
			lib::L2CValue::operator-((L2CValue *)this_00);
			lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_70);
		}
	}
	WorkModule::set_float(fighter.module_accessor, this_00+LStack_b0, param_7.get_i32());
}
*/