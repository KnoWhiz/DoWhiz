> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Interactive terminal (PTY)

The PTY (pseudo-terminal) module allows you to create interactive terminal sessions in the sandbox with real-time, bidirectional communication.

Unlike `commands.run()` which executes a command and returns output after completion, PTY provides:

* **Real-time streaming** - Output is streamed as it happens via callbacks
* **Bidirectional input** - Send input while the terminal is running
* **Interactive shell** - Full terminal support with ANSI colors and escape sequences
* **Session persistence** - Disconnect and reconnect to running sessions

## Create a PTY session

Use `sandbox.pty.create()` to start an interactive bash shell.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const terminal = await sandbox.pty.create({
    cols: 80,              // Terminal width in characters
    rows: 24,              // Terminal height in characters
    onData: (data) => {
      // Called whenever terminal outputs data
      process.stdout.write(data)
    },
    envs: { MY_VAR: 'hello' },  // Optional environment variables
    cwd: '/home/user',          // Optional working directory
    user: 'root',               // Optional user to run as
  })

  // terminal.pid contains the process ID
  console.log('Terminal PID:', terminal.pid)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  terminal = sandbox.pty.create(
      cols=80,              # Terminal width in characters
      rows=24,              # Terminal height in characters
      on_data=lambda data: print(data.decode(), end=''),  # end='' prevents print from adding extra newline
      envs={'MY_VAR': 'hello'},  # Optional environment variables
      cwd='/home/user',          # Optional working directory
      user='root',               # Optional user to run as
  )

  # terminal.pid contains the process ID
  print('Terminal PID:', terminal.pid)
  ```
</CodeGroup>

<Note>
  The PTY runs an interactive bash shell with `TERM=xterm-256color`, which supports ANSI colors and escape sequences.
</Note>

## Timeout

By default, PTY sessions have a **60-second timeout** which limits the total duration of the session. When the timeout is reached, the connection to the PTY session will be closed regardless of activity.

For long-running sessions, set `timeoutMs: 0` (JavaScript) or `timeout=0` (Python) to disable the timeout.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const terminal = await sandbox.pty.create({
    cols: 80,
    rows: 24,
    onData: (data) => process.stdout.write(data),
    timeoutMs: 0,  // No timeout for long-running sessions
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  # end='' prevents print() from adding an extra newline
  # (PTY output already contains newlines)
  terminal = sandbox.pty.create(
      cols=80,
      rows=24,
      on_data=lambda data: print(data.decode(), end=''),
      timeout=0,  # No timeout for long-running sessions
  )
  ```
</CodeGroup>

## Send input to PTY

Use `sendInput()` in JavaScript or `send_stdin()` in Python to send data to the terminal. These methods return a Promise (JavaScript) or complete synchronously (Python) - the actual output will be delivered to your `onData` callback.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const terminal = await sandbox.pty.create({
    cols: 80,
    rows: 24,
    onData: (data) => process.stdout.write(data),
  })

  // Send a command (don't forget the newline!)
  await sandbox.pty.sendInput(
    terminal.pid,
    new TextEncoder().encode('echo "Hello from PTY"\n')
  )
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  terminal = sandbox.pty.create(
      cols=80,
      rows=24,
      on_data=lambda data: print(data.decode(), end=''),  # end='' prevents extra newline
  )

  # Send a command as bytes (b'...' is Python's byte string syntax)
  # Don't forget the newline!
  sandbox.pty.send_stdin(terminal.pid, b'echo "Hello from PTY"\n')
  ```
</CodeGroup>

## Resize the terminal

When the user's terminal window changes size, notify the PTY with `resize()`. The `cols` and `rows` parameters are measured in characters, not pixels.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const terminal = await sandbox.pty.create({
    cols: 80,
    rows: 24,
    onData: (data) => process.stdout.write(data),
  })

  // Resize to new dimensions (in characters)
  await sandbox.pty.resize(terminal.pid, {
    cols: 120,
    rows: 40,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  terminal = sandbox.pty.create(
      cols=80,
      rows=24,
      on_data=lambda data: print(data.decode(), end=''),  # end='' prevents extra newline
  )

  # Resize to new dimensions (in characters)
  sandbox.pty.resize(terminal.pid, cols=120, rows=40)
  ```
</CodeGroup>

## Disconnect and reconnect

You can disconnect from a PTY session while keeping it running, then reconnect later with a new data handler. This is useful for:

* Resuming terminal sessions after network interruptions
* Sharing terminal access between multiple clients
* Implementing terminal session persistence

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  // Create a PTY session
  const terminal = await sandbox.pty.create({
    cols: 80,
    rows: 24,
    onData: (data) => console.log('Handler 1:', new TextDecoder().decode(data)),
  })

  const pid = terminal.pid

  // Send a command
  await sandbox.pty.sendInput(pid, new TextEncoder().encode('echo hello\n'))

  // Disconnect - PTY keeps running in the background
  await terminal.disconnect()

  // Later: reconnect with a new data handler
  const reconnected = await sandbox.pty.connect(pid, {
    onData: (data) => console.log('Handler 2:', new TextDecoder().decode(data)),
  })

  // Continue using the session
  await sandbox.pty.sendInput(pid, new TextEncoder().encode('echo world\n'))

  // Wait for the terminal to exit
  await reconnected.wait()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import time
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  # Create a PTY session
  terminal = sandbox.pty.create(
      cols=80,
      rows=24,
      on_data=lambda data: print('Handler 1:', data.decode()),
  )

  pid = terminal.pid

  # Send a command
  sandbox.pty.send_stdin(pid, b'echo hello\n')
  time.sleep(0.5)

  # Disconnect - PTY keeps running in the background
  terminal.disconnect()

  # Later: reconnect with a new data handler
  reconnected = sandbox.pty.connect(
      pid,
      on_data=lambda data: print('Handler 2:', data.decode()),
  )

  # Continue using the session
  sandbox.pty.send_stdin(pid, b'echo world\n')

  # Wait for the terminal to exit
  reconnected.wait()
  ```
</CodeGroup>

## Kill the PTY

Terminate the PTY session with `kill()`.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const terminal = await sandbox.pty.create({
    cols: 80,
    rows: 24,
    onData: (data) => process.stdout.write(data),
  })

  // Kill the PTY
  const killed = await sandbox.pty.kill(terminal.pid)
  console.log('Killed:', killed)  // true if successful

  // Or use the handle method
  // await terminal.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  terminal = sandbox.pty.create(
      cols=80,
      rows=24,
      on_data=lambda data: print(data.decode(), end=''),  # end='' prevents extra newline
  )

  # Kill the PTY
  killed = sandbox.pty.kill(terminal.pid)
  print('Killed:', killed)  # True if successful

  # Or use the handle method
  # terminal.kill()
  ```
</CodeGroup>

## Wait for PTY to exit

Use `wait()` to wait for the terminal session to end (e.g., when the user types `exit`).

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const terminal = await sandbox.pty.create({
    cols: 80,
    rows: 24,
    onData: (data) => process.stdout.write(data),
  })

  // Send exit command
  await sandbox.pty.sendInput(terminal.pid, new TextEncoder().encode('exit\n'))

  // Wait for the terminal to exit
  const result = await terminal.wait()
  console.log('Exit code:', result.exitCode)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox()

  terminal = sandbox.pty.create(
      cols=80,
      rows=24,
      on_data=lambda data: print(data.decode(), end=''),  # end='' prevents extra newline
  )

  # Send exit command
  sandbox.pty.send_stdin(terminal.pid, b'exit\n')

  # Wait for the terminal to exit
  result = terminal.wait()
  print('Exit code:', result.exit_code)
  ```
</CodeGroup>

## Interactive terminal (SSH-like)

Building a fully interactive terminal (like SSH) requires handling raw mode, stdin forwarding, and terminal resize events. For a production implementation, see the [E2B CLI source code](https://github.com/e2b-dev/E2B/blob/main/packages/cli/src/terminal.ts) which uses the same `sandbox.pty` API documented above.
