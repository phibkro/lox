export function getKeyFromValue<K extends string | number | symbol, V>(
  object: Record<K, V>,
  value: V,
) {
  return Object.keys(object).find((key) => object[key as K] === value);
}
