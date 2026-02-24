> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# User and workdir

> Default user and working directory in the sandbox and template

The default user in the template is `user` with the `/home/user` (home directory) as the working directory.
This is different from the Docker defaults, where the default user is `root` with `/` as the working directory. This is to help with tools installation, and to improve default security.

The last set user and workdir in the template is then persisted as a default to the sandbox execution.
Example of setting user and workdir in the template definition are below.

<Info>
  Requires the E2B SDK version at least 2.3.0
</Info>

## Default user and workdir in sandbox

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const sbx = await Sandbox.create()
  await sbx.commands.run("whoami") // user
  await sbx.commands.run("pwd") // /home/user
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  sbx = Sandbox.create()
  sbx.commands.run("whoami")  # user
  sbx.commands.run("pwd")  # /home/user
  ```
</CodeGroup>

## Custom user and workdir template

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  const template = Template()
    .fromBaseImage()
    .runCmd("whoami") // user
    .runCmd("pwd") // /home/user
    .setUser("guest")
    .runCmd("whoami") // guest
    .runCmd("pwd") // /home/guest


  // build_dev.ts
  await Template.build(template, 'custom-user-template', {
    onBuildLogs: defaultBuildLogger()
  })


  // index.ts
  const sbx = await Sandbox.create("custom-user-template")
  await sbx.commands.run("whoami") // guest
  await sbx.commands.run("pwd") // /home/guest
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  template = (
      Template()
      .from_base_image()
      .run_cmd("whoami") # user
      .run_cmd("pwd") # /home/user
      .set_user("guest")
      .run_cmd("whoami") # guest
      .run_cmd("pwd") # /home/guest
  )


  # build_dev.py
  Template.build(template, 'custom-user-template',
      on_build_logs=default_build_logger()
  )


  # main.py
  sbx = Sandbox.create("custom-user-template")
  sbx.commands.run("whoami")  # guest
  sbx.commands.run("pwd")  # /home/guest
  ```
</CodeGroup>
