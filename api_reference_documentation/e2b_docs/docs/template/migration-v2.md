> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# V2 migration guide

> How to migrate from the legacy template definition

We've made ready for you three ways how to migrate existing template to the new definition format.

* Using the migration command (recommended)
* Using the `fromDockerfile()` method to parse existing Dockerfile
* Building the Docker image manually and using `fromImage()` method

<Warning>
  *This is handled for you automatically when using any of the options below.*

  The default user for the template is `user` with the working directory set to the user home directory. This is a difference from the Docker defaults.
</Warning>

## Migration command (recommended)

To migrate the existing template definition to the new format, follow these steps:

<Steps>
  <Step title="Install E2B CLI">
    Install the latest version of the [E2B CLI](https://e2b.dev/docs/cli)
  </Step>

  <Step title="Navigate to your template folder">
    Navigate to the folder of your existing template definition (where you have `e2b.toml` and `e2b.Dockerfile` files).
  </Step>

  <Step title="Run migration command">
    <Note>
      Your existing `e2b.toml` and `e2b.Dockerfile` files will be renamed to `e2b.toml.old` and `e2b.Dockerfile.old`, respectively, as they are no longer required.
    </Note>

    ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
    e2b template migrate
    ```
  </Step>

  <Step title="Follow the prompts">
    Follow the prompts to complete the migration process.
  </Step>

  <Step title="Done" />
</Steps>

### Generated output

The migration command generates three files (based on the selected language).

<CodeGroup>
  ```markdown JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.ts - Template definition using the SDK
  build.dev.ts - Development build script
  build.prod.ts - Production build script
  ```

  ```markdown Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.py - Template definition using the SDK
  build_dev.py - Development build script
  build_prod.py - Production build script
  ```
</CodeGroup>

## Using `fromDockerfile()` Method

If you want to keep using Dockerfile for your template, you can use the `fromDockerfile()` method.
We'll automatically parse the Dockerfile for you and build the template based on it.
You can find more at [Base Image](/docs/template/base-image#parsing-existing-dockerfiles).

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b';

  const template = Template()
    .fromDockerfile(dockerfileContent);
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_dockerfile(dockerfile_content)
  )
  ```
</CodeGroup>

<Info>
  Only a limited set of instructions are supported and converted to equivalent template.

  Compatible instructions:

  `FROM`, `RUN`, `COPY`, `ADD`, `WORKDIR`, `USER`, `ENV`, `ARG`, `CMD`, `ENTRYPOINT`
</Info>

After the template definition, you can create the build scripts as described in the [Quickstart](/docs/template/quickstart#create-a-development-build-script) section.

## Using `fromImage()` Method

If you already have a Docker image built, or you want to keep building the Docker image yourself you can use the `fromImage()` method.
Simply build the Docker image for the `linux/amd64` platform, push it to a registry of your choice and reference the image tag in the `fromImage()` method.
You can also specify credentials for private registries as described in the [Base Image](/docs/template/base-image#custom-base-image) section.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b';

  const template = Template()
    .fromImage("image-tag");
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_image("image-tag")
  )
  ```
</CodeGroup>
