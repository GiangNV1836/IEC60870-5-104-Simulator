use serde::{Deserialize, Serialize};

/// IEC 60870-5-101/104 ASDU Type Identifiers (commonly used subset).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AsduTypeId {
    // ---- Monitor direction (slave -> master) ----
    /// Single-point information (Type 1)
    MSpNa1 = 1,
    /// Single-point with CP24Time2a (Type 2)
    MSpTa1 = 2,
    /// Single-point with CP56Time2a (Type 30)
    MSpTb1 = 30,
    /// Double-point information (Type 3)
    MDpNa1 = 3,
    /// Double-point with CP24Time2a (Type 4)
    MDpTa1 = 4,
    /// Double-point with CP56Time2a (Type 31)
    MDpTb1 = 31,
    /// Step position (Type 5)
    MStNa1 = 5,
    /// Step position with CP24Time2a (Type 6)
    MStTa1 = 6,
    /// Step position with CP56Time2a (Type 32)
    MStTb1 = 32,
    /// Bitstring of 32 bit (Type 7)
    MBoNa1 = 7,
    /// Bitstring with CP56Time2a (Type 33)
    MBoTb1 = 33,
    /// Measured value, normalized (Type 9)
    MMeNa1 = 9,
    /// Measured value, normalized with CP24Time2a (Type 10)
    MMeTa1 = 10,
    /// Measured value, normalized with CP56Time2a (Type 34)
    MMeTd1 = 34,
    /// Measured value, normalized without quality descriptor (Type 21)
    MMeNd1 = 21,
    /// Measured value, scaled (Type 11)
    MMeNb1 = 11,
    /// Measured value, scaled with CP24Time2a (Type 12)
    MMeTb1 = 12,
    /// Measured value, scaled with CP56Time2a (Type 35)
    MMeTe1 = 35,
    /// Measured value, short floating point (Type 13)
    MMeNc1 = 13,
    /// Measured value, short float with CP24Time2a (Type 14)
    MMeTc1 = 14,
    /// Measured value, short float with CP56Time2a (Type 36)
    MMeTf1 = 36,
    /// Integrated totals (Type 15)
    MItNa1 = 15,
    /// Integrated totals with CP56Time2a (Type 37)
    MItTb1 = 37,

    // ---- Control direction (master -> slave) ----
    /// Single command (Type 45)
    CScNa1 = 45,
    /// Double command (Type 46)
    CDcNa1 = 46,
    /// Step command (Type 47)
    CRcNa1 = 47,
    /// Set-point, normalized (Type 48)
    CSeNa1 = 48,
    /// Set-point, scaled (Type 49)
    CSeNb1 = 49,
    /// Set-point, short floating point (Type 50)
    CSeNc1 = 50,
    /// Bitstring of 32 bit command (Type 51)
    CBoNa1 = 51,
    /// Single command with CP56Time2a (Type 58)
    CScTa1 = 58,
    /// Double command with CP56Time2a (Type 59)
    CDcTa1 = 59,
    /// Step command with CP56Time2a (Type 60)
    CRcTa1 = 60,
    /// Set-point, normalized with CP56Time2a (Type 61)
    CSeTa1 = 61,
    /// Set-point, scaled with CP56Time2a (Type 62)
    CSeTb1 = 62,
    /// Set-point, short float with CP56Time2a (Type 63)
    CSeTc1 = 63,
    /// Bitstring command with CP56Time2a (Type 64)
    CBoTa1 = 64,

    // ---- System commands ----
    /// Interrogation command (Type 100)
    CIcNa1 = 100,
    /// Counter interrogation command (Type 101)
    CCiNa1 = 101,
    /// Clock synchronization command (Type 103)
    CCsNa1 = 103,
}

impl AsduTypeId {
    /// Short display name for ASDU type.
    pub fn name(&self) -> &'static str {
        match self {
            Self::MSpNa1 => "M_SP_NA_1",
            Self::MSpTa1 => "M_SP_TA_1",
            Self::MSpTb1 => "M_SP_TB_1",
            Self::MDpNa1 => "M_DP_NA_1",
            Self::MDpTa1 => "M_DP_TA_1",
            Self::MDpTb1 => "M_DP_TB_1",
            Self::MStNa1 => "M_ST_NA_1",
            Self::MStTa1 => "M_ST_TA_1",
            Self::MStTb1 => "M_ST_TB_1",
            Self::MBoNa1 => "M_BO_NA_1",
            Self::MBoTb1 => "M_BO_TB_1",
            Self::MMeNa1 => "M_ME_NA_1",
            Self::MMeTa1 => "M_ME_TA_1",
            Self::MMeTd1 => "M_ME_TD_1",
            Self::MMeNd1 => "M_ME_ND_1",
            Self::MMeNb1 => "M_ME_NB_1",
            Self::MMeTb1 => "M_ME_TB_1",
            Self::MMeTe1 => "M_ME_TE_1",
            Self::MMeNc1 => "M_ME_NC_1",
            Self::MMeTc1 => "M_ME_TC_1",
            Self::MMeTf1 => "M_ME_TF_1",
            Self::MItNa1 => "M_IT_NA_1",
            Self::MItTb1 => "M_IT_TB_1",
            Self::CScNa1 => "C_SC_NA_1",
            Self::CDcNa1 => "C_DC_NA_1",
            Self::CRcNa1 => "C_RC_NA_1",
            Self::CSeNa1 => "C_SE_NA_1",
            Self::CSeNb1 => "C_SE_NB_1",
            Self::CSeNc1 => "C_SE_NC_1",
            Self::CBoNa1 => "C_BO_NA_1",
            Self::CScTa1 => "C_SC_TA_1",
            Self::CDcTa1 => "C_DC_TA_1",
            Self::CRcTa1 => "C_RC_TA_1",
            Self::CSeTa1 => "C_SE_TA_1",
            Self::CSeTb1 => "C_SE_TB_1",
            Self::CSeTc1 => "C_SE_TC_1",
            Self::CBoTa1 => "C_BO_TA_1",
            Self::CIcNa1 => "C_IC_NA_1",
            Self::CCiNa1 => "C_CI_NA_1",
            Self::CCsNa1 => "C_CS_NA_1",
        }
    }

    /// Human-readable Chinese description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::MSpNa1 | Self::MSpTa1 | Self::MSpTb1 => "单点信息",
            Self::MDpNa1 | Self::MDpTa1 | Self::MDpTb1 => "双点信息",
            Self::MStNa1 | Self::MStTa1 | Self::MStTb1 => "步位置信息",
            Self::MBoNa1 | Self::MBoTb1 => "32位串",
            Self::MMeNa1 | Self::MMeTa1 | Self::MMeTd1 => "归一化测量值",
            Self::MMeNd1 => "归一化测量值(无品质)",
            Self::MMeNb1 | Self::MMeTb1 | Self::MMeTe1 => "标度化测量值",
            Self::MMeNc1 | Self::MMeTc1 | Self::MMeTf1 => "短浮点测量值",
            Self::MItNa1 | Self::MItTb1 => "累计量",
            Self::CScNa1 | Self::CScTa1 => "单点命令",
            Self::CDcNa1 | Self::CDcTa1 => "双点命令",
            Self::CRcNa1 | Self::CRcTa1 => "步调节命令",
            Self::CSeNa1 | Self::CSeTa1 => "归一化设定值",
            Self::CSeNb1 | Self::CSeTb1 => "标度化设定值",
            Self::CSeNc1 | Self::CSeTc1 => "短浮点设定值",
            Self::CBoNa1 | Self::CBoTa1 => "位串命令",
            Self::CIcNa1 => "总召唤",
            Self::CCiNa1 => "累计量召唤",
            Self::CCsNa1 => "时钟同步",
        }
    }

    /// Get the data category this ASDU type belongs to.
    pub fn category(&self) -> DataCategory {
        match self {
            Self::MSpNa1 | Self::MSpTa1 | Self::MSpTb1 => DataCategory::SinglePoint,
            Self::MDpNa1 | Self::MDpTa1 | Self::MDpTb1 => DataCategory::DoublePoint,
            Self::MStNa1 | Self::MStTa1 | Self::MStTb1 => DataCategory::StepPosition,
            Self::MBoNa1 | Self::MBoTb1 => DataCategory::Bitstring,
            Self::MMeNa1 | Self::MMeTa1 | Self::MMeTd1 | Self::MMeNd1 => DataCategory::NormalizedMeasured,
            Self::MMeNb1 | Self::MMeTb1 | Self::MMeTe1 => DataCategory::ScaledMeasured,
            Self::MMeNc1 | Self::MMeTc1 | Self::MMeTf1 => DataCategory::FloatMeasured,
            Self::MItNa1 | Self::MItTb1 => DataCategory::IntegratedTotals,
            Self::CScNa1 | Self::CScTa1 => DataCategory::SingleCommand,
            Self::CDcNa1 | Self::CDcTa1 => DataCategory::DoubleCommand,
            Self::CRcNa1 | Self::CRcTa1 => DataCategory::StepCommand,
            Self::CSeNa1 | Self::CSeTa1 => DataCategory::NormalizedSetpoint,
            Self::CSeNb1 | Self::CSeTb1 => DataCategory::ScaledSetpoint,
            Self::CSeNc1 | Self::CSeTc1 => DataCategory::FloatSetpoint,
            Self::CBoNa1 | Self::CBoTa1 => DataCategory::BitstringCommand,
            Self::CIcNa1 | Self::CCiNa1 | Self::CCsNa1 => DataCategory::System,
        }
    }

    /// Whether this is a control-direction (command / set-point) type.
    pub fn is_control(&self) -> bool {
        matches!(
            self,
            Self::CScNa1
                | Self::CDcNa1
                | Self::CRcNa1
                | Self::CSeNa1
                | Self::CSeNb1
                | Self::CSeNc1
                | Self::CBoNa1
                | Self::CScTa1
                | Self::CDcTa1
                | Self::CRcTa1
                | Self::CSeTa1
                | Self::CSeTb1
                | Self::CSeTc1
                | Self::CBoTa1
        )
    }

    /// 该控制类型允许映射到的监视方向分类(控制与监视方向独立,同族即可:
    /// 45/58→SP(1/30), 46/59→DP(3/31), 47/60→ST(5/32), 51/64→BO(7/33),
    /// 48/61→ME_NA(9/34/21), 49/62→ME_NB(11/35), 50/63→ME_NC(13/36))。
    /// 非控制类型返回空切片。
    pub fn allowed_target_categories(&self) -> &'static [DataCategory] {
        match self.category() {
            DataCategory::SingleCommand => &[DataCategory::SinglePoint],
            DataCategory::DoubleCommand => &[DataCategory::DoublePoint],
            DataCategory::StepCommand => &[DataCategory::StepPosition],
            DataCategory::BitstringCommand => &[DataCategory::Bitstring],
            DataCategory::NormalizedSetpoint => &[DataCategory::NormalizedMeasured],
            DataCategory::ScaledSetpoint => &[DataCategory::ScaledMeasured],
            DataCategory::FloatSetpoint => &[DataCategory::FloatMeasured],
            _ => &[],
        }
    }

    /// Whether this ASDU type carries a timestamp (CP56Time2a or CP24Time2a).
    pub fn is_timestamped(&self) -> bool {
        if self.is_cp24() {
            return true;
        }
        matches!(
            self,
            Self::MSpTb1
                | Self::MDpTb1
                | Self::MStTb1
                | Self::MBoTb1
                | Self::MMeTd1
                | Self::MMeTe1
                | Self::MMeTf1
                | Self::MItTb1
                | Self::CScTa1
                | Self::CDcTa1
                | Self::CRcTa1
                | Self::CSeTa1
                | Self::CSeTb1
                | Self::CSeTc1
                | Self::CBoTa1
        )
    }

    /// Whether this ASDU type carries a 3-byte CP24Time2a (short) timestamp.
    /// These are the TA monitor variants (Type 2/4/6/10/12/14); they transmit
    /// as their own TypeID and never participate in NA→TB derivation.
    pub fn is_cp24(&self) -> bool {
        matches!(
            self,
            Self::MSpTa1
                | Self::MDpTa1
                | Self::MStTa1
                | Self::MMeTa1
                | Self::MMeTb1
                | Self::MMeTc1
        )
    }

    /// Map an NA (no timestamp) type to its CP56Time2a-bearing counterpart.
    /// Covers monitor types (M_SP_NA_1 → M_SP_TB_1) and control types
    /// (C_SC_NA_1 → C_SC_TA_1). Returns `None` for system types and for
    /// types that are already timestamped.
    pub fn timestamped_variant(&self) -> Option<AsduTypeId> {
        match self {
            Self::MSpNa1 => Some(Self::MSpTb1),
            Self::MDpNa1 => Some(Self::MDpTb1),
            Self::MStNa1 => Some(Self::MStTb1),
            Self::MBoNa1 => Some(Self::MBoTb1),
            Self::MMeNa1 => Some(Self::MMeTd1),
            Self::MMeNb1 => Some(Self::MMeTe1),
            Self::MMeNc1 => Some(Self::MMeTf1),
            Self::MItNa1 => Some(Self::MItTb1),
            Self::CScNa1 => Some(Self::CScTa1),
            Self::CDcNa1 => Some(Self::CDcTa1),
            Self::CRcNa1 => Some(Self::CRcTa1),
            Self::CSeNa1 => Some(Self::CSeTa1),
            Self::CSeNb1 => Some(Self::CSeTb1),
            Self::CSeNc1 => Some(Self::CSeTc1),
            Self::CBoNa1 => Some(Self::CBoTa1),
            _ => None,
        }
    }

    /// Inverse of [`Self::timestamped_variant`]: strip the timestamp from a TB/TA
    /// type back to its NA peer. Identity for already-untimestamped types.
    /// CP24 monitor variants (M_*_TA / M_ME_TB / M_ME_TC) also map to their NA peer.
    pub fn untimestamped_variant(&self) -> AsduTypeId {
        match self {
            Self::MSpTa1 => Self::MSpNa1,
            Self::MDpTa1 => Self::MDpNa1,
            Self::MStTa1 => Self::MStNa1,
            Self::MMeTa1 => Self::MMeNa1,
            Self::MMeTb1 => Self::MMeNb1,
            Self::MMeTc1 => Self::MMeNc1,
            Self::MSpTb1 => Self::MSpNa1,
            Self::MDpTb1 => Self::MDpNa1,
            Self::MStTb1 => Self::MStNa1,
            Self::MBoTb1 => Self::MBoNa1,
            Self::MMeTd1 => Self::MMeNa1,
            Self::MMeTe1 => Self::MMeNb1,
            Self::MMeTf1 => Self::MMeNc1,
            Self::MItTb1 => Self::MItNa1,
            Self::CScTa1 => Self::CScNa1,
            Self::CDcTa1 => Self::CDcNa1,
            Self::CRcTa1 => Self::CRcNa1,
            Self::CSeTa1 => Self::CSeNa1,
            Self::CSeTb1 => Self::CSeNb1,
            Self::CSeTc1 => Self::CSeNc1,
            Self::CBoTa1 => Self::CBoNa1,
            other => *other,
        }
    }

    /// Parse from type ID integer.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::MSpNa1),
            2 => Some(Self::MSpTa1),
            3 => Some(Self::MDpNa1),
            4 => Some(Self::MDpTa1),
            5 => Some(Self::MStNa1),
            6 => Some(Self::MStTa1),
            7 => Some(Self::MBoNa1),
            9 => Some(Self::MMeNa1),
            10 => Some(Self::MMeTa1),
            21 => Some(Self::MMeNd1),
            11 => Some(Self::MMeNb1),
            12 => Some(Self::MMeTb1),
            13 => Some(Self::MMeNc1),
            14 => Some(Self::MMeTc1),
            15 => Some(Self::MItNa1),
            30 => Some(Self::MSpTb1),
            31 => Some(Self::MDpTb1),
            32 => Some(Self::MStTb1),
            33 => Some(Self::MBoTb1),
            34 => Some(Self::MMeTd1),
            35 => Some(Self::MMeTe1),
            36 => Some(Self::MMeTf1),
            37 => Some(Self::MItTb1),
            45 => Some(Self::CScNa1),
            46 => Some(Self::CDcNa1),
            47 => Some(Self::CRcNa1),
            48 => Some(Self::CSeNa1),
            49 => Some(Self::CSeNb1),
            50 => Some(Self::CSeNc1),
            51 => Some(Self::CBoNa1),
            58 => Some(Self::CScTa1),
            59 => Some(Self::CDcTa1),
            60 => Some(Self::CRcTa1),
            61 => Some(Self::CSeTa1),
            62 => Some(Self::CSeTb1),
            63 => Some(Self::CSeTc1),
            64 => Some(Self::CBoTa1),
            100 => Some(Self::CIcNa1),
            101 => Some(Self::CCiNa1),
            103 => Some(Self::CCsNa1),
            _ => None,
        }
    }
}

/// Data categories for grouping ASDU types in the UI tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataCategory {
    /// M_SP_* (single-point information)
    SinglePoint,
    /// M_DP_* (double-point information)
    DoublePoint,
    /// M_ST_* (step position)
    StepPosition,
    /// M_BO_* (bitstring)
    Bitstring,
    /// M_ME_NA/TD (normalized measured value)
    NormalizedMeasured,
    /// M_ME_NB/TE (scaled measured value)
    ScaledMeasured,
    /// M_ME_NC/TF (short floating point measured value)
    FloatMeasured,
    /// M_IT_* (integrated totals / counters)
    IntegratedTotals,
    /// C_SC_NA/TA (single command, control direction)
    SingleCommand,
    /// C_DC_NA/TA (double command, control direction)
    DoubleCommand,
    /// C_RC_NA/TA (regulating step command, control direction)
    StepCommand,
    /// C_BO_NA/TA (bitstring command, control direction)
    BitstringCommand,
    /// C_SE_NA/TA (set-point normalized, control direction)
    NormalizedSetpoint,
    /// C_SE_NB/TB (set-point scaled, control direction)
    ScaledSetpoint,
    /// C_SE_NC/TC (set-point short float, control direction)
    FloatSetpoint,
    /// System commands (GI, Counter Read, Clock Sync)
    System,
}

impl DataCategory {
    /// 测量类(QDS 中 OV/溢出位有意义的分类):归一化/标度化/浮点。
    /// SP/DP/ST/BO/IT 的 OV 不适用。
    pub fn is_measured(&self) -> bool {
        matches!(
            self,
            Self::NormalizedMeasured | Self::ScaledMeasured | Self::FloatMeasured
        )
    }

    /// Short display name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::SinglePoint => "单点 (SP)",
            Self::DoublePoint => "双点 (DP)",
            Self::StepPosition => "步位置 (ST)",
            Self::Bitstring => "位串 (BO)",
            Self::NormalizedMeasured => "归一化 (ME_NA)",
            Self::ScaledMeasured => "标度化 (ME_NB)",
            Self::FloatMeasured => "浮点 (ME_NC)",
            Self::IntegratedTotals => "累计量 (IT)",
            Self::SingleCommand => "单点命令 (C_SC)",
            Self::DoubleCommand => "双点命令 (C_DC)",
            Self::StepCommand => "步调节命令 (C_RC)",
            Self::BitstringCommand => "位串命令 (C_BO)",
            Self::NormalizedSetpoint => "归一化设定值 (C_SE_NA)",
            Self::ScaledSetpoint => "标度化设定值 (C_SE_NB)",
            Self::FloatSetpoint => "浮点设定值 (C_SE_NC)",
            Self::System => "系统命令",
        }
    }

    /// 稳定标识串(serde snake_case 变体名)。跨 IPC 传给前端做匹配/本地化,
    /// 替代旧的中文 `name()` 键——UI 展示文案由前端字典按此键翻译。
    pub fn key(&self) -> &'static str {
        match self {
            Self::SinglePoint => "single_point",
            Self::DoublePoint => "double_point",
            Self::StepPosition => "step_position",
            Self::Bitstring => "bitstring",
            Self::NormalizedMeasured => "normalized_measured",
            Self::ScaledMeasured => "scaled_measured",
            Self::FloatMeasured => "float_measured",
            Self::IntegratedTotals => "integrated_totals",
            Self::SingleCommand => "single_command",
            Self::DoubleCommand => "double_command",
            Self::StepCommand => "step_command",
            Self::BitstringCommand => "bitstring_command",
            Self::NormalizedSetpoint => "normalized_setpoint",
            Self::ScaledSetpoint => "scaled_setpoint",
            Self::FloatSetpoint => "float_setpoint",
            Self::System => "system",
        }
    }

    /// Whether this is a control-direction (command / set-point) category.
    pub fn is_control(&self) -> bool {
        matches!(
            self,
            Self::SingleCommand
                | Self::DoubleCommand
                | Self::StepCommand
                | Self::BitstringCommand
                | Self::NormalizedSetpoint
                | Self::ScaledSetpoint
                | Self::FloatSetpoint
        )
    }

    /// 控制方向分类在旧「同 CA+IOA 自动映射」下对应的监视方向分类。
    /// 非控制分类返回 None。
    pub fn auto_map_monitor_category(&self) -> Option<DataCategory> {
        match self {
            Self::SingleCommand => Some(Self::SinglePoint),
            Self::DoubleCommand => Some(Self::DoublePoint),
            Self::StepCommand => Some(Self::StepPosition),
            Self::BitstringCommand => Some(Self::Bitstring),
            Self::NormalizedSetpoint => Some(Self::NormalizedMeasured),
            Self::ScaledSetpoint => Some(Self::ScaledMeasured),
            Self::FloatSetpoint => Some(Self::FloatMeasured),
            _ => None,
        }
    }

    /// All monitor-direction categories (for tree display).
    pub fn monitor_categories() -> &'static [DataCategory] {
        &[
            Self::SinglePoint,
            Self::DoublePoint,
            Self::StepPosition,
            Self::Bitstring,
            Self::NormalizedMeasured,
            Self::ScaledMeasured,
            Self::FloatMeasured,
            Self::IntegratedTotals,
        ]
    }

    /// All control-direction categories (for tree display).
    pub fn control_categories() -> &'static [DataCategory] {
        &[
            Self::SingleCommand,
            Self::DoubleCommand,
            Self::StepCommand,
            Self::BitstringCommand,
            Self::NormalizedSetpoint,
            Self::ScaledSetpoint,
            Self::FloatSetpoint,
        ]
    }
}

/// Quality descriptor flags per IEC 60870-5-101.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct QualityFlags {
    /// Overflow
    pub ov: bool,
    /// Blocked
    pub bl: bool,
    /// Substituted
    pub sb: bool,
    /// Not topical
    pub nt: bool,
    /// Invalid
    pub iv: bool,
}

impl QualityFlags {
    pub fn good() -> Self {
        Self::default()
    }

    pub fn invalid() -> Self {
        Self { iv: true, ..Default::default() }
    }

    /// 公共品质位 BL/SB/NT/IV 组装到 QDS/SIQ/DIQ 的高 4 位
    /// (`BL=0x10 / SB=0x20 / NT=0x40 / IV=0x80`)。不含 OV。
    pub fn upper_bits(&self) -> u8 {
        (if self.bl { 0x10 } else { 0 })
            | (if self.sb { 0x20 } else { 0 })
            | (if self.nt { 0x40 } else { 0 })
            | (if self.iv { 0x80 } else { 0 })
    }

    /// 测量类(M_ME_*)的完整 QDS 字节:高 4 位 + OV(bit1=0x01)。
    /// OV 仅对测量类有意义,SP/DP/IT 与 Step/Bitstring 不用此方法。
    pub fn qds_byte(&self) -> u8 {
        self.upper_bits() | (if self.ov { 0x01 } else { 0 })
    }
}

/// Cause of Transmission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CauseOfTransmission {
    Periodic = 1,
    Background = 2,
    Spontaneous = 3,
    Initialized = 4,
    Request = 5,
    Activation = 6,
    ActivationCon = 7,
    Deactivation = 8,
    DeactivationCon = 9,
    ActivationTermination = 10,
    Interrogated = 20,
    CounterInterrogated = 37,
}

impl CauseOfTransmission {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Periodic),
            2 => Some(Self::Background),
            3 => Some(Self::Spontaneous),
            4 => Some(Self::Initialized),
            5 => Some(Self::Request),
            6 => Some(Self::Activation),
            7 => Some(Self::ActivationCon),
            8 => Some(Self::Deactivation),
            9 => Some(Self::DeactivationCon),
            10 => Some(Self::ActivationTermination),
            20 => Some(Self::Interrogated),
            37 => Some(Self::CounterInterrogated),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Periodic => "周期",
            Self::Background => "背景",
            Self::Spontaneous => "突发",
            Self::Initialized => "初始化",
            Self::Request => "请求",
            Self::Activation => "激活",
            Self::ActivationCon => "激活确认",
            Self::Deactivation => "停止激活",
            Self::DeactivationCon => "停止确认",
            Self::ActivationTermination => "激活终止",
            Self::Interrogated => "总召唤",
            Self::CounterInterrogated => "累计量召唤",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asdu_type_from_u8() {
        assert_eq!(AsduTypeId::from_u8(1), Some(AsduTypeId::MSpNa1));
        assert_eq!(AsduTypeId::from_u8(13), Some(AsduTypeId::MMeNc1));
        assert_eq!(AsduTypeId::from_u8(100), Some(AsduTypeId::CIcNa1));
        assert_eq!(AsduTypeId::from_u8(255), None);
    }

    #[test]
    fn test_m_me_nd_1_metadata() {
        let nd = AsduTypeId::from_u8(21).expect("21 → MMeNd1");
        assert_eq!(nd, AsduTypeId::MMeNd1);
        assert_eq!(nd.name(), "M_ME_ND_1");
        assert_eq!(nd.category(), DataCategory::NormalizedMeasured);
        assert!(!nd.is_timestamped(), "ND 不带时标");
        assert_eq!(nd.timestamped_variant(), None, "ND 无带时标变体");
        assert_eq!(nd.untimestamped_variant(), AsduTypeId::MMeNd1, "untimestamped 恒等");
    }

    #[test]
    fn test_asdu_type_category() {
        assert_eq!(AsduTypeId::MSpNa1.category(), DataCategory::SinglePoint);
        assert_eq!(AsduTypeId::MSpTb1.category(), DataCategory::SinglePoint);
        assert_eq!(AsduTypeId::MMeNc1.category(), DataCategory::FloatMeasured);
        assert_eq!(AsduTypeId::MItNa1.category(), DataCategory::IntegratedTotals);
    }

    #[test]
    fn test_quality_flags_default() {
        let q = QualityFlags::good();
        assert!(!q.ov && !q.bl && !q.sb && !q.nt && !q.iv);
    }

    #[test]
    fn test_category_is_measured() {
        assert!(DataCategory::NormalizedMeasured.is_measured());
        assert!(DataCategory::ScaledMeasured.is_measured());
        assert!(DataCategory::FloatMeasured.is_measured());
        assert!(!DataCategory::SinglePoint.is_measured());
        assert!(!DataCategory::DoublePoint.is_measured());
        assert!(!DataCategory::StepPosition.is_measured());
        assert!(!DataCategory::Bitstring.is_measured());
        assert!(!DataCategory::IntegratedTotals.is_measured());
    }

    #[test]
    fn test_asdu_type_name() {
        assert_eq!(AsduTypeId::MSpNa1.name(), "M_SP_NA_1");
        assert_eq!(AsduTypeId::CIcNa1.name(), "C_IC_NA_1");
    }

    #[test]
    fn test_timestamped_variant_round_trip() {
        let na_types = [
            AsduTypeId::MSpNa1,
            AsduTypeId::MDpNa1,
            AsduTypeId::MStNa1,
            AsduTypeId::MBoNa1,
            AsduTypeId::MMeNa1,
            AsduTypeId::MMeNb1,
            AsduTypeId::MMeNc1,
            AsduTypeId::MItNa1,
        ];
        for na in na_types {
            let tb = na.timestamped_variant().expect("NA → TB");
            assert!(tb.is_timestamped(), "{:?} should be timestamped", tb);
            assert_eq!(tb.untimestamped_variant(), na);
        }
    }

    #[test]
    fn test_timestamped_variant_for_control_and_none_for_system() {
        assert_eq!(AsduTypeId::CScNa1.timestamped_variant(), Some(AsduTypeId::CScTa1));
        assert_eq!(AsduTypeId::CScTa1.untimestamped_variant(), AsduTypeId::CScNa1);
        assert_eq!(AsduTypeId::CIcNa1.timestamped_variant(), None);
        assert_eq!(AsduTypeId::MSpTb1.timestamped_variant(), None);
    }

    #[test]
    fn test_cp24_types_full_matrix() {
        // (TypeID, 枚举, NA 基类型, 分类)
        let cases = [
            (2u8, AsduTypeId::MSpTa1, AsduTypeId::MSpNa1, DataCategory::SinglePoint),
            (4, AsduTypeId::MDpTa1, AsduTypeId::MDpNa1, DataCategory::DoublePoint),
            (6, AsduTypeId::MStTa1, AsduTypeId::MStNa1, DataCategory::StepPosition),
            (10, AsduTypeId::MMeTa1, AsduTypeId::MMeNa1, DataCategory::NormalizedMeasured),
            (12, AsduTypeId::MMeTb1, AsduTypeId::MMeNb1, DataCategory::ScaledMeasured),
            (14, AsduTypeId::MMeTc1, AsduTypeId::MMeNc1, DataCategory::FloatMeasured),
        ];
        for (id, ta, na, cat) in cases {
            assert_eq!(AsduTypeId::from_u8(id), Some(ta), "from_u8({})", id);
            assert_eq!(ta as u8, id);
            assert!(ta.is_cp24(), "{:?} is CP24", ta);
            assert!(ta.is_timestamped(), "{:?} carries a timestamp", ta);
            assert!(!ta.is_control());
            assert_eq!(ta.category(), cat);
            assert_eq!(ta.untimestamped_variant(), na);
            // TA 不参与 NA→TB 派生:自身没有 timestamped_variant。
            assert_eq!(ta.timestamped_variant(), None, "{:?} has no derived variant", ta);
            // NA 的 CP56 派生目标保持 TB,不被 CP24 变体劫持。
            assert!(!na.timestamped_variant().unwrap().is_cp24());
        }
        // 非 CP24 类型不误报。
        assert!(!AsduTypeId::MSpNa1.is_cp24());
        assert!(!AsduTypeId::MSpTb1.is_cp24());
        assert!(!AsduTypeId::CScTa1.is_cp24());
    }

    #[test]
    fn test_cp24_type_names() {
        assert_eq!(AsduTypeId::MSpTa1.name(), "M_SP_TA_1");
        assert_eq!(AsduTypeId::MDpTa1.name(), "M_DP_TA_1");
        assert_eq!(AsduTypeId::MStTa1.name(), "M_ST_TA_1");
        assert_eq!(AsduTypeId::MMeTa1.name(), "M_ME_TA_1");
        assert_eq!(AsduTypeId::MMeTb1.name(), "M_ME_TB_1");
        assert_eq!(AsduTypeId::MMeTc1.name(), "M_ME_TC_1");
    }
}
