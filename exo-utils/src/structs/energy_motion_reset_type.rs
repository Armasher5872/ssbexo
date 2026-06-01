//Credited to WuBoyTH and BluJay, the following functions handle kinetics, ultimately related to sliding normals
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyMotionResetType {
    GroundTransLoop = 0x0,
    GroundTransLoopGekikara,
    GroundTrans,
    GroundTransIgnoreNorm,
    AirTrans,
    AirTransAngle,
    AirTransY,
    AirTransAngleSuperJumpPunch,
    AirTrans2nd,
    CliffTransIntp,
    CliffTrans,
    CliffTransGround,
    LadderMove,
    LadderTrans
}

impl EnergyMotionResetType {
    pub fn is_ground(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, GroundTransLoop | GroundTransLoopGekikara | GroundTrans | GroundTransIgnoreNorm)
    }

    pub fn is_air(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, AirTrans | AirTransAngle | AirTransY | AirTransAngleSuperJumpPunch | AirTrans2nd)
    }

    pub fn is_cliff(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, CliffTransIntp | CliffTrans | CliffTransGround)
    }

    pub fn is_ladder(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, LadderMove | LadderTrans)
    }

    pub fn is_2nd(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, AirTrans2nd)
    }
}