> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Connect to sandbox

You can connect an interactive terminal to an already running sandbox.

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox connect <sandbox-id>
  ```
</CodeGroup>

Unlike the `create` command, `connect` does not kill the sandbox when you disconnect. When you exit the terminal, only your terminal session is closed—the sandbox continues running.

Once connected, you can inspect the sandbox filesystem and processes to debug or experiment, or use it as a disposable environment for running agents instead of your local computer.
