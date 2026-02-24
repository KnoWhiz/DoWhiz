> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Shutdown running sandboxes

You can shutdown single or all running sandboxes with the E2B CLI.

## Shutdown single or multiple sandboxes

To shutdown a single or multiple sandboxes, run the following command:

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox kill <sandbox-id1> <sandbox-id2> <sandbox-id3>
  ```
</CodeGroup>

## Shutdown all sandboxes

To shutdown all running sandboxes, run the following command:

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox kill --all
  ```
</CodeGroup>

Further, you can filter the sandboxes to be shutdown by state, metadata or both.

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox kill --all --state=running,paused
  ```
</CodeGroup>

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox kill --all --metadata=key=value
  ```
</CodeGroup>

<CodeGroup>
  ```bash Terminal theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  e2b sandbox kill --all --state=running,paused --metadata=key=value
  ```
</CodeGroup>
