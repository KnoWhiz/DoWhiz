> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Authentication in CLI

There are two ways to authenticate with the E2B CLI:

## Option 1: browser authentication

<Warning>
  This option requires an interactive browser environment.
</Warning>

Run the following command to sign in through your browser:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b auth login
```

This will open your default browser and prompt you to authenticate with your E2B account.

## Option 2: setup environment variables

Set your environment variables:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
export E2B_API_KEY=your_api_key_here
export E2B_ACCESS_TOKEN=your_acces_token_here
```

<Info>
  Learn more about obtaining and managing your API key and access token on the [API Key page](/docs/api-key).
</Info>
