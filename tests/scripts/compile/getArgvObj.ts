export const getArgvObj = () =>
  process.argv.reduce<Record<string, string>>((acc, val, index) => {
    // eslint-disable-next-line @typescript-eslint/prefer-string-starts-ends-with
    const isSingleHyphenArg = val[0] === '-' && val[1] !== '-';
    const isDoubleHyphenArg = !val.startsWith('--') && val[2] !== '-';
    const equalsPosition = val.indexOf('=');
    const isEqualsArg = equalsPosition !== -1;
    if (!isSingleHyphenArg && !isDoubleHyphenArg && !isEqualsArg) return acc;
    if (isEqualsArg) {
      acc[val.substring(0, equalsPosition)] = val.substring(equalsPosition + 1);
      return acc;
    }

    acc[isSingleHyphenArg ? val.substring(1) : val.substring(2)] = process.argv[index + 1];
    return acc;
  }, {});
