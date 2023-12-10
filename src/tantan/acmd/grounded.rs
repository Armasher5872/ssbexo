use super::*;

unsafe extern "C" fn ssbexo_tantan_hurtbox_disable_acmd(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("tantan")
    .game_acmd("0x172240c033", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("0x1778b4545c", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("0x18bc531aa8", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_aircatchhang", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_aircatchhit", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_aircatchloop", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_aircatchpose", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartl2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartl3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartlb2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartlb3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartr2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartr3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartrb2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attacklongstartrb3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartl2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartl3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartlb1", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartlb2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartlb3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartr2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartr3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartrb1", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartrb2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_attackshortstartrb3", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_catchpull", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_damageair2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_specialairhi2", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_specialhilong", ssbexo_tantan_hurtbox_disable_acmd)
    .game_acmd("game_specialhishort", ssbexo_tantan_hurtbox_disable_acmd)
    .install()
    ;
}