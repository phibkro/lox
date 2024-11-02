export function getKeyFromValue<K extends string | number | symbol, V>(
  object: Record<K, V>,
  value: V,
): K | undefined {
  return Object.keys(object).find(
    // Assert key type as key of Record<K, V> is always of type K
    (key) => object[key as K] === value,
    // Assert return type as find() on Record<K, V> always returns K or undefined
  ) as K | undefined;
}
