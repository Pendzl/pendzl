import fs from 'fs-extra';
import path from 'path';
import glob from 'glob';
import chalk from 'chalk';
import { getArgvObj } from 'scripts/compile/getArgvObj';

const snakeToCamel = (str: string) => str.toLowerCase().replace(/([-_][a-z])/g, (group) => group.toUpperCase().replace('-', '').replace('_', ''));

function capitalizeFirstLetter(string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

const replaceQueryCalls = (contractsRootPath: string, isDebug = false) => {
  const paths = glob.sync(`${contractsRootPath}/**/event-types/*.ts`);
  const foundEvents: { contractName: string; events: string[] }[] = [];
  for (const p of paths) {
    const data = fs.readFileSync(p, 'utf8');
    const matched = Array.from(data.matchAll(/export interface (.*) {/g)).map((match) => match[1]);
    if (matched.length > 0) foundEvents.push({ contractName: capitalizeFirstLetter(snakeToCamel(path.parse(p).name)), events: matched });
  }
  return foundEvents;
};

(async (args: Record<string, unknown>) => {
  if (require.main !== module) return;
  const typechainOutputPath = process.argv[2] ?? './typechain';
  const resultFileOutputPath = process.argv[3] ?? './typechain/events/enum.ts';
  console.log('Generating events enum');
  const foundEvents = replaceQueryCalls(typechainOutputPath);

  const enumString = `
${foundEvents
  .map(
    ({ contractName, events }) => `
export enum ${contractName}Event {
  ${events
    .filter((value, index, self) => self.indexOf(value) === index) // remove duplicates
    .map(
      (e, index) => `  ${e} = '${e}'${index === events.length - 1 ? '' : ','}
`,
    )
    .join('')}
}`,
  )
  .join('\n')}
export type AnyAbaxContractEvent = ${foundEvents
    .map(({ contractName }, index) => `${contractName}Event${index === foundEvents.length - 1 ? ';' : ' | '}`)
    .join('')}
export const ContractsEvents = {
  ${foundEvents
    .map(
      ({ contractName }, index) => `${contractName}Event${index === foundEvents.length - 1 ? '' : ','}
  `,
    )
    .join('')}
}
  `;
  fs.writeFileSync(resultFileOutputPath, enumString, 'utf8');
  console.log('Finished!');
  process.exit(0);
})(getArgvObj()).catch((e) => {
  console.log(e);
  console.error(chalk.red(JSON.stringify(e, null, 2)));
  process.exit(0);
});
