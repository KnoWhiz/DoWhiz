> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Execute commands in sandbox

You can execute commands in a running sandbox.

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox exec <sandbox-id> <command>
  ```
</CodeGroup>

### Pipe command from stdin

You can pipe directly into the sandbox as well:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
echo "foo" | e2b sandbox exec <sandbox-id> <command>
```

### Run in background

Use the `--background` flag to run a command in the background and return immediately. The command will print the process ID (PID) to stderr:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sandbox exec --background <sandbox-id> "sleep 60 && echo done"
```

### Set working directory

Use the `--cwd` flag to specify the working directory for the command:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sandbox exec --cwd /home/user <sandbox-id> ls
```

### Run as specific user

Use the `--user` flag to run the command as a specific user:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sandbox exec --user root <sandbox-id> apt-get update
```

### Set environment variables

Use the `--env` flag to set environment variables. This flag can be repeated for multiple variables:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sandbox exec --env NODE_ENV=production --env DEBUG=true <sandbox-id> node app.js
```
