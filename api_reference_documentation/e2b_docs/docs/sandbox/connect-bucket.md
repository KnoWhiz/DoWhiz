> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Connecting storage bucket to the sandbox

To connect a bucket for storing data from the sandbox, we will use the FUSE file system to mount the bucket to the sandbox.

You will need to create a custom sandbox template with the FUSE file system installed. The guide for building a custom sandbox template can be found [here](/docs/template/quickstart).

## Google Cloud Storage

### Prerequisites

To use Google Cloud Storage, you'll need a bucket and a service account. You can create a service account [here](https://console.cloud.google.com/iam-admin/serviceaccounts) and a bucket [here](https://console.cloud.google.com/storage).

If you want to write to the bucket, make sure the service account has the `Storage Object User` role for this bucket.

You can find a guide on creating a service account key [here](https://cloud.google.com/iam/docs/keys-create-delete#iam-service-account-keys-create-console).

### Mounting the bucket

To use the Google Cloud Storage we need to install the `gcsfuse` package. There's simple [template](/docs/template/quickstart) that can be used to create a container with the `gcsfuse` installed.

<CodeGroup>
  ```ts JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .fromTemplate("code-interpreter-v1")
    .aptInstall(["gnupg", "lsb-release"])
    .runCmd("lsb_release -c -s > /tmp/lsb_release")
    .runCmd(
      'GCSFUSE_REPO=$(cat /tmp/lsb_release) && echo "deb [signed-by=/usr/share/keyrings/cloud.google.asc] https://packages.cloud.google.com/apt gcsfuse-$GCSFUSE_REPO main" | sudo tee /etc/apt/sources.list.d/gcsfuse.list',
    )
    .runCmd(
      "curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo tee /usr/share/keyrings/cloud.google.asc",
    )
    .aptInstall(["gcsfuse"])
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .from_template("code-interpreter-v1")
      .apt_install(["gnupg", "lsb-release"])
      .run_cmd("lsb_release -c -s > /tmp/lsb_release")
      .run_cmd(
          'GCSFUSE_REPO=$(cat /tmp/lsb_release) && echo "deb [signed-by=/usr/share/keyrings/cloud.google.asc] https://packages.cloud.google.com/apt gcsfuse-$GCSFUSE_REPO main" | sudo tee /etc/apt/sources.list.d/gcsfuse.list'
      )
      .run_cmd(
          "curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo tee /usr/share/keyrings/cloud.google.asc"
      )
      .apt_install(["gcsfuse"])
  )
  ```
</CodeGroup>

The bucket is mounted during the sandbox runtime using the `gcsfuse` command.

<CodeGroup isRunnable={false}>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('<your template id>')
  await sandbox.files.makeDir('/home/user/bucket')
  await sandbox.files.write('key.json', '<your service account key>')

  await sandbox.commands.run('sudo gcsfuse <flags> --key-file /home/user/key.json <bucket-name> /home/user/bucket')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox

  sandbox = Sandbox.create("<your template id>")
  sandbox.files.make_dir("/home/user/bucket")
  sandbox.files.write("key.json", "<your key file content>")

  output = sandbox.commands.run(
      "sudo gcsfuse <flags> --key-file /home/user/key.json <bucket-name> /home/user/bucket"
  )
  ```
</CodeGroup>

### Flags

The complete list of flags is available [here](https://cloud.google.com/storage/docs/gcsfuse-cli#options).

### Allow the default user to access the files

To allow the default user to access the files, we can use the following flags:

```
-o allow_other -file-mode=777 -dir-mode=777
```

## Amazon S3

To use Amazon S3, we can use the `s3fs` package. The [template](/docs/template/quickstart) setup is similar to that of Google Cloud Storage.

<CodeGroup>
  ```ts JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .fromImage("ubuntu:latest")
    .aptInstall(["s3fs"])
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .from_image("ubuntu:latest")
      .apt_install(["s3fs"])
  )
  ```
</CodeGroup>

Similar to Google Cloud Storage, the bucket is mounted during the runtime of the sandbox. The `s3fs` command is used to mount the bucket to the sandbox.

<CodeGroup isRunnable={false}>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('<your template id>')
  await sandbox.files.makeDir('/home/user/bucket')

  // Create a file with the credentials
  // If you use another path for the credentials you need to add the path in the command s3fs command
  await sandbox.files.write('/root/.passwd-s3fs', '<AWS_ACCESS_KEY_ID>:<AWS_SECRET_ACCESS_KEY>')
  await sandbox.commands.run('sudo chmod 600 /root/.passwd-s3fs')

  await sandbox.commands.run('sudo s3fs <flags> <bucket-name> /home/user/bucket')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox

  sandbox = Sandbox.create("<your template id>")
  sandbox.files.make_dir("/home/user/bucket")

  # Create a file with the credentials
  # If you use another path for the credentials you need to add the path in the command s3fs command
  sandbox.files.write("/root/.passwd-s3fs", "<AWS_ACCESS_KEY_ID>:<AWS_SECRET_ACCESS_KEY>")
  sandbox.commands.run("sudo chmod 600 /root/.passwd-s3fs")

  sandbox.commands.run("sudo s3fs <flags> <bucket-name> /home/user/bucket")
  ```
</CodeGroup>

### Flags

The complete list of flags is available [here](https://manpages.ubuntu.com/manpages/xenial/man1/s3fs.1.html).

### Allow the default user to access the files

To allow the default user to access the files, add the following flag:

```
-o allow_other
```

## Cloudflare R2

For Cloudflare R2, we can use a setup very similar to S3. The template remains the same as for S3. However, the mounting differs slightly; we need to specify the endpoint for R2.

<CodeGroup isRunnable={false}>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create({ template: '<your template id>' })
  await sandbox.files.makeDir('/home/user/bucket')

  // Create a file with the R2 credentials
  // If you use another path for the credentials you need to add the path in the command s3fs command
  await sandbox.files.write('/root/.passwd-s3fs', '<R2_ACCESS_KEY_ID>:<R2_SECRET_ACCESS_KEY>')
  await sandbox.commands.run('sudo chmod 600 /root/.passwd-s3fs')

  await sandbox.commands.run('sudo s3fs -o url=https://<ACCOUNT ID>.r2.cloudflarestorage.com <flags> <bucket-name> /home/user/bucket')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox

  sandbox = Sandbox.create("<your template id>")
  sandbox.files.make_dir("/home/user/bucket")

  # Create a file with the R2 credentials
  # If you use another path for the credentials you need to add the path in the command s3fs command
  sandbox.files.write("/root/.passwd-s3fs", "<R2_ACCESS_KEY_ID>:<R2_SECRET_ACCESS_KEY>")
  sandbox.commands.run("sudo chmod 600 /root/.passwd-s3fs")

  sandbox.commands.run(
      "sudo s3fs -o url=https://<ACCOUNT ID>.r2.cloudflarestorage.com <flags> <bucket-name> /home/user/bucket"
  )
  ```
</CodeGroup>

### Flags

It's the same as for S3.
