// 共享 ASDU 类型清单：用于 BatchAddModal / DataPointModal 等 dropdown。
// `value` 是后端 `parse_asdu_type` 接受的 PascalCase 枚举名；
// `labelKey` 是 i18n 字典里的 key（zh-CN / en-US 在 asduType.* 下定义）；
// `typeId` 是 IEC 60870-5-101/104 type identification 数字编号,
// 与 crates/iec104sim-core/src/types.rs::AsduTypeId 一致；
// `category` 是子站树的分类稳定键（snake_case,与后端 DataCategory 序列化一致）,
// 用于「新建点位时只显示当前分类相关类型」的过滤。
export interface AsduTypeOption {
  value: string
  labelKey: string
  typeId: number
  category: string
}

export const ASDU_TYPE_OPTIONS: AsduTypeOption[] = [
  { value: 'MSpNa1', labelKey: 'asduType.sp',    typeId: 1,  category: 'single_point' },
  { value: 'MSpTa1', labelKey: 'asduType.sp_ta', typeId: 2,  category: 'single_point' },
  { value: 'MSpTb1', labelKey: 'asduType.sp_tb', typeId: 30, category: 'single_point' },
  { value: 'MDpNa1', labelKey: 'asduType.dp',    typeId: 3,  category: 'double_point' },
  { value: 'MDpTa1', labelKey: 'asduType.dp_ta', typeId: 4,  category: 'double_point' },
  { value: 'MDpTb1', labelKey: 'asduType.dp_tb', typeId: 31, category: 'double_point' },
  { value: 'MStNa1', labelKey: 'asduType.st',    typeId: 5,  category: 'step_position' },
  { value: 'MStTa1', labelKey: 'asduType.st_ta', typeId: 6,  category: 'step_position' },
  { value: 'MStTb1', labelKey: 'asduType.st_tb', typeId: 32, category: 'step_position' },
  { value: 'MBoNa1', labelKey: 'asduType.bo',    typeId: 7,  category: 'bitstring' },
  { value: 'MBoTb1', labelKey: 'asduType.bo_tb', typeId: 33, category: 'bitstring' },
  { value: 'MMeNa1', labelKey: 'asduType.me_na', typeId: 9,  category: 'normalized_measured' },
  { value: 'MMeTa1', labelKey: 'asduType.me_ta', typeId: 10, category: 'normalized_measured' },
  { value: 'MMeTd1', labelKey: 'asduType.me_td', typeId: 34, category: 'normalized_measured' },
  { value: 'MMeNd1', labelKey: 'asduType.me_nd', typeId: 21, category: 'normalized_measured' },
  { value: 'MMeNb1', labelKey: 'asduType.me_nb', typeId: 11, category: 'scaled_measured' },
  { value: 'MMeTb1', labelKey: 'asduType.me_tb', typeId: 12, category: 'scaled_measured' },
  { value: 'MMeTe1', labelKey: 'asduType.me_te', typeId: 35, category: 'scaled_measured' },
  { value: 'MMeNc1', labelKey: 'asduType.me_nc', typeId: 13, category: 'float_measured' },
  { value: 'MMeTc1', labelKey: 'asduType.me_tc', typeId: 14, category: 'float_measured' },
  { value: 'MMeTf1', labelKey: 'asduType.me_tf', typeId: 36, category: 'float_measured' },
  { value: 'MItNa1', labelKey: 'asduType.it',    typeId: 15, category: 'integrated_totals' },
  { value: 'MItTb1', labelKey: 'asduType.it_tb', typeId: 37, category: 'integrated_totals' },
  { value: 'CScNa1', labelKey: 'asduType.c_sc_na', typeId: 45, category: 'single_command' },
  { value: 'CScTa1', labelKey: 'asduType.c_sc_ta', typeId: 58, category: 'single_command' },
  { value: 'CDcNa1', labelKey: 'asduType.c_dc_na', typeId: 46, category: 'double_command' },
  { value: 'CDcTa1', labelKey: 'asduType.c_dc_ta', typeId: 59, category: 'double_command' },
  { value: 'CRcNa1', labelKey: 'asduType.c_rc_na', typeId: 47, category: 'step_command' },
  { value: 'CRcTa1', labelKey: 'asduType.c_rc_ta', typeId: 60, category: 'step_command' },
  { value: 'CSeNa1', labelKey: 'asduType.c_se_na', typeId: 48, category: 'normalized_setpoint' },
  { value: 'CSeTa1', labelKey: 'asduType.c_se_ta', typeId: 61, category: 'normalized_setpoint' },
  { value: 'CSeNb1', labelKey: 'asduType.c_se_nb', typeId: 49, category: 'scaled_setpoint' },
  { value: 'CSeTb1', labelKey: 'asduType.c_se_tb', typeId: 62, category: 'scaled_setpoint' },
  { value: 'CSeNc1', labelKey: 'asduType.c_se_nc', typeId: 50, category: 'float_setpoint' },
  { value: 'CSeTc1', labelKey: 'asduType.c_se_tc', typeId: 63, category: 'float_setpoint' },
  { value: 'CBoNa1', labelKey: 'asduType.c_bo_na', typeId: 51, category: 'bitstring_command' },
  { value: 'CBoTa1', labelKey: 'asduType.c_bo_ta', typeId: 64, category: 'bitstring_command' },
]

/// 按显示名(如 "M_SP_NA_1")或 PascalCase 枚举名查条目;忽略大小写与分隔符。
export function findAsduTypeOption(raw: string): AsduTypeOption | undefined {
  const key = raw.replace(/[^a-z0-9]/gi, '').toLowerCase()
  return ASDU_TYPE_OPTIONS.find(o => o.value.toLowerCase() === key)
}

/// 表格/下拉的显示后缀:`M_SP_NA_1 (Type ID: 1)`。未知类型原样返回。
export function formatAsduTypeWithId(raw: string): string {
  const opt = findAsduTypeOption(raw)
  return opt ? `${raw} (Type ID: ${opt.typeId})` : raw
}
