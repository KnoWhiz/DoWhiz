> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Template tags & versioning

> Use tags to version your templates and manage environment-based deployments

Template versioning allows you to maintain multiple versions of the same template using tags. This enables workflows like semantic versioning, environment-based deployments, and gradual rollouts.

## Tag format

Tags follow the `name:tag` format, where `name` is your template's identifier and `tag` is the version label.

```
my-template:v1.0.0              // Within your team
my-template:production          // Within your team
acme/my-template:v1.0.0         // Full namespaced reference
```

## The default tag

When you build or reference a template without specifying a tag, E2B uses the `default` tag automatically. This means:

* `my-template` is equivalent to `my-template:default`
* Existing templates without tags continue to work seamlessly

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // These are equivalent
  const sandbox1 = await Sandbox.create('my-template')
  const sandbox2 = await Sandbox.create('my-template:default')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # These are equivalent
  sandbox1 = Sandbox.create('my-template')
  sandbox2 = Sandbox.create('my-template:default')
  ```
</CodeGroup>

## Building with tags

You can build templates with one or more tags to create versioned builds.

### Single tag

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template } from 'e2b'

  // Build with a specific version tag
  await Template.build(template, 'my-template:v1.0.0')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template

  # Build with a specific version tag
  Template.build(template, 'my-template:v1.0.0')
  ```
</CodeGroup>

### Multiple tags

Build with multiple tags to assign several version labels to the same build artifact.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template } from 'e2b'

  // Build with multiple tags pointing to the same artifact
  await Template.build(template, 'my-template', { tags: ['v1.2.0', 'latest'] })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template

  # Build with multiple tags pointing to the same artifact
  Template.build(template, 'my-template', tags=['v1.2.0', 'latest'])
  ```
</CodeGroup>

## Managing tags

You can manage tags on existing template builds without rebuilding.

### Assign tags

Assign new tag(s) to an existing build. This is useful for promoting a tested version to production or marking a version as stable.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template } from 'e2b'

  // Assign a single tag
  await Template.assignTags('my-template:v1.2.0', 'production')

  // Assign multiple tags at once
  await Template.assignTags('my-template:v1.2.0', ['production', 'stable'])
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template

  # Assign a single tag
  Template.assign_tags('my-template:v1.2.0', 'production')

  # Assign multiple tags at once
  Template.assign_tags('my-template:v1.2.0', tags=['production', 'stable'])
  ```
</CodeGroup>

### Remove tags

Remove a tag from a template. The underlying build artifact remains accessible via other tags.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template } from 'e2b'

  // Remove a tag
  await Template.removeTags('my-template', 'staging')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template

  # Remove a tag
  Template.remove_tags('my-template', 'staging')
  ```
</CodeGroup>

<Warning>
  Removing a tag does not delete the build artifact. Other tags pointing to the same build will continue to work.
</Warning>

## Use cases

### Semantic versioning

Use semantic version tags to track releases and enable rollbacks.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Release versions
  await Template.build(template, 'api-server:v1.0.0')
  await Template.build(template, 'api-server:v1.1.0')
  await Template.build(template, 'api-server:v2.0.0')

  // Create sandbox from specific version
  const sandbox = await Sandbox.create('api-server:v1.1.0')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Release versions
  Template.build(template, 'api-server:v1.0.0')
  Template.build(template, 'api-server:v1.1.0')
  Template.build(template, 'api-server:v2.0.0')

  # Create sandbox from specific version
  sandbox = Sandbox.create('api-server:v1.1.0')
  ```
</CodeGroup>

### Environment tags

Use environment tags for deployment pipelines.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Build new version
  await Template.build(template, 'my-app:v1.5.0')

  // Promote through environments
  await Template.assignTags('my-app:v1.5.0', 'staging')

  // After testing, promote to production
  await Template.assignTags('my-app:v1.5.0', 'production')

  // Use in your application
  const env = process.env.NODE_ENV
  const sandbox = await Sandbox.create(`my-app:${env}`)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os

  # Build new version
  Template.build(template, 'my-app:v1.5.0')

  # Promote through environments
  Template.assign_tags('my-app:v1.5.0', 'staging')

  # After testing, promote to production
  Template.assign_tags('my-app:v1.5.0', 'production')

  # Use in your application
  env = os.environ.get('ENV', 'staging')
  sandbox = Sandbox.create(f'my-app:{env}')
  ```
</CodeGroup>

### Latest and stable tags

Maintain rolling tags that always point to specific versions.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Build with version and latest tag
  await Template.build(template, 'my-tool', { tags: ['v3.0.0', 'latest'] })

  // Mark a tested version as stable
  await Template.assignTags('my-tool:v2.9.0', 'stable')

  // You can choose your risk tolerance
  const latestSandbox = await Sandbox.create('my-tool:latest')  // Newest
  const stableSandbox = await Sandbox.create('my-tool:stable')  // Tested
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Build with version and latest tag
  Template.build(template, 'my-tool', tags=['v3.0.0', 'latest'])

  # Mark a tested version as stable
  Template.assign_tags('my-tool:v2.9.0', 'stable')

  # You can choose your risk tolerance
  latest_sandbox = Sandbox.create('my-tool:latest')  # Newest
  stable_sandbox = Sandbox.create('my-tool:stable')  # Tested
  ```
</CodeGroup>
