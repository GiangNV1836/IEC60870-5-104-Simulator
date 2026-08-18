/** Consistent ASDU label used by both Master and Slave data/detail views. */
export function formatAsduTypeWithId(raw: string, typeId?: number | null): string {
  return typeId == null ? raw : `${raw} (Type ID: ${typeId})`
}
