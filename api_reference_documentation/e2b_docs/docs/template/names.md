> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Template names

> Understanding and managing template names

Template names are unique identifiers used to reference and create sandboxes from your templates. They serve as human-readable names that make it easy to identify and use your templates across your applications.

## What is a template name?

A name is a string identifier that you assign to a template when building it. Once a template is built with a name, you can use that name to create sandboxes from the template.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Build a template with a name
  await Template.build(template, 'my-python-env', {
    cpuCount: 2,
    memoryMB: 2048,
  })

  // Create a sandbox using the name
  const sandbox = await Sandbox.create('my-python-env')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Build a template with a name
  Template.build(
      template,
      'my-python-env',
      cpu_count=2,
      memory_mb=2048,
  )

  # Create a sandbox using the name
  sandbox = Sandbox.create('my-python-env')
  ```
</CodeGroup>

## Team-local naming

Template names are scoped to your team. This means:

* Your template named `my-app` is stored as `your-team-slug/my-app`
* You can reference it simply as `my-app` within your team
* Other teams can have their own `my-app` template without conflict
* Public templates should be referenced using the full namespaced format (`team-slug/template-name`)

<Info>
  **Backwards Compatibility**: Existing public templates remain accessible without the team slug prefix. New public templates should be referenced using the full namespaced format (`team-slug/template-name`).
</Info>

## Common use cases

### Development and production environments

Use different names for different environments:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Development template
  await Template.build(template, 'myapp-dev', {
    cpuCount: 1,
    memoryMB: 1024,
  })

  // Production template
  await Template.build(template, 'myapp-prod', {
    cpuCount: 4,
    memoryMB: 4096,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Development template
  Template.build(
      template,
      'myapp-dev',
      cpu_count=1,
      memory_mb=1024,
  )

  # Production template
  Template.build(
      template,
      'myapp-prod',
      cpu_count=4,
      memory_mb=4096,
  )
  ```
</CodeGroup>

### Multiple template variants

Create different variants of the same template with different configurations:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Small instance
  await Template.build(template, 'myapp-small', {
    cpuCount: 1,
    memoryMB: 512,
  })

  // Large instance
  await Template.build(template, 'myapp-large', {
    cpuCount: 8,
    memoryMB: 8192,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Small instance
  Template.build(
      template,
      'myapp-small',
      cpu_count=1,
      memory_mb=512,
  )

  # Large instance
  Template.build(
      template,
      'myapp-large',
      cpu_count=8,
      memory_mb=8192,
  )
  ```
</CodeGroup>

<Tip>
  When building variants with the same template definition but different CPU/RAM configurations, E2B's caching system will reuse common layers, making subsequent builds much faster.
</Tip>

## Checking name availability

You can check if a name is already in use within your team with the `exists` method.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template } from 'e2b'

  const exists = await Template.exists('my-template')
  console.log(`Name ${exists ? 'is taken' : 'is available'}`)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template

  exists = Template.exists('my-template')
  print(f"Name {'is taken' if exists else 'is available'}")
  ```
</CodeGroup>

## Best practices

1. **Use descriptive names**: Choose names that clearly indicate the template's purpose or configuration
2. **Use tags for versioning**: Instead of baking version numbers into names, use [tags](/docs/template/tags) for version management (e.g., `myapp:v1`, `myapp:v2`)
3. **Use consistent naming**: Establish a naming convention for your team and stick to it
