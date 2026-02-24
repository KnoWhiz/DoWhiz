> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Base image

> How to define a base image for your template

## Creating template

When creating a template, you can specify options:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template({
    fileContextPath: ".", // Custom file context path
    fileIgnorePatterns: [".git", "node_modules"], // File patterns to ignore
  });
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = Template(
      file_context_path=".",  # Custom file context path
      file_ignore_patterns=[".git", "node_modules"],  # File patterns to ignore
  )
  ```
</CodeGroup>

**File ignoring**: The SDK automatically reads `.dockerignore` files and combines them with your `fileIgnorePatterns` (TypeScript) or `file_ignore_patterns` (Python). Files matching these patterns are excluded from uploads and hash calculations.

## Defining base image

Every template starts with a base image that provides the foundation for your sandbox environment.

### Predefined base images

Use convenience methods for common base images with Ubuntu, Debian, Python, Node.js, or Bun:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.fromUbuntuImage("22.04"); // ubuntu:22.04
  template.fromDebianImage("stable-slim"); // debian:stable-slim
  template.fromPythonImage("3.13"); // python:3.13
  template.fromNodeImage("lts"); // node:lts
  template.fromBunImage("1.3"); // oven/bun:1.3
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.from_ubuntu_image("22.04")  # ubuntu:22.04
  template.from_debian_image("stable-slim")  # debian:stable-slim
  template.from_python_image("3.13")  # python:3.13
  template.from_node_image("lts")  # node:lts
  template.from_bun_image("1.3")  # oven/bun:1.3
  ```
</CodeGroup>

### Custom base image

Use any Docker image from Docker Hub or other registries:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.fromImage("custom-image:latest");
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.from_image("custom-image:latest")
  ```
</CodeGroup>

### Default E2B base image

Use the default E2B base image, which comes pre-configured for sandbox environments:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.fromBaseImage(); // e2bdev/base
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.from_base_image()  # e2bdev/base
  ```
</CodeGroup>

### Build from existing template

Extend an existing template from your team or organization:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.fromTemplate("my-template"); // Your team's template
  template.fromTemplate("acme/other-template"); // Full namespaced reference
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.from_template("my-template")  # Your team's template
  template.from_template("acme/other-template")  # Full namespaced reference
  ```
</CodeGroup>

<Note>
  You can only call base image methods once per template. Subsequent calls will throw an error.
</Note>

## Parsing existing Dockerfiles

Convert existing Dockerfiles to template format using `fromDockerfile()`:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const dockerfileContent = `
  FROM ubuntu:22.04
  RUN apt-get update && apt-get install -y curl
  WORKDIR /app
  COPY . .
  ENV NODE_ENV=production
  ENV PORT=3000
  USER appuser`;

  const template = Template()
    .fromDockerfile(dockerfileContent)
    .setStartCmd("npm start", waitForTimeout(5_000));
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  dockerfile_content = """
  FROM ubuntu:22.04
  RUN apt-get update && apt-get install -y curl
  WORKDIR /app
  COPY . .
  ENV NODE_ENV=production
  ENV PORT=3000
  USER appuser
  """

  template = (
      Template()
      .from_dockerfile(dockerfile_content)
      .set_start_cmd("npm start", wait_for_timeout(5_000))
  )
  ```
</CodeGroup>

### Dockerfile instructions support

| Instruction          |                   Supported                   | Behavior                                                                                          |
| :------------------- | :-------------------------------------------: | :------------------------------------------------------------------------------------------------ |
| `FROM`               | <Icon icon="square-check" iconType="solid" /> | Sets base image                                                                                   |
| `RUN`                | <Icon icon="square-check" iconType="solid" /> | Converts to `runCmd()` / `run_cmd()`                                                              |
| `COPY` / `ADD`       | <Icon icon="square-check" iconType="solid" /> | Converts to `copy()`                                                                              |
| `WORKDIR`            | <Icon icon="square-check" iconType="solid" /> | Converts to `setWorkdir()` / `set_workdir()`                                                      |
| `USER`               | <Icon icon="square-check" iconType="solid" /> | Converts to `setUser()` / `set_user()`                                                            |
| `ENV`                | <Icon icon="square-check" iconType="solid" /> | Converts to `setEnvs()` / `set_envs()`; supports both `ENV key=value` and `ENV key value` formats |
| `CMD` / `ENTRYPOINT` | <Icon icon="square-check" iconType="solid" /> | Converts to `setStartCmd()` / `set_start_cmd()` with 20 seconds timeout as ready command          |
| `EXPOSE`             |     <Icon icon="xmark" iconType="solid" />    | Skipped (not supported)                                                                           |
| `VOLUME`             |     <Icon icon="xmark" iconType="solid" />    | Skipped (not supported)                                                                           |

<Warning>
  Multi-stage Dockerfiles are not supported.
</Warning>
