import { spawn } from 'child_process';
import chalk from 'chalk';

const forbiddenRegexps = [
  /Unable to find handler for subscription/,
  /Unable to find active subscription/,
  /has multiple versions, ensure that there is only one installed/,
  /Either remove and explicitly install matching versions or dedupe using your package manager/,
  /The following conflicting packages were found/,
  /cjs \d[.]\d[.]\d/,
  /API-WS: disconnected from ws/,
  /RPC methods not decorated: timestamp_setTime/,
  /disconnected from ws:\/\/127.0.0.1:\d+: 1000:: Normal connection closure/,
  /1000:: Normal Closure/,
  /CONTRACT: Unable to decode contract event: createType\(AccountId\):: Invalid AccountId provided, expected 32 bytes, found 18/, // event decoding fails and that is an artifact of it
  /CONTRACT: Unable to decode contract event/,
];

(async () => {
  if (require.main !== module) return;
  const cliArgs = process.argv.slice(2);
  const command = cliArgs[0];
  const args = cliArgs.slice(1);
  console.log(`Executing command ` + chalk.green(`${command} ${args.join(' ')} `) + `with surpressed warnings!`);
  const p = spawn(command, args, { stdio: ['inherit', 'inherit', 'pipe'] });
  p.stderr?.on('data', (data) => {
    if (forbiddenRegexps.every((reg) => !data.toString().match(reg))) console.log(data.toString());
  });
  p.on('exit', function (code) {
    process.exit(code ?? 0);
  });
  p.on('error', function (err) {
    throw err;
  });
})().catch((e) => {
  console.log(e);
  console.error(chalk.red(JSON.stringify(e, null, 2)));
  process.exit(1);
});
