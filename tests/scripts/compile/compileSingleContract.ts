import { getArgvObj } from 'scripts/compile/getArgvObj';
import { compileContractByNameAndCopyArtifacts } from './common';
import chalk from 'chalk';

const printHelp = () => {
  console.log(
    chalk.yellow('Supply contract name via ') + chalk.green('--name <contract_name> ') + chalk.yellow('or as a first argument of the script'),
  );
  console.log(`\nExample usages:`);
  console.log(chalk.cyan('npm run cs flipper'));
  console.log(chalk.cyan('npm run cs --name flipper'));
};

(async (args: Record<string, unknown>) => {
  if (require.main !== module) return;
  const contractsRootPath = './src/contracts';
  const contractName = (args['name'] as string) ?? process.argv[2];
  if ((!args['name'] && process.argv.length === 4) || process.argv.length > 3) {
    console.log(chalk.yellow('Invalid or missing arguments supplied!'));
    printHelp();
    process.exit(127);
  }
  if (!contractName) {
    printHelp();
    process.exit(127);
  }
  await compileContractByNameAndCopyArtifacts(contractsRootPath, contractName);

  console.log('Success!');
  process.exit(0);
})(getArgvObj()).catch((e) => {
  console.log(e);
  console.error(chalk.red(JSON.stringify(e, null, 2)));
  process.exit(0);
});
