// 3-byte IOA upper bound per IEC 60870-5-101 §7.2.5.
export const IOA_MAX = 16_777_215

// Assumes xs is sorted ascending and unique.
export function compressRanges(xs: readonly number[]): string {
  if (xs.length === 0) return ''
  const fmt = (s: number, e: number) => (s === e ? String(s) : `${s}–${e}`)
  const parts: string[] = []
  let s = xs[0]
  let e = xs[0]
  for (let i = 1; i < xs.length; i++) {
    if (xs[i] === e + 1) {
      e = xs[i]
      continue
    }
    parts.push(fmt(s, e))
    s = e = xs[i]
  }
  parts.push(fmt(s, e))
  return parts.join(', ')
}

// Index of first element ≥ target in a sorted array.
export function lowerBound(xs: readonly number[], target: number): number {
  let l = 0
  let r = xs.length
  while (l < r) {
    const m = (l + r) >>> 1
    if (xs[m] < target) l = m + 1
    else r = m
  }
  return l
}

// Smallest s ≥ 0 such that [s, s+count-1] is disjoint from xs.
// xs must be sorted ascending. Returns null if result would exceed IOA_MAX.
export function findNextFreeGap(xs: readonly number[], count: number): number | null {
  if (count <= 0) return 0
  let s = 0
  for (const x of xs) {
    if (x < s) continue
    if (x <= s + count - 1) {
      s = x + 1
      continue
    }
    break
  }
  if (s + count - 1 > IOA_MAX) return null
  return s
}

export interface IoaExpr {
  ranges: Array<[number, number]>  // 已校验 lo<=hi 且 hi<=IOA_MAX
  singles: number[]                // 升序去重
  error: string | null             // 非 null = 语法错（值为肇事 token），调用方据此禁用写入
}

// 解析 IOA 表达式：逗号/空格/换行分隔；单点 `n`、闭区间 `a-b`。
// 任一非法 token（非数字、b<a、越域）立即返回 error=该 token。
export function parseIoaExpression(input: string): IoaExpr {
  const ranges: Array<[number, number]> = []
  const singles = new Set<number>()
  const tokens = input.split(/[\s,]+/).filter((t) => t.length > 0)
  for (const tok of tokens) {
    const m = tok.match(/^(\d+)-(\d+)$/)
    if (m) {
      const lo = Number(m[1])
      const hi = Number(m[2])
      if (lo > hi || hi > IOA_MAX) {
        return { ranges, singles: sortedUnique(singles), error: tok }
      }
      ranges.push([lo, hi])
      continue
    }
    if (/^\d+$/.test(tok)) {
      const n = Number(tok)
      if (n > IOA_MAX) return { ranges, singles: sortedUnique(singles), error: tok }
      singles.add(n)
      continue
    }
    return { ranges, singles: sortedUnique(singles), error: tok }
  }
  return { ranges, singles: sortedUnique(singles), error: null }
}

function sortedUnique(s: Set<number>): number[] {
  return Array.from(s).sort((a, b) => a - b)
}

// 把表达式展开为升序去重的显式 IOA 列表。error 时返回空数组;
// 展开数量超过 cap 返回 null(调用方据此提示批量过大),循环带早停,
// 巨区间 (如 0-16777215) 不会真的展开 1600 万项。
export function expandIoaExpression(expr: IoaExpr, cap: number): number[] | null {
  if (expr.error) return []
  const out = new Set<number>()
  for (const n of expr.singles) {
    out.add(n)
    if (out.size > cap) return null
  }
  for (const [lo, hi] of expr.ranges) {
    for (let n = lo; n <= hi; n++) {
      out.add(n)
      if (out.size > cap) return null
    }
  }
  return Array.from(out).sort((a, b) => a - b)
}

export interface IoaHits {
  hitIoas: number[]        // 升序去重：实际存在且被表达式覆盖的 IOA
  missedSingles: number[]  // 升序去重：合法但不存在的单点（区间不计入）
}

// existingIoas 必须升序去重（区间用 lowerBound 二分切片）。
export function resolveIoaHits(expr: IoaExpr, existingIoas: readonly number[]): IoaHits {
  if (expr.error) return { hitIoas: [], missedSingles: [] }
  const existingSet = new Set(existingIoas)
  const hit = new Set<number>()
  const missed = new Set<number>()
  for (const n of expr.singles) {
    if (existingSet.has(n)) hit.add(n)
    else missed.add(n)
  }
  for (const [lo, hi] of expr.ranges) {
    const start = lowerBound(existingIoas, lo)
    const end = lowerBound(existingIoas, hi + 1)
    for (let i = start; i < end; i++) hit.add(existingIoas[i])
  }
  return {
    hitIoas: Array.from(hit).sort((a, b) => a - b),
    missedSingles: Array.from(missed).sort((a, b) => a - b),
  }
}
