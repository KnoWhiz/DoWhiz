> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# SSH access

> Connect to your sandbox via SSH using a WebSocket proxy

SSH access enables remote terminal sessions, SCP/SFTP file transfers, and integration with tools that expect SSH connectivity.

## Quickstart

<Steps>
  <Step title="Build the SSH template">
    Define a template with OpenSSH server and [websocat](https://github.com/vi/websocat):

    <CodeGroup>
      ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      // template.ts
      import { Template, waitForPort } from 'e2b'

      export const template = Template()
        .fromUbuntuImage('25.04')
        .aptInstall(['openssh-server'])
        .runCmd([
          'curl -fsSL -o /usr/local/bin/websocat https://github.com/vi/websocat/releases/latest/download/websocat.x86_64-unknown-linux-musl',
          'chmod a+x /usr/local/bin/websocat',
        ], { user: 'root' })
        .setStartCmd('sudo websocat -b --exit-on-eof ws-l:0.0.0.0:8081 tcp:127.0.0.1:22', waitForPort(8081))
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      # template.py
      from e2b import Template, wait_for_port

      template = (
          Template()
          .from_ubuntu_image("25.04")
          .apt_install(["openssh-server"])
          .run_cmd([
              "curl -fsSL -o /usr/local/bin/websocat https://github.com/vi/websocat/releases/latest/download/websocat.x86_64-unknown-linux-musl",
              "chmod a+x /usr/local/bin/websocat",
          ], user="root")
          .set_start_cmd("sudo websocat -b --exit-on-eof ws-l:0.0.0.0:8081 tcp:127.0.0.1:22", wait_for_port(8081))
      )
      ```
    </CodeGroup>

    Build the template:

    <CodeGroup>
      ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      // build.ts
      import { Template, defaultBuildLogger } from 'e2b'
      import { template as sshTemplate } from './template'

      await Template.build(sshTemplate, 'ssh-ready', {
        cpuCount: 2,
        memoryMB: 2048,
        onBuildLogs: defaultBuildLogger(),
      })
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      # build.py
      from e2b import Template, default_build_logger
      from template import template as ssh_template

      Template.build(ssh_template, "ssh-ready",
          cpu_count=2,
          memory_mb=2048,
          on_build_logs=default_build_logger(),
      )
      ```
    </CodeGroup>
  </Step>

  <Step title="Create a sandbox from the template">
    <CodeGroup>
      ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      import { Sandbox } from 'e2b'

      const sbx = await Sandbox.create('ssh-ready')
      console.log(sbx.sandboxId)
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      from e2b import Sandbox

      sbx = Sandbox.create("ssh-ready")
      print(sbx.sandbox_id)
      ```
    </CodeGroup>
  </Step>

  <Step title="Connect from your machine">
    <CodeGroup>
      ```bash macOS theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      # Install websocat
      brew install websocat

      # Connect to your sandbox
      ssh -o 'ProxyCommand=websocat --binary -B 65536 - wss://8081-%h.e2b.app' user@<sandbox-id>
      ```

      ```bash Linux theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      # Install websocat
      sudo curl -fsSL -o /usr/local/bin/websocat https://github.com/vi/websocat/releases/latest/download/websocat.x86_64-unknown-linux-musl
      sudo chmod a+x /usr/local/bin/websocat

      # Connect to your sandbox
      ssh -o 'ProxyCommand=websocat --binary -B 65536 - wss://8081-%h.e2b.app' user@<sandbox-id>
      ```
    </CodeGroup>
  </Step>
</Steps>

***

## How it works

This method uses [websocat](https://github.com/vi/websocat) to proxy SSH connections over WebSocket through the sandbox's exposed ports.

```
┌───────────────────────────────────────────────────────────┐
│  Your Machine                                             │
│  ┌──────────┐    ProxyCommand    ┌──────────────────┐     │
│  │   SSH    │ ────────────────── │    websocat      │     │
│  │  Client  │                    │   (WebSocket)    │     │
│  └──────────┘                    └─────────┬────────┘     │
└────────────────────────────────────────────┼──────────────┘
                                             │
                         wss://8081-<sandbox-id>.e2b.app
                                             │
┌────────────────────────────────────────────┼──────────────┐
│  E2B Sandbox                               ▼              │
│                                  ┌──────────────────┐     │
│                                  │    websocat      │     │
│                                  │  (WS → TCP:22)   │     │
│                                  └─────────┬────────┘     │
│                                            │              │
│                                  ┌─────────▼────────┐     │
│                                  │   SSH Server     │     │
│                                  │   (OpenSSH)      │     │
│                                  └──────────────────┘     │
└───────────────────────────────────────────────────────────┘
```
