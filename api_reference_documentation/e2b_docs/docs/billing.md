> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Billing & pricing

E2B uses [usage-based pricing](#usage-based-pricing) - you pay only for what you use. New users receive \$100 in free credits to get started.

[Manage billing in dashboard](https://e2b.dev/dashboard?tab=billing)

## Plans

| Feature                  | Hobby            | Pro              | Enterprise |
| ------------------------ | ---------------- | ---------------- | ---------- |
| **Base price**           | \$0/month        | \$150/month      | Custom     |
| **Free credits**         | \$100 (one-time) | \$100 (one-time) | Custom     |
| **Max session length**   | 1 hour           | 24 hours         | Custom     |
| **Concurrent sandboxes** | 20               | 100 - 1,100      | 1,100+     |

<Note>
  Pro plan includes 100 concurrent sandboxes. Higher concurrency up to 1,100 is available as a purchasable [add-on](https://e2b.dev/dashboard?tab=billing).
</Note>

Plans have different [API rate limits](/docs/sandbox/rate-limits).

To upgrade your plan or purchase add-ons, visit the [dashboard billing tab](https://e2b.dev/dashboard?tab=billing). For Enterprise plans, [contact sales](mailto:enterprise@e2b.dev).

***

## Usage-based pricing

You pay per second for compute resources while your sandbox is running.

### Compute costs

Use the [usage cost calculator](https://e2b.dev/pricing#:~:text=Usage%20Cost%20Calculator) on our pricing page to estimate costs for your specific configuration.

### Customizing compute resources

You can customize allocated CPU and RAM when building custom templates by specifying `cpuCount` and `memoryMB` in the build configuration.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template, defaultBuildLogger } from 'e2b'

  await Template.build(template, 'my-template', {
    cpuCount: 8,
    memoryMB: 4096,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template, default_build_logger

  Template.build(
      template,
      'my-template',
      cpu_count=8,
      memory_mb=4096,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>

See [template quickstart](/docs/template/quickstart) for more details on building custom templates.

<Note>
  Need higher CPU or RAM limits? Contact [support@e2b.dev](mailto:support@e2b.dev) for more information.
</Note>

***

## Monitoring usage

Check your usage and costs in the [dashboard usage tab](https://e2b.dev/dashboard?tab=usage).

***

## FAQ

<AccordionGroup>
  <Accordion title="When am I charged?">
    Automatically at the start of the month for the previous month's usage.
  </Accordion>

  <Accordion title="What happens when I run out of credits?">
    Your account will be blocked. Add a payment method to continue using E2B.
  </Accordion>

  <Accordion title="Can I set spending limits?">
    Yes, you can set spending limits on the [budget page](https://e2b.dev/dashboard?tab=budget) in your dashboard.
  </Accordion>

  <Accordion title="How do I optimize costs?">
    * **Always kill sandboxes when done** - Use `sbx.kill()` to stop billing immediately
    * **Enable autopause** - Automatically pause sandboxes after a period of inactivity to stop billing while preserving state
    * **Allocate only what you need** - Start with default resources (2 vCPU, 1 GB RAM) and increase only if necessary
    * **Implement automatic timeouts** - Set max session lengths to prevent forgotten sandboxes from running
    * **Monitor actively running sandboxes** - Use the [CLI](/docs/cli/list-sandboxes) or [dashboard](https://e2b.dev/dashboard?tab=usage) to track active sandboxes
    * **Use lifecycle events** - Set up [webhooks](/docs/sandbox/lifecycle-events-webhooks) to get notified when sandboxes are created
  </Accordion>

  <Accordion title="Do I pay for stopped sandboxes?">
    No. You only pay while a sandbox is actively running. Once a sandbox is paused, killed or times out, billing stops immediately.
  </Accordion>
</AccordionGroup>
