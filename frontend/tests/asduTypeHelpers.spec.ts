// slave-asdu-type-helpers: CP24 类型补齐 + Type ID 显示 + 分类过滤数据 (issue #28)
import { describe, it, expect } from 'vitest'
import {
  ASDU_TYPE_OPTIONS,
  findAsduTypeOption,
  formatAsduTypeWithId,
} from '../src/constants/asduTypes'

describe('ASDU_TYPE_OPTIONS', () => {
  it('包含全部 6 个 CP24 (TA) 监视类型', () => {
    const byId = new Map(ASDU_TYPE_OPTIONS.map(o => [o.typeId, o]))
    expect(byId.get(2)?.value).toBe('MSpTa1')
    expect(byId.get(4)?.value).toBe('MDpTa1')
    expect(byId.get(6)?.value).toBe('MStTa1')
    expect(byId.get(10)?.value).toBe('MMeTa1')
    expect(byId.get(12)?.value).toBe('MMeTb1')
    expect(byId.get(14)?.value).toBe('MMeTc1')
  })

  it('每个条目都归属一个分类稳定键', () => {
    for (const opt of ASDU_TYPE_OPTIONS) {
      expect(opt.category, `${opt.value} 缺 category`).toMatch(/^[a-z_]+$/)
    }
    // 抽查:控制类型对(45/58)同属 single_command → 分类过滤后只剩这两个
    const singleCmd = ASDU_TYPE_OPTIONS.filter(o => o.category === 'single_command')
    expect(singleCmd.map(o => o.typeId).sort((a, b) => a - b)).toEqual([45, 58])
    const singlePoint = ASDU_TYPE_OPTIONS.filter(o => o.category === 'single_point')
    expect(singlePoint.map(o => o.typeId).sort((a, b) => a - b)).toEqual([1, 2, 30])
  })

  it('typeId 无重复', () => {
    const ids = ASDU_TYPE_OPTIONS.map(o => o.typeId)
    expect(new Set(ids).size).toBe(ids.length)
  })
})

describe('findAsduTypeOption / formatAsduTypeWithId', () => {
  it('按显示名与枚举名查找(忽略分隔符与大小写)', () => {
    expect(findAsduTypeOption('M_SP_NA_1')?.value).toBe('MSpNa1')
    expect(findAsduTypeOption('MSpNa1')?.value).toBe('MSpNa1')
    expect(findAsduTypeOption('m_me_tc_1')?.typeId).toBe(14)
    expect(findAsduTypeOption('Nope')).toBeUndefined()
  })

  it('表格类型列显示 Type ID (issue #28)', () => {
    expect(formatAsduTypeWithId('M_SP_NA_1')).toBe('M_SP_NA_1 (Type ID: 1)')
    expect(formatAsduTypeWithId('C_SE_NC_1')).toBe('C_SE_NC_1 (Type ID: 50)')
    expect(formatAsduTypeWithId('M_SP_TA_1')).toBe('M_SP_TA_1 (Type ID: 2)')
    expect(formatAsduTypeWithId('UNKNOWN_9')).toBe('UNKNOWN_9')
  })
})
