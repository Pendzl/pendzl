import { encodeAddress } from '@polkadot/util-crypto';
import chalk from 'chalk';
import fs from 'fs-extra';
import { getArgvObj } from 'scripts/compile/getArgvObj';

const printHelp = () => {
  console.log(chalk.yellow('Supply input file via') + chalk.green('--input <path> ') + chalk.yellow('or as a first argument of the script'));
  console.log(chalk.yellow('Supply output file via') + chalk.green('--output <path> ') + chalk.yellow('or as a second argument of the script'));
  console.log(`\nExample usages:`);
  console.log(chalk.cyan('npx tsx ./ansiFileToHtml.ts --input ./myFile.txt --output ./outputFile.html'));
  console.log(chalk.cyan('npx tsx ./ansiFileToHtml.ts ./myFile.txt ./outputFile.html'));
};
(async (args: Record<string, unknown>) => {
  if (require.main !== module) return;
  const inputFile = (args['input'] as string) ?? process.argv[2] ?? process.env.PWD;
  if (!inputFile) throw 'could not determine input path';
  if (!inputFile || !fs.pathExistsSync(inputFile)) {
    console.log(chalk.yellow('Invalid or missing arguments deposit!'));
    printHelp();
    process.exit(127);
  }

  const inputData = fs.readFileSync(inputFile, 'utf-8');
  const outputData = inputData.replace(/\[(.*)\]/g, (_, p1: string) => {
    const dataArray = p1.split(', ');
    if (dataArray.length === 32) {
      return encodeAddress(Uint8Array.from(dataArray.map((s) => parseInt(s, 16))), 42).toString();
    }
    return dataArray.join('');
  });
  fs.writeFileSync(`${inputFile.replace('.log', '')}.retouched.log`, outputData, 'utf-8');

  process.exit(0);
})(getArgvObj()).catch((e) => {
  console.log(e);
  console.error(chalk.red(JSON.stringify(e, null, 2)));
  process.exit(1);
});
