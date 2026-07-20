// slave-batch-add-expression: 批量添加的 IOA 表达式展开 (issue #28)
import { describe, it, expect } from 'vitest'
import { parseIoaExpression, expandIoaExpression } from '../../src/components/batchAdd/ioaRanges'

describe('expandIoaExpression', () => {
  it('展开连续范围 6001-6005', () => {
    const expr = parseIoaExpression('6001-6005')
    expect(expandIoaExpression(expr, 100000)).toEqual([6001, 6002, 6003, 6004, 6005])
  })

  it('展开逗号列表并升序去重', () => {
    const expr = parseIoaExpression('6012, 6001, 6003, 6003')
    expect(expandIoaExpression(expr, 100000)).toEqual([6001, 6003, 6012])
  })

  it('范围与单点混合去重', () => {
    const expr = parseIoaExpression('10-12, 11, 20')
    expect(expandIoaExpression(expr, 100000)).toEqual([10, 11, 12, 20])
  })

  it('语法错时返回空数组', () => {
    const expr = parseIoaExpression('abc')
    expect(expr.error).toBe('abc')
    expect(expandIoaExpression(expr, 100000)).toEqual([])
  })

  it('超过 cap 返回 null 且带早停(巨区间不真展开)', () => {
    const expr = parseIoaExpression('0-16777215')
    const start = Date.now()
    expect(expandIoaExpression(expr, 1000)).toBeNull()
    expect(Date.now() - start).toBeLessThan(1000)
  })

  it('cap 边界:恰好等于 cap 不为 null', () => {
    const expr = parseIoaExpression('1-100')
    expect(expandIoaExpression(expr, 100)).toHaveLength(100)
    expect(expandIoaExpression(expr, 99)).toBeNull()
  })
})
