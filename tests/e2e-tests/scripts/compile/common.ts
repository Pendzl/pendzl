import util from 'node:util';
import fs, { copy } from 'fs-extra';
import chalk from 'chalk';
import { exec, spawn } from 'child_process';
import path from 'path';
import glob from 'glob';

export const getLineSeparator = () => '='.repeat(process.stdout.columns ?? 60);
export const execPromise = util.promisify(exec);

export const createFileWithDirectoriesSync = (filePath: string, data: string) => {
  fs.ensureFileSync(filePath);
  fs.writeFileSync(filePath, data);
};

export const compileContract = async (contractPath: string) => {
  const command = 'cargo';
  const args = ['contract', 'build', ...(process.env.BUILD_PROD ? ['--release'] : [])];
  console.log(getLineSeparator());
  console.log(chalk.bgGreen(`running ${command} ${args.join(' ')}...`));
  console.log(getLineSeparator());

  return new Promise<number>((resolve, reject) => {
    const process = spawn(command, args, { cwd: contractPath, stdio: 'inherit' });
    process.stdout?.on('data', (data) => {
      console.log(data);
    });
    process.stderr?.on('data', (data) => {
      console.log(data);
    });
    process.on('exit', function (code) {
      if (code === null || code === 0) resolve(code ?? 0);
      reject(code);
    });
    process.on('error', function (err) {
      reject(err);
    });
  });
};

function copyArtifactsInternal(
  compileOutputPath: string,
  contractName: string,
  artifactsOutputPath: string,
  outputContractName: string = contractName,
) {
  console.log(`Copying from ${compileOutputPath} to ${artifactsOutputPath}...`);
  fs.ensureDirSync(artifactsOutputPath);
  fs.copyFileSync(path.join(compileOutputPath, `${contractName}.contract`), path.join(artifactsOutputPath, `${outputContractName}.contract`));
  fs.copyFileSync(path.join(compileOutputPath, `${contractName}.wasm`), path.join(artifactsOutputPath, `${outputContractName}.wasm`));
  fs.copyFileSync(path.join(compileOutputPath, `${contractName}.json`), path.join(artifactsOutputPath, `${outputContractName}.json`));
}

export const copyArtifacts = (fullPath: string, contractName: string) => {
  const contractFolderName = path.dirname(fullPath).split(path.sep).pop();
  const contractFolderPath = path.parse(fullPath).dir;
  const contractNameSanitized = contractName.replace(/-/g, '_');
  const workspaceArtifactsCompileOutputPath = path.join('src', 'target', 'ink', contractNameSanitized);
  const localArtifactsCompileOutputPath = path.join(contractFolderPath, 'target', 'ink');
  const artifactsOutputPath = path.join('artifacts');
  console.log(`Copying artifacts of ${contractName} using name ${contractFolderName} as an output name...`);
  try {
    copyArtifactsInternal(localArtifactsCompileOutputPath, contractNameSanitized, artifactsOutputPath, contractFolderName);
  } catch (_) {
    console.log('copying from local failed, trying from workspace');
    try {
      copyArtifactsInternal(workspaceArtifactsCompileOutputPath, contractNameSanitized, artifactsOutputPath, contractFolderName);
    } catch (e) {
      console.error('Failed to copy artifacts');
      throw e;
    }
  }
};

export const compileContractByNameAndCopyArtifacts = async (fullPath: string, contractName: string) => {
  const contractFolderPath = path.parse(fullPath).dir;
  console.log(getLineSeparator());
  console.log(chalk.bgGreen(`compiling contract ${contractName} from ${contractFolderPath}...`));
  console.log(getLineSeparator());
  try {
    await compileContract(contractFolderPath);
  } catch (e) {
    console.error(`Contract ${contractName} failed to compile`);
    throw e;
  }
  copyArtifacts(fullPath, contractName);
};
