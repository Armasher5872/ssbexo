use super::*;

//Status Pre Attack Air, used to permit momentum transfer for aerials
#[skyline::hook(replace = L2CFighterCommon_status_air_lasso_main)]
unsafe extern "C" fn status_air_lasso_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    bVar1 = app::lua_bind::CancelModule__is_enable_cancel_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar1 & 1));
    lib::L2CValue::L2CValue(&LStack_40,true);
    uVar4 = lib::L2CValue::operator==(&LStack_50,(L2CValue *)&LStack_40);
    lib::L2CValue::~L2CValue(&LStack_40);
    if ((uVar4 & 1) == 0) {
        lib::L2CValue::~L2CValue(&LStack_50);
        LAB_7100064b2c:
        pLVar5 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
        lib::L2CValue::L2CValue(&LStack_40,SITUATION_KIND_GROUND);
        uVar4 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack_40);
        lib::L2CValue::~L2CValue(&LStack_40);
        if ((uVar4 & 1) != 0) {
            lib::L2CValue::L2CValue(&LStack_50,FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
            iVar3 = lib::L2CValue::as_integer(&LStack_50);
            bVar1 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar3);
            lib::L2CValue::L2CValue(&LStack_40,(bool)(bVar1 & 1));
            bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_40);
            lib::L2CValue::~L2CValue(&LStack_40);
            lib::L2CValue::~L2CValue(&LStack_50);
            if (bVar2) {
                lib::L2CValue::L2CValue(&LStack_90,_FIGHTER_STATUS_KIND_AIR_LASSO_LANDING);
                lib::L2CValue::L2CValue(&LStack_a0,false);
                L2CFighterBase::change_status((L2CFighterBase *)this,(L2CValue *)&LStack_90,(L2CValue *)&LStack_a0);
                lib::L2CValue::~L2CValue(&LStack_a0);
                pLVar5 = &LStack_90;
                goto LAB_7100064ca4;
            }
        }
        sub_transition_group_check_air_landing(this);
        bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_40);
        lib::L2CValue::~L2CValue(&LStack_40);
        if (!bVar2) {
            bVar1 = app::lua_bind::MotionModule__is_end_impl(this->moduleAccessor);
            lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar1 & 1));
            lib::L2CValue::L2CValue(&LStack_40,true);
            uVar4 = lib::L2CValue::operator==(&LStack_50,(L2CValue *)&LStack_40);
            lib::L2CValue::~L2CValue(&LStack_40);
            lib::L2CValue::~L2CValue(&LStack_50);
            if ((uVar4 & 1) == 0) {
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
                iVar3 = lib::L2CValue::as_integer(&LStack_50);
                bVar1 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar3);
                lib::L2CValue::L2CValue(&LStack_40,(bool)(bVar1 & 1));
                bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_40);
                if (bVar2) {
                    sub_fighter_general_term_is_cliff_check_pos(this);
                    bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
                    lib::L2CValue::~L2CValue(&LStack_60);
                    lib::L2CValue::~L2CValue(&LStack_40);
                    lib::L2CValue::~L2CValue(&LStack_50);
                    if (bVar2) {
                        lib::L2CValue::L2CValue(&LStack_d0,FIGHTER_STATUS_KIND_AIR_LASSO_REACH);
                        lib::L2CValue::L2CValue(&LStack_e0,false);
                        L2CFighterBase::change_status((L2CFighterBase *)this,(L2CValue *)&LStack_d0,(L2CValue *)&LStack_e0);
                        lib::L2CValue::~L2CValue(&LStack_e0);
                        pLVar5 = &LStack_d0;
                        goto LAB_7100064ca4;
                    }
                }
                else {
                    lib::L2CValue::~L2CValue(&LStack_40);
                    lib::L2CValue::~L2CValue(&LStack_50);
                }
                iVar3 = 0;
                goto LAB_7100064cb0;
            }
            lib::L2CValue::L2CValue(&LStack_b0,FIGHTER_STATUS_KIND_FALL);
            lib::L2CValue::L2CValue(&LStack_c0,false);
            L2CFighterBase::change_status((L2CFighterBase *)this,(L2CValue *)&LStack_b0,(L2CValue *)&LStack_c0);
            lib::L2CValue::~L2CValue(&LStack_c0);
            pLVar5 = &LStack_b0;
            LAB_7100064ca4:
            lib::L2CValue::~L2CValue(pLVar5);
        }
    }
    else {
        lib::L2CValue::L2CValue(&LStack_70,false);
        sub_wait_ground_check_common(this,(int)&stack0xfffffffffffffff0 - 0x60);
        lib::L2CValue::L2CValue(&LStack_40,false);
        uVar4 = lib::L2CValue::operator==(&LStack_60,(L2CValue *)&LStack_40);
        lib::L2CValue::~L2CValue(&LStack_40);
        if ((uVar4 & 1) == 0) {
            lib::L2CValue::~L2CValue(&LStack_60);
            lib::L2CValue::~L2CValue(&LStack_70);
            pLVar5 = &LStack_50;
            goto LAB_7100064ca4;
        }
        sub_air_check_fall_common(this);
        lib::L2CValue::L2CValue(&LStack_40,false);
        uVar4 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_40);
        lib::L2CValue::~L2CValue(&LStack_40);
        lib::L2CValue::~L2CValue(&LStack_80);
        lib::L2CValue::~L2CValue(&LStack_60);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_50);
        if ((uVar4 & 1) != 0) goto LAB_7100064b2c;
    }
    iVar3 = 1;
    LAB_7100064cb0:
    lib::L2CValue::L2CValue(in_x8,iVar3);
    return;
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_pre_attackair);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}