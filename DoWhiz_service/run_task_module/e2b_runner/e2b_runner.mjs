import { Sandbox } from 'e2b';
import fs from 'node:fs';
import path from 'node:path';

async function readStdin() {
  return await new Promise((resolve, reject) => {
    let data = '';
    process.stdin.setEncoding('utf8');
    process.stdin.on('data', (chunk) => (data += chunk));
    process.stdin.on('end', () => resolve(data));
    process.stdin.on('error', reject);
  });
}

function commandResultFromError(err) {
  if (err && typeof err === 'object') {
    const exitCode = typeof err.exitCode === 'number' ? err.exitCode : 1;
    const stdout = typeof err.stdout === 'string' ? err.stdout : '';
    const stderr = typeof err.stderr === 'string' ? err.stderr : '';
    return {
      exitCode,
      stdout,
      stderr,
      error: err.message ? String(err.message) : 'command failed',
    };
  }
  return {
    exitCode: 1,
    stdout: '',
    stderr: '',
    error: err ? String(err) : 'command failed',
  };
}

async function run() {
  const raw = await readStdin();
  if (!raw || !raw.trim()) {
    throw new Error('missing config on stdin');
  }
  const config = JSON.parse(raw);
  const templateId = config.templateId;
  const apiKey = config.apiKey;
  if (!templateId || !apiKey) {
    throw new Error('templateId and apiKey are required');
  }

  const sandbox = await Sandbox.create(templateId, {
    apiKey,
    timeoutMs: config.timeoutMs,
    envs: config.sandboxEnv || {},
    metadata: config.metadata || {},
    allowInternetAccess: true,
  });

  const remoteWorkspace = config.remoteWorkspace || '/workspace';
  const remoteTarPath = config.remoteTarPath || '/tmp/workspace.tar';
  const remoteOutputTar = config.remoteOutputTar || '/tmp/workspace_out.tar';
  const sandboxUser = config.user || 'root';
  const bootstrapUser = config.bootstrapUser || sandboxUser;
  const commandUser = config.commandUser || sandboxUser;

  try {
    await sandbox.commands.run(`mkdir -p ${remoteWorkspace} /tmp`, { user: bootstrapUser });

    if (config.workspaceTar) {
      const tarBytes = fs.readFileSync(config.workspaceTar);
      await sandbox.files.write(remoteTarPath, tarBytes);
      await sandbox.commands.run(`tar -xf ${remoteTarPath} -C ${remoteWorkspace}`, {
        user: bootstrapUser,
      });
    }

    if (Array.isArray(config.bootstrap)) {
      for (const cmd of config.bootstrap) {
        if (!cmd) continue;
        await sandbox.commands.run(cmd, {
          user: bootstrapUser,
          envs: config.env || {},
          timeoutMs: config.bootstrapTimeoutMs,
        });
      }
    }

    let result;
    try {
      result = await sandbox.commands.run(config.command, {
        user: commandUser,
        cwd: remoteWorkspace,
        envs: config.env || {},
        timeoutMs: config.commandTimeoutMs,
      });
    } catch (err) {
      result = commandResultFromError(err);
    }

    await sandbox.commands.run(`tar -cf ${remoteOutputTar} -C ${remoteWorkspace} .`, {
      user: bootstrapUser,
      timeoutMs: config.bootstrapTimeoutMs,
    });

    if (config.localOutputTar) {
      const outBytes = await sandbox.files.read(remoteOutputTar, { format: 'bytes' });
      fs.writeFileSync(config.localOutputTar, Buffer.from(outBytes));
    }

    const payload = {
      ok: result.exitCode === 0,
      exitCode: result.exitCode,
      stdout: result.stdout || '',
      stderr: result.stderr || '',
      error: result.error || null,
      sandboxId: sandbox.sandboxId,
    };

    process.stdout.write(JSON.stringify(payload));
  } finally {
    try {
      await sandbox.kill();
    } catch (_) {
      // ignore cleanup failures
    }
  }
}

run().catch((err) => {
  const payload = {
    ok: false,
    exitCode: 1,
    stdout: '',
    stderr: '',
    error: err ? String(err) : 'e2b runner failed',
  };
  try {
    process.stdout.write(JSON.stringify(payload));
  } catch (_) {
    // ignore
  }
  process.exitCode = 1;
});
