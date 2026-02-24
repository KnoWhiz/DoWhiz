> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Private registries

> Access private registries as the base image

If your base image is hosted in a private registry, you can provide credentials using the following helpers:

* General registry
* GCP Artifact Registry
* AWS ECR

## General registry

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Template().fromImage('ubuntu:22.04', {
    username: 'user',
    password: 'pass',
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Template().from_image(
      image="ubuntu:22.04",
      username="user",
      password="pass",
  )
  ```
</CodeGroup>

## GCP Artifact Registry

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // From file path
  Template().fromGCPRegistry('ubuntu:22.04', {
    serviceAccountJSON: './service_account.json',
  })

  // From object
  Template().fromGCPRegistry('ubuntu:22.04', {
    serviceAccountJSON: { project_id: '123', private_key_id: '456' },
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # From file path
  Template().from_gcp_registry(
      image="ubuntu:22.04",
      service_account_json="./service_account.json",
  )

  # From object
  Template().from_gcp_registry(
      image="ubuntu:22.04",
      service_account_json={"project_id": "123", "private_key_id": "456"},
  )
  ```
</CodeGroup>

## AWS ECR

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Template().fromAWSRegistry('ubuntu:22.04', {
    accessKeyId: '123',
    secretAccessKey: '456',
    region: 'us-west-1',
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Template().from_aws_registry(
      image="ubuntu:22.04",
      access_key_id="123",
      secret_access_key="456",
      region="us-west-1",
  )
  ```
</CodeGroup>
