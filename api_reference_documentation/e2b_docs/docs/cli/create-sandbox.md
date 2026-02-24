> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Create sandbox

You can create a sandbox and connect an interactive terminal to it.

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox create <template>
  ```
</CodeGroup>

For example, to create a sandbox from the `base` template:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sandbox create base
```

This will:

1. Create a new sandbox from the specified template
2. Connect your terminal to the sandbox
3. Keep the sandbox alive while you're connected
4. Automatically kill the sandbox when you exit the terminal

Once connected, you can inspect the sandbox filesystem and processes to debug or experiment, or use it as a disposable environment for running agents instead of your local computer.
