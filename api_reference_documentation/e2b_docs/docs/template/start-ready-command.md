> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Start & ready commands

> Define running processes for the sandbox

## Start command

The start command allows you to specify a command that will be **already running** when you spawn your custom sandbox.
This way, you can for example have running servers or seeded databases inside the sandbox that are already fully ready when you spawn the sandbox using the SDK and with zero waiting time for your users during the runtime.

The idea behind the start command feature is to lower the wait times for your users and have everything ready for your users when you spawn your sandbox.

You can see how it works [here](/docs/template/how-it-works).

## Ready command

The ready command allows you to specify a command that will determine **template sandbox** readiness before a [snapshot](/docs/template/how-it-works) is created.
It is executed in an infinite loop until it returns a successful **exit code 0**.
This way you can control how long should we wait for the [start command](/docs/template/start-ready-command#start-command) or any system state.

## Usage

Set the command that runs when the sandbox starts and the command that determines when the sandbox is ready:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Set both start command and ready command
  template.setStartCmd('npm start', waitForPort(3000))

  // Set custom start and ready command
  template.setStartCmd('npm start', 'curl -s -o /dev/null -w "200"')

  // Set only ready command
  template.setReadyCmd(waitForTimeout(10_000))
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Set both start command and ready command
  template.set_start_cmd("npm start", wait_for_port(3000))

  # Set custom start and ready command
  template.set_start_cmd("npm start", 'curl -s -o /dev/null -w "200"')

  # Set only ready command
  template.set_ready_cmd(wait_for_timeout(10_000))
  ```
</CodeGroup>

The ready command is used to determine when the sandbox is ready to accept connections.

## Ready command helpers

The SDK provides helper functions for common ready command patterns:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import {
    waitForPort,
    waitForProcess,
    waitForFile,
    waitForTimeout,
  } from 'e2b'

  // Wait for a port to be available
  waitForPort(3000)

  // Wait for a process to be running
  waitForProcess('node')

  // Wait for a file to exist
  waitForFile('/tmp/ready')

  // Wait for a timeout
  waitForTimeout(10_000) // 10 seconds
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import wait_for_port, wait_for_process, wait_for_file, wait_for_timeout

  # Wait for a port to be available
  wait_for_port(3000)

  # Wait for a process to be running
  wait_for_process("node")

  # Wait for a file to exist
  wait_for_file("/tmp/ready")

  # Wait for a specified duration
  wait_for_timeout(10_000) # 10 seconds
  ```
</CodeGroup>
