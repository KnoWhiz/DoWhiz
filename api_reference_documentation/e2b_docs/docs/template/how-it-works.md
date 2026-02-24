> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# How it works

> How the template building process works

## General overview

Every time you are building a sandbox template, we create a container based on the definition.
We extract the container's filesystem, do provisioning and configuration (e.g. installing required dependencies), run layers commands and start a sandbox.

Then, these steps happen:

1. We take the running sandbox.
2. (Only if you specified the [start command](/docs/template/start-ready-command#start-command), otherwise this step is skipped) Execute the start command.
3. Wait for readiness (by default 20 seconds if start command is specified, otherwise immediately ready). Readiness check can be configured using [ready command](/docs/template/start-ready-command#ready-command).
4. Snapshot the sandbox and make it ready for you to spawn it with the SDK.

We call this sandbox snapshot a *sandbox template*.

<Note title="Sandbox snapshot">
  Snapshots are saved running sandboxes. We serialize and save the whole sandbox's filesystem together with all the
  running processes in a way that can be loaded later.

  This allows us to load the sandbox in \~80ms any time later with all the processes already running
  and the filesystem exactly as it was.
</Note>

## Default user and workdir

To learn more about default user and workdir, please refer to the [User and workdir](/docs/template/user-and-workdir) section.

## Caching

To learn more about caching, please refer to the [Caching](/docs/template/caching) section.

## Kernel

E2B sandboxes run on an **LTS 6.1 Linux kernel**.\
The exact kernel version available in your sandbox depends on **when your template was built**:

| Template build date (DD.MM.YYYY) | Kernel version    |
| -------------------------------- | ----------------- |
| ≥ 27.11.2025                     | 6.1.158 (current) |
| \< 27.11.2025                    | 6.1.102           |

> **Note:**\
> Kernel versions are fixed at **template build time**.\
> If your template was created on an older kernel, **the kernel cannot be upgraded or changed** for that template.\
> To use a newer kernel, you must **rebuild the template** or create a new one.

You can find the kernel configuration files for each version [here](https://github.com/e2b-dev/fc-kernels/tree/main/configs).
