// ================================================= Array helpers =================================================

// Cut an array into an array of sized-length arrays
// Example: chunk([1,2,3,4,5,6,7,8], 3) → [[1,2,3],[4,5,6],[7,8]]
export const chunk = <T>(array: T[], size = 1) =>
  Array.from({ length: Math.ceil(array.length / size) }, (_, i) =>
    array.slice(i * size, i * size + size)
  );

// Cartesian cross product of an array of arrays
// Example: product([1,2],[a,b,c],[true]) → [[1,a,true],[1,b,true],[1,c,true],[2,a,true],[2,b,true],[2,c,true]]
export const product = (...arrays: any) =>
  arrays.reduce(
    (a: any, b: any) => a.flatMap((ai: any) => b.map((bi: any) => [...ai, bi])),
    [[]]
  );

// Range from start to end in increment
// Example: range(17,42,7) → [17,24,31,38]
export const range = (
  start: number,
  stop: number | undefined = undefined,
  step = 1
) => {
  if (!stop) {
    stop = start;
    start = 0;
  }
  return start < stop
    ? Array.from(
        { length: Math.ceil((stop - start) / step) },
        (_, i) => start + i * step
      )
    : [];
};

// Unique elements, with an optional getter function
// Example: unique([1,1,2,3,4,8,1,3,8,13,42]) → [1,2,3,4,8,13,42]
export const unique = <T>(array: T[], op = (x: T) => x) =>
  array.filter(
    (obj, i) => array.findIndex((entry) => op(obj) === op(entry)) === i
  );

// Zip arrays together. If some arrays are smaller, undefined is used as a filler.
// Example: zip([1,2],[a,b,c],[true]) → [[1,a,true],[2,b,undefined],[undefined,c,undefined]]
export const zip = (...args: any) =>
  Array.from(
    { length: Math.max(...args.map((arg: any) => arg.length)) },
    (_, i) => args.map((arg: any) => arg[i])
  );

// ================================================ Object helpers =================================================

// Create a new object by mapping the values through a function, keeping the keys
// Example: mapValues({a:1,b:2,c:3}, x => x**2) → {a:1,b:4,c:9}
export const mapValues = <TKey extends string | number | symbol, TVal, TValRes>(
  obj: Record<TKey, TVal>,
  fn: (arg: TVal) => TValRes
) =>
  Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, fn(v as TVal)]));
