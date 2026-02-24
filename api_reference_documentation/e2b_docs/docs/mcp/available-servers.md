> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Available servers

> Browse available MCP servers

E2B provides access to 200+ MCP servers from [Docker's catalog](https://hub.docker.com/mcp). You can also run [custom MCP servers](/docs/mcp/custom-servers) inside the sandbox.

## Airtable

Provides AI assistants with direct access to Airtable bases, allowing them to read schemas, query records, and interact with your Airtable data. Supports listing bases, retrieving table structures, and searching through records to help automate workflows and answer questions about your organized data.

[View on Docker Hub](https://hub.docker.com/mcp/server/airtable-mcp-server/overview)

<ResponseField name="airtable" type="object">
  <Expandable title="properties">
    <ResponseField name="airtableApiKey" type="string" required />

    <ResponseField name="nodeenv" type="string" required />
  </Expandable>
</ResponseField>

## Azure Kubernetes Service (AKS)

Azure Kubernetes Service (AKS) official MCP server.

[View on Docker Hub](https://hub.docker.com/mcp/server/aks/overview)

<ResponseField name="aks" type="object">
  <Expandable title="properties">
    <ResponseField name="accessLevel" type="string" required>
      Access level for the MCP server, One of \[ readonly, readwrite, admin ]
    </ResponseField>

    <ResponseField name="additionalTools" type="string">
      Comma-separated list of additional tools, One of \[ helm, cilium ]
    </ResponseField>

    <ResponseField name="allowNamespaces" type="string">
      Comma-separated list of namespaces to allow access to. If not specified, all namespaces are allowed.
    </ResponseField>

    <ResponseField name="azureDir" type="string" required>
      Path to the Azure configuration directory (e.g. /home/azureuser/.azure). Used for Azure CLI authentication, you should be logged in (e.g. run `az login`) on the host before starting the MCP server.
    </ResponseField>

    <ResponseField name="containerUser" type="string">
      Username or UID of the container user (format `<name|uid>`\[:`<group|gid>`] e.g. 10000), ensuring correct permissions to access the Azure and kubeconfig files. Leave empty to use default user in the container.
    </ResponseField>

    <ResponseField name="kubeconfig" type="string" required>
      Path to the kubeconfig file for the AKS cluster (e.g. /home/azureuser/.kube/config). Used to connect to the AKS cluster.
    </ResponseField>
  </Expandable>
</ResponseField>

## Apify

Apify is the world's largest marketplace of tools for web scraping, data extraction, and web automation. You can extract structured data from social media, e-commerce, search engines, maps, travel sites, or any other website.

[View on Docker Hub](https://hub.docker.com/mcp/server/apify-mcp-server/overview)

<ResponseField name="apify" type="object">
  <Expandable title="properties">
    <ResponseField name="apifyToken" type="string" required />

    <ResponseField name="tools" type="string" required>
      Comma-separated list of tools to enable. Can be either a tool category, a specific tool, or an Apify Actor. For example: "actors,docs,apify/rag-web-browser". For more details visit [https://mcp.apify.com](https://mcp.apify.com).
    </ResponseField>
  </Expandable>
</ResponseField>

## Api-gateway

A universal MCP (Model Context Protocol) server to integrate any API with Claude Desktop using only Docker configurations.

[View on Docker Hub](https://hub.docker.com/mcp/server/mcp-api-gateway/overview)

<ResponseField name="apiGateway" type="object">
  <Expandable title="properties">
    <ResponseField name="api1HeaderAuthorization" type="string" required />

    <ResponseField name="api1Name" type="string" required />

    <ResponseField name="api1SwaggerUrl" type="string" required />
  </Expandable>
</ResponseField>

## ArXiv

The ArXiv MCP Server provides a comprehensive bridge between AI assistants and arXiv's research repository through the Model Context Protocol (MCP).

**Features:**

* Search arXiv papers with advanced filtering
* Download and store papers locally as markdown
* Read and analyze paper content
* Deep research analysis prompts
* Local paper management and storage
* Enhanced tool descriptions optimized for local AI models
* Docker MCP Gateway compatible with detailed context

[View on Docker Hub](https://hub.docker.com/mcp/server/arxiv-mcp-server/overview)

<ResponseField name="arxiv" type="object">
  <Expandable title="properties">
    <ResponseField name="storagePath" type="string" required>
      Directory path where downloaded papers will be stored
    </ResponseField>
  </Expandable>
</ResponseField>

The ArXiv MCP Server provides a comprehensive bridge between AI assistants and arXiv's research repository through the Model Context Protocol (MCP).   Features: • Search arXiv papers with advanced filtering • Download and store papers locally as markdown • Read and analyze paper content • Deep research analysis prompts • Local paper management and storage • Enhanced tool descriptions optimized for local AI models • Docker MCP Gateway compatible with detailed context  Perfect for researchers, academics, and AI assistants conducting literature reviews and research analysis.  **Recent Update**: Enhanced tool descriptions specifically designed to resolve local AI model confusion and improve Docker MCP Gateway compatibility.

## ast-grep

ast-grep is a fast and polyglot tool for code structural search, lint, rewriting at large scale.

[View on Docker Hub](https://hub.docker.com/mcp/server/ast-grep/overview)

<ResponseField name="astGrep" type="object">
  <Expandable title="properties">
    <ResponseField name="path" type="string" required />
  </Expandable>
</ResponseField>

## Astra DB

An MCP server for Astra DB workloads.

[View on Docker Hub](https://hub.docker.com/mcp/server/astra-db/overview)

<ResponseField name="astraDb" type="object">
  <Expandable title="properties">
    <ResponseField name="astraDbApplicationToken" type="string" required />

    <ResponseField name="endpoint" type="string" required />
  </Expandable>
</ResponseField>

## Astro Docs

Access the latest Astro web framework documentation, guides, and API references.

[View on Docker Hub](https://hub.docker.com/mcp/server/astro-docs/overview)

<ResponseField name="astroDocs" type="object" />

## Atlan

MCP server for interacting with Atlan services including asset search, updates, and lineage traversal for comprehensive data governance and discovery.

[View on Docker Hub](https://hub.docker.com/mcp/server/atlan/overview)

<ResponseField name="atlan" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="baseUrl" type="string" required />
  </Expandable>
</ResponseField>

## Atlas Docs

Provide LLMs hosted, clean markdown documentation of libraries and frameworks.

[View on Docker Hub](https://hub.docker.com/mcp/server/atlas-docs/overview)

<ResponseField name="atlasDocs" type="object">
  <Expandable title="properties">
    <ResponseField name="apiUrl" type="string" required />
  </Expandable>
</ResponseField>

## Atlassian

Tools for Atlassian products (Confluence and Jira). This integration supports both Atlassian Cloud and Jira Server/Data Center deployments.

[View on Docker Hub](https://hub.docker.com/mcp/server/atlassian/overview)

<ResponseField name="atlassian" type="object">
  <Expandable title="properties">
    <ResponseField name="confluenceApiToken" type="string" />

    <ResponseField name="confluencePersonalToken" type="string" />

    <ResponseField name="confluenceUrl" type="string" required />

    <ResponseField name="confluenceUsername" type="string" />

    <ResponseField name="jiraApiToken" type="string" />

    <ResponseField name="jiraPersonalToken" type="string" />

    <ResponseField name="jiraUrl" type="string" required />

    <ResponseField name="jiraUsername" type="string" />
  </Expandable>
</ResponseField>

## Audiense Insights

Audiense Insights MCP Server is a server based on the Model Context Protocol (MCP) that allows Claude and other MCP-compatible clients to interact with your Audiense Insights account.

[View on Docker Hub](https://hub.docker.com/mcp/server/audiense-insights/overview)

<ResponseField name="audienseInsights" type="object">
  <Expandable title="properties">
    <ResponseField name="audienseClientSecret" type="string" />

    <ResponseField name="clientId" type="string" required />

    <ResponseField name="twitterBearerToken" type="string" />
  </Expandable>
</ResponseField>

## AWS CDK

AWS Cloud Development Kit (CDK) best practices, infrastructure as code patterns, and security compliance with CDK Nag.

[View on Docker Hub](https://hub.docker.com/mcp/server/aws-cdk-mcp-server/overview)

<ResponseField name="awsCdk" type="object" />

## AWS Core

Starting point for using the awslabs MCP servers.

[View on Docker Hub](https://hub.docker.com/mcp/server/aws-core-mcp-server/overview)

<ResponseField name="awsCore" type="object" />

## AWS Diagram

Seamlessly create diagrams using the Python diagrams package DSL. This server allows you to generate AWS diagrams, sequence diagrams, flow diagrams, and class diagrams using Python code.

[View on Docker Hub](https://hub.docker.com/mcp/server/aws-diagram/overview)

<ResponseField name="awsDiagram" type="object" />

## AWS Documentation

Tools to access AWS documentation, search for content, and get recommendations.

[View on Docker Hub](https://hub.docker.com/mcp/server/aws-documentation/overview)

<ResponseField name="awsDocumentation" type="object" />

## AWS KB Retrieval (Archived)

An MCP server implementation for retrieving information from the AWS Knowledge Base using the Bedrock Agent Runtime.

[View on Docker Hub](https://hub.docker.com/mcp/server/aws-kb-retrieval-server/overview)

<ResponseField name="awsKbRetrievalServer" type="object">
  <Expandable title="properties">
    <ResponseField name="accessKeyId" type="string" required />

    <ResponseField name="awsSecretAccessKey" type="string" />
  </Expandable>
</ResponseField>

## AWS Terraform

Terraform on AWS best practices, infrastructure as code patterns, and security compliance with Checkov.

[View on Docker Hub](https://hub.docker.com/mcp/server/aws-terraform/overview)

<ResponseField name="awsTerraform" type="object" />

## Azure

The Azure MCP Server, bringing the power of Azure to your agents.

[View on Docker Hub](https://hub.docker.com/mcp/server/azure/overview)

<ResponseField name="azure" type="object" />

## Beagle security

Connects with the Beagle Security backend using a user token to manage applications, run automated security tests, track vulnerabilities across environments, and gain intelligence from Application and API vulnerability data.

[View on Docker Hub](https://hub.docker.com/mcp/server/beagle-security/overview)

<ResponseField name="beagleSecurity" type="object">
  <Expandable title="properties">
    <ResponseField name="beagleSecurityApiToken" type="string" required />
  </Expandable>
</ResponseField>

## Bitrefill

A Model Context Protocol Server connector for Bitrefill public API, to enable AI agents to search and shop on Bitrefill.

[View on Docker Hub](https://hub.docker.com/mcp/server/bitrefill/overview)

<ResponseField name="bitrefill" type="object">
  <Expandable title="properties">
    <ResponseField name="apiId" type="string" required />

    <ResponseField name="apiSecret" type="string" />
  </Expandable>
</ResponseField>

## Box

An MCP server capable of interacting with the Box API.

[View on Docker Hub](https://hub.docker.com/mcp/server/box/overview)

<ResponseField name="box" type="object">
  <Expandable title="properties">
    <ResponseField name="clientId" type="string" required />

    <ResponseField name="clientSecret" type="string" />
  </Expandable>
</ResponseField>

## Brave Search

Search the Web for pages, images, news, videos, and more using the Brave Search API.

[View on Docker Hub](https://hub.docker.com/mcp/server/brave/overview)

<ResponseField name="brave" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Browserbase

Allow LLMs to control a browser with Browserbase and Stagehand for AI-powered web automation, intelligent data extraction, and screenshot capture.

[View on Docker Hub](https://hub.docker.com/mcp/server/browserbase/overview)

<ResponseField name="browserbase" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="geminiApiKey" type="string" required />

    <ResponseField name="projectId" type="string" required />
  </Expandable>
</ResponseField>

## Buildkite

Buildkite MCP lets agents interact with Buildkite Builds, Jobs, Logs, Packages and Test Suites.

[View on Docker Hub](https://hub.docker.com/mcp/server/buildkite/overview)

<ResponseField name="buildkite" type="object">
  <Expandable title="properties">
    <ResponseField name="apiToken" type="string" required />
  </Expandable>
</ResponseField>

## Camunda BPM process engine

Tools to interact with the Camunda 7 Community Edition Engine using the Model Context Protocol (MCP). Whether you're automating workflows, querying process instances, or integrating with external systems, Camunda MCP Server is your agentic solution for seamless interaction with Camunda.

[View on Docker Hub](https://hub.docker.com/mcp/server/camunda/overview)

<ResponseField name="camunda" type="object">
  <Expandable title="properties">
    <ResponseField name="camundahost" type="string" required />
  </Expandable>
</ResponseField>

## CData Connect Cloud

This fully functional MCP Server allows you to connect to any data source in Connect Cloud from Claude Desktop.

[View on Docker Hub](https://hub.docker.com/mcp/server/cdata-connectcloud/overview)

<ResponseField name="cdataConnectcloud" type="object">
  <Expandable title="properties">
    <ResponseField name="cdataPat" type="string" />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## CharmHealth

An MCP server for CharmHealth EHR that allows LLMs and MCP clients to interact with patient records, encounters, and practice information.

[View on Docker Hub](https://hub.docker.com/mcp/server/charmhealth-mcp-server/overview)

<ResponseField name="charmhealth" type="object">
  <Expandable title="properties">
    <ResponseField name="charmhealthApiKey" type="string" required />

    <ResponseField name="charmhealthBaseUrl" type="string" required />

    <ResponseField name="charmhealthClientId" type="string" required />

    <ResponseField name="charmhealthClientSecret" type="string" required />

    <ResponseField name="charmhealthRedirectUri" type="string" required />

    <ResponseField name="charmhealthRefreshToken" type="string" required />

    <ResponseField name="charmhealthTokenUrl" type="string" required />
  </Expandable>
</ResponseField>

## Chroma

A Model Context Protocol (MCP) server implementation that provides database capabilities for Chroma.

[View on Docker Hub](https://hub.docker.com/mcp/server/chroma/overview)

<ResponseField name="chroma" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## CircleCI

A specialized server implementation for the Model Context Protocol (MCP) designed to integrate with CircleCI's development workflow. This project serves as a bridge between CircleCI's infrastructure and the Model Context Protocol, enabling enhanced AI-powered development experiences.

[View on Docker Hub](https://hub.docker.com/mcp/server/circleci/overview)

<ResponseField name="circleci" type="object">
  <Expandable title="properties">
    <ResponseField name="token" type="string" required />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Official ClickHouse

Official ClickHouse MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/clickhouse/overview)

<ResponseField name="clickhouse" type="object">
  <Expandable title="properties">
    <ResponseField name="connectTimeout" type="string" required />

    <ResponseField name="host" type="string" required />

    <ResponseField name="password" type="string" required />

    <ResponseField name="port" type="string" required />

    <ResponseField name="secure" type="string" required />

    <ResponseField name="sendReceiveTimeout" type="string" required />

    <ResponseField name="user" type="string" required />

    <ResponseField name="verify" type="string" required />
  </Expandable>
</ResponseField>

## Close

Streamline sales processes with integrated calling, email, SMS, and automated workflows for small and scaling businesses.

[View on Docker Hub](https://hub.docker.com/mcp/server/close/overview)

<ResponseField name="close" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Cloudflare Docs

Access the latest documentation on Cloudflare products such as Workers, Pages, R2, D1, KV.

[View on Docker Hub](https://hub.docker.com/mcp/server/cloudflare-docs/overview)

<ResponseField name="cloudflareDocs" type="object" />

## Cloud Run MCP

MCP server to deploy apps to Cloud Run.

[View on Docker Hub](https://hub.docker.com/mcp/server/cloud-run-mcp/overview)

<ResponseField name="cloudRun" type="object">
  <Expandable title="properties">
    <ResponseField name="credentialsPath" type="string" required>
      path to application-default credentials (eg \$HOME/.config/gcloud/application\_default\_credentials.json )
    </ResponseField>
  </Expandable>
</ResponseField>

## CockroachDB

Enable AI agents to manage, monitor, and query CockroachDB using natural language. Perform complex database operations, cluster management, and query execution seamlessly through AI-driven workflows. Integrate effortlessly with MCP clients for scalable and high-performance data operations.

[View on Docker Hub](https://hub.docker.com/mcp/server/cockroachdb/overview)

<ResponseField name="cockroachdb" type="object">
  <Expandable title="properties">
    <ResponseField name="caPath" type="string" required />

    <ResponseField name="crdbPwd" type="string" required />

    <ResponseField name="database" type="string" required />

    <ResponseField name="host" type="string" required />

    <ResponseField name="port" type="number" required />

    <ResponseField name="sslCertfile" type="string" required />

    <ResponseField name="sslKeyfile" type="string" required />

    <ResponseField name="sslMode" type="string" required />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Python Interpreter

A Python-based execution tool that mimics a Jupyter notebook environment. It accepts code snippets, executes them, and maintains state across sessions — preserving variables, imports, and past results. Ideal for iterative development, debugging, or code execution.

[View on Docker Hub](https://hub.docker.com/mcp/server/mcp-code-interpreter/overview)

<ResponseField name="codeInterpreter" type="object" />

## Context7

Context7 MCP Server -- Up-to-date code documentation for LLMs and AI code editors.

[View on Docker Hub](https://hub.docker.com/mcp/server/context7/overview)

<ResponseField name="context7" type="object" />

## Couchbase

Couchbase is a distributed document database with a powerful search engine and in-built operational and analytical capabilities.

[View on Docker Hub](https://hub.docker.com/mcp/server/couchbase/overview)

<ResponseField name="couchbase" type="object">
  <Expandable title="properties">
    <ResponseField name="cbBucketName" type="string" required>
      Bucket in the Couchbase cluster to use for the MCP server.
    </ResponseField>

    <ResponseField name="cbConnectionString" type="string" required>
      Connection string for the Couchbase cluster.
    </ResponseField>

    <ResponseField name="cbMcpReadOnlyQueryMode" type="string" required>
      Setting to "true" (default) enables read-only query mode while running SQL++ queries.
    </ResponseField>

    <ResponseField name="cbPassword" type="string" required />

    <ResponseField name="cbUsername" type="string" required>
      Username for the Couchbase cluster with access to the bucket.
    </ResponseField>
  </Expandable>
</ResponseField>

## The official for Cylera.

Brings context about device inventory, threats, risks and utilization powered by the Cylera Partner API into an LLM.

[View on Docker Hub](https://hub.docker.com/mcp/server/cylera-mcp-server/overview)

<ResponseField name="cylera" type="object">
  <Expandable title="properties">
    <ResponseField name="cyleraBaseUrl" type="string" required />

    <ResponseField name="cyleraPassword" type="string" required />

    <ResponseField name="cyleraUsername" type="string" required />
  </Expandable>
</ResponseField>

## Shodan

A Model Context Protocol server that provides access to Shodan API functionality.

[View on Docker Hub](https://hub.docker.com/mcp/server/cyreslab-ai-shodan/overview)

<ResponseField name="cyreslabAiShodan" type="object">
  <Expandable title="properties">
    <ResponseField name="shodanApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Dappier

Enable fast, free real-time web search and access premium data from trusted media brands—news, financial markets, sports, entertainment, weather, and more. Build powerful AI agents with Dappier.

[View on Docker Hub](https://hub.docker.com/mcp/server/dappier/overview)

<ResponseField name="dappier" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Dappier Remote

Enable fast, free real-time web search and access premium data from trusted media brands—news, financial markets, sports, entertainment, weather, and more. Build powerful AI agents with Dappier.

[View on Docker Hub](https://hub.docker.com/mcp/server/dappier-remote/overview)

<ResponseField name="dappierRemote" type="object">
  <Expandable title="properties">
    <ResponseField name="dappierRemoteApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Dart AI

Dart AI Model Context Protocol (MCP) server.

[View on Docker Hub](https://hub.docker.com/mcp/server/dart/overview)

<ResponseField name="dart" type="object">
  <Expandable title="properties">
    <ResponseField name="host" type="string" required />

    <ResponseField name="token" type="string" required />
  </Expandable>
</ResponseField>

## MCP Database Server

Comprehensive database server supporting PostgreSQL, MySQL, and SQLite with natural language SQL query capabilities. Enables AI agents to interact with databases through both direct SQL and natural language queries.

[View on Docker Hub](https://hub.docker.com/mcp/server/database-server/overview)

<ResponseField name="databaseServer" type="object">
  <Expandable title="properties">
    <ResponseField name="databaseUrl" type="string" required>
      Connection string for your database. Examples: SQLite: sqlite+aiosqlite:///data/mydb.db, PostgreSQL: postgresql+asyncpg://user:password\@localhost:5432/mydb, MySQL: mysql+aiomysql://user:password\@localhost:3306/mydb
    </ResponseField>
  </Expandable>
</ResponseField>

## Databutton

Databutton MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/databutton/overview)

<ResponseField name="databutton" type="object" />

## DeepWiki

Tools for fetching and asking questions about GitHub repositories.

[View on Docker Hub](https://hub.docker.com/mcp/server/deepwiki/overview)

<ResponseField name="deepwiki" type="object" />

## Descope

The Descope Model Context Protocol (MCP) server provides an interface to interact with Descope's Management APIs, enabling the search and retrieval of project-related information.

[View on Docker Hub](https://hub.docker.com/mcp/server/descope/overview)

<ResponseField name="descope" type="object">
  <Expandable title="properties">
    <ResponseField name="managementKey" type="string" />

    <ResponseField name="projectId" type="string" required />
  </Expandable>
</ResponseField>

## Desktop Commander

Search, update, manage files and run terminal commands with AI.

[View on Docker Hub](https://hub.docker.com/mcp/server/desktop-commander/overview)

<ResponseField name="desktopCommander" type="object">
  <Expandable title="properties">
    <ResponseField name="paths" type="string[]" required>
      List of directories that Desktop Commander can access
    </ResponseField>
  </Expandable>
</ResponseField>

## DevHub CMS

DevHub CMS LLM integration through the Model Context Protocol.

[View on Docker Hub](https://hub.docker.com/mcp/server/devhub-cms/overview)

<ResponseField name="devhubCms" type="object">
  <Expandable title="properties">
    <ResponseField name="devhubApiKey" type="string" />

    <ResponseField name="devhubApiSecret" type="string" />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Discord

Interact with the Discord platform.

[View on Docker Hub](https://hub.docker.com/mcp/server/mcp-discord/overview)

<ResponseField name="discord" type="object">
  <Expandable title="properties">
    <ResponseField name="discordToken" type="string" required />
  </Expandable>
</ResponseField>

## Docker Hub

Docker Hub official MCP server.

[View on Docker Hub](https://hub.docker.com/mcp/server/dockerhub/overview)

<ResponseField name="dockerhub" type="object">
  <Expandable title="properties">
    <ResponseField name="hubPatToken" type="string" required />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Dodo Payments

Tools for cross-border payments, taxes, and compliance.

[View on Docker Hub](https://hub.docker.com/mcp/server/dodo-payments/overview)

<ResponseField name="dodoPayments" type="object">
  <Expandable title="properties">
    <ResponseField name="dodoPaymentsApiKey" type="string" required />
  </Expandable>
</ResponseField>

## DreamFactory

DreamFactory is a REST API generation platform with support for hundreds of data sources, including Microsoft SQL Server, MySQL, PostgreSQL, and MongoDB. The DreamFactory MCP Server makes it easy for users to securely interact with their data sources via an MCP client.

[View on Docker Hub](https://hub.docker.com/mcp/server/dreamfactory-mcp/overview)

<ResponseField name="dreamfactory" type="object">
  <Expandable title="properties">
    <ResponseField name="dreamfactoryapikey" type="string" required />

    <ResponseField name="dreamfactoryurl" type="string" required />
  </Expandable>
</ResponseField>

## DuckDuckGo

A Model Context Protocol (MCP) server that provides web search capabilities through DuckDuckGo, with additional features for content fetching and parsing.

[View on Docker Hub](https://hub.docker.com/mcp/server/duckduckgo/overview)

<ResponseField name="duckduckgo" type="object" />

## Dynatrace

This MCP Server allows interaction with the Dynatrace observability platform, brining real-time observability data directly into your development workflow.

[View on Docker Hub](https://hub.docker.com/mcp/server/dynatrace-mcp-server/overview)

<ResponseField name="dynatrace" type="object">
  <Expandable title="properties">
    <ResponseField name="oauthClientId" type="string" required />

    <ResponseField name="oauthClientSecret" type="string" required />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## E2B

Giving Claude ability to run code with E2B via MCP (Model Context Protocol).

[View on Docker Hub](https://hub.docker.com/mcp/server/e2b/overview)

<ResponseField name="e2b" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## EduBase

The EduBase MCP server enables Claude and other LLMs to interact with EduBase's comprehensive e-learning platform through the Model Context Protocol (MCP).

[View on Docker Hub](https://hub.docker.com/mcp/server/edubase/overview)

<ResponseField name="edubase" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" />

    <ResponseField name="app" type="string" required />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Effect MCP

Tools and resources for writing Effect code in Typescript.

[View on Docker Hub](https://hub.docker.com/mcp/server/effect-mcp/overview)

<ResponseField name="effect" type="object" />

## Elasticsearch

Interact with your Elasticsearch indices through natural language conversations.

[View on Docker Hub](https://hub.docker.com/mcp/server/elasticsearch/overview)

<ResponseField name="elasticsearch" type="object">
  <Expandable title="properties">
    <ResponseField name="esApiKey" type="string" />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Elevenlabs MCP

Official ElevenLabs Model Context Protocol (MCP) server that enables interaction with powerful Text to Speech and audio processing APIs.

[View on Docker Hub](https://hub.docker.com/mcp/server/elevenlabs/overview)

<ResponseField name="elevenlabs" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" />

    <ResponseField name="data" type="string" required />
  </Expandable>
</ResponseField>

## EverArt (Archived)

Image generation server using EverArt's API.

[View on Docker Hub](https://hub.docker.com/mcp/server/everart/overview)

<ResponseField name="everart" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Exa

Exa MCP for web search and web crawling!.

[View on Docker Hub](https://hub.docker.com/mcp/server/exa/overview)

<ResponseField name="exa" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Explorium B2B Data

Discover companies, contacts, and business insights—powered by dozens of trusted external data sources.

[View on Docker Hub](https://hub.docker.com/mcp/server/explorium/overview)

<ResponseField name="explorium" type="object">
  <Expandable title="properties">
    <ResponseField name="apiAccessToken" type="string" required />
  </Expandable>
</ResponseField>

## Fetch (Reference)

Fetches a URL from the internet and extracts its contents as markdown.

[View on Docker Hub](https://hub.docker.com/mcp/server/fetch/overview)

<ResponseField name="fetch" type="object" />

## Fibery

Interact with your Fibery workspace.

[View on Docker Hub](https://hub.docker.com/mcp/server/fibery/overview)

<ResponseField name="fibery" type="object">
  <Expandable title="properties">
    <ResponseField name="apiToken" type="string" required />

    <ResponseField name="host" type="string" required />
  </Expandable>
</ResponseField>

## Filesystem (Reference)

Local filesystem access with configurable allowed paths.

[View on Docker Hub](https://hub.docker.com/mcp/server/filesystem/overview)

<ResponseField name="filesystem" type="object">
  <Expandable title="properties">
    <ResponseField name="paths" type="string[]" required />
  </Expandable>
</ResponseField>

## Find-A-Domain

Tools for finding domain names.

[View on Docker Hub](https://hub.docker.com/mcp/server/find-a-domain/overview)

<ResponseField name="findADomain" type="object" />

## Firecrawl

🔥 Official Firecrawl MCP Server - Adds powerful web scraping and search to Cursor, Claude and any other LLM clients.

[View on Docker Hub](https://hub.docker.com/mcp/server/firecrawl/overview)

<ResponseField name="firecrawl" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="creditCriticalThreshold" type="number" required />

    <ResponseField name="creditWarningThreshold" type="number" required />

    <ResponseField name="retryBackoffFactor" type="number" required />

    <ResponseField name="retryDelay" type="number" required />

    <ResponseField name="retryMax" type="number" required />

    <ResponseField name="retryMaxDelay" type="number" required />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Firewalla

Real-time network monitoring, security analysis, and firewall management through 28 specialized tools. Access security alerts, network flows, device status, and firewall rules directly from your Firewalla device.

[View on Docker Hub](https://hub.docker.com/mcp/server/firewalla-mcp-server/overview)

<ResponseField name="firewalla" type="object">
  <Expandable title="properties">
    <ResponseField name="boxId" type="string" required>
      Your Firewalla Box Global ID
    </ResponseField>

    <ResponseField name="firewallaMspToken" type="string" required />

    <ResponseField name="mspId" type="string" required>
      Your Firewalla MSP domain (e.g., yourdomain.firewalla.net)
    </ResponseField>
  </Expandable>
</ResponseField>

## FlexPrice

Official flexprice MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/flexprice/overview)

<ResponseField name="flexprice" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="baseUrl" type="string" required />
  </Expandable>
</ResponseField>

## Git (Reference)

Git repository interaction and automation.

[View on Docker Hub](https://hub.docker.com/mcp/server/git/overview)

<ResponseField name="git" type="object">
  <Expandable title="properties">
    <ResponseField name="paths" type="string[]" required />
  </Expandable>
</ResponseField>

## GitHub (Archived)

Tools for interacting with the GitHub API, enabling file operations, repository management, search functionality, and more.

[View on Docker Hub](https://hub.docker.com/mcp/server/github/overview)

<ResponseField name="github" type="object">
  <Expandable title="properties">
    <ResponseField name="personalAccessToken" type="string" required />
  </Expandable>
</ResponseField>

## GitHub Chat

A Model Context Protocol (MCP) for analyzing and querying GitHub repositories using the GitHub Chat API.

[View on Docker Hub](https://hub.docker.com/mcp/server/github-chat/overview)

<ResponseField name="githubChat" type="object">
  <Expandable title="properties">
    <ResponseField name="githubApiKey" type="string" required />
  </Expandable>
</ResponseField>

## GitHub Official

Official GitHub MCP Server, by GitHub. Provides seamless integration with GitHub APIs, enabling advanced automation and interaction capabilities for developers and tools.

[View on Docker Hub](https://hub.docker.com/mcp/server/github-official/overview)

<ResponseField name="githubOfficial" type="object">
  <Expandable title="properties">
    <ResponseField name="githubPersonalAccessToken" type="string" required />
  </Expandable>
</ResponseField>

## GitLab (Archived)

MCP Server for the GitLab API, enabling project management, file operations, and more.

[View on Docker Hub](https://hub.docker.com/mcp/server/gitlab/overview)

<ResponseField name="gitlab" type="object">
  <Expandable title="properties">
    <ResponseField name="personalAccessToken" type="string" required />

    <ResponseField name="url" type="string" required>
      api url - optional for self-hosted instances
    </ResponseField>
  </Expandable>
</ResponseField>

## GitMCP

Tools for interacting with Git repositories.

[View on Docker Hub](https://hub.docker.com/mcp/server/gitmcp/overview)

<ResponseField name="gitmcp" type="object" />

## glif.app

Easily run glif.app AI workflows inside your LLM: image generators, memes, selfies, and more. Glif supports all major multimedia AI models inside one app.

[View on Docker Hub](https://hub.docker.com/mcp/server/glif/overview)

<ResponseField name="glif" type="object">
  <Expandable title="properties">
    <ResponseField name="apiToken" type="string" required />

    <ResponseField name="ids" type="string" required />

    <ResponseField name="ignoredSaved" type="boolean" required />
  </Expandable>
</ResponseField>

## Gmail

A Model Context Protocol server for Gmail operations using IMAP/SMTP with app password authentication. Supports listing messages, searching emails, and sending messages. To create your app password, visit your Google Account settings under Security > App Passwords. Or visit the link [https://myaccount.google.com/apppasswords](https://myaccount.google.com/apppasswords).

[View on Docker Hub](https://hub.docker.com/mcp/server/gmail-mcp/overview)

<ResponseField name="gmail" type="object">
  <Expandable title="properties">
    <ResponseField name="emailAddress" type="string" required>
      Your Gmail email address
    </ResponseField>

    <ResponseField name="emailPassword" type="string" />
  </Expandable>
</ResponseField>

## Google Maps (Archived)

Tools for interacting with the Google Maps API.

[View on Docker Hub](https://hub.docker.com/mcp/server/google-maps/overview)

<ResponseField name="googleMaps" type="object">
  <Expandable title="properties">
    <ResponseField name="googleMapsApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Google Maps Comprehensive MCP

Complete Google Maps integration with 8 tools including geocoding, places search, directions, elevation data, and more using Google's latest APIs.

[View on Docker Hub](https://hub.docker.com/mcp/server/google-maps-comprehensive/overview)

<ResponseField name="googleMapsComprehensive" type="object">
  <Expandable title="properties">
    <ResponseField name="googleMapsApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Grafana

MCP server for Grafana.

[View on Docker Hub](https://hub.docker.com/mcp/server/grafana/overview)

<ResponseField name="grafana" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Gyazo

Official Model Context Protocol server for Gyazo.

[View on Docker Hub](https://hub.docker.com/mcp/server/gyazo/overview)

<ResponseField name="gyazo" type="object">
  <Expandable title="properties">
    <ResponseField name="accessToken" type="string" required />
  </Expandable>
</ResponseField>

## Hackernews mcp

A Model Context Protocol (MCP) server that provides access to Hacker News stories, comments, and user data, with support for search and content retrieval.

[View on Docker Hub](https://hub.docker.com/mcp/server/mcp-hackernews/overview)

<ResponseField name="hackernews" type="object" />

## Hackle

Model Context Protocol server for Hackle.

[View on Docker Hub](https://hub.docker.com/mcp/server/hackle/overview)

<ResponseField name="hackle" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Handwriting OCR

Model Context Protocol (MCP) Server for Handwriting OCR.

[View on Docker Hub](https://hub.docker.com/mcp/server/handwriting-ocr/overview)

<ResponseField name="handwritingOcr" type="object">
  <Expandable title="properties">
    <ResponseField name="apiToken" type="string" required />
  </Expandable>
</ResponseField>

## Humanitarian Data Exchange

HDX MCP Server provides access to humanitarian data through the Humanitarian Data Exchange (HDX) API - [https://data.humdata.org/hapi](https://data.humdata.org/hapi). This server offers 33 specialized tools for retrieving humanitarian information including affected populations (refugees, IDPs, returnees), baseline demographics, food security indicators, conflict data, funding information, and operational presence across hundreds of countries and territories. See repository for instructions on getting a free HDX\_APP\_INDENTIFIER for access.

[View on Docker Hub](https://hub.docker.com/mcp/server/hdx/overview)

<ResponseField name="hdx" type="object">
  <Expandable title="properties">
    <ResponseField name="appIdentifier" type="string" required />
  </Expandable>
</ResponseField>

## Heroku

Heroku Platform MCP Server using the Heroku CLI.

[View on Docker Hub](https://hub.docker.com/mcp/server/heroku/overview)

<ResponseField name="heroku" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Hostinger API

Interact with Hostinger services over the Hostinger API.

[View on Docker Hub](https://hub.docker.com/mcp/server/hostinger-mcp-server/overview)

<ResponseField name="hostinger" type="object">
  <Expandable title="properties">
    <ResponseField name="apitoken" type="string" required />
  </Expandable>
</ResponseField>

## Hoverfly

A Model Context Protocol (MCP) server that exposes Hoverfly as a programmable tool for AI assistants like Cursor, Claude, GitHub Copilot, and others supporting MCP. It enables dynamic mocking of third-party APIs to unblock development, automate testing, and simulate unavailable services during integration.

[View on Docker Hub](https://hub.docker.com/mcp/server/hoverfly-mcp-server/overview)

<ResponseField name="hoverfly" type="object">
  <Expandable title="properties">
    <ResponseField name="data" type="string" required />
  </Expandable>
</ResponseField>

## HubSpot

Unite marketing, sales, and customer service with AI-powered automation, lead management, and comprehensive analytics.

[View on Docker Hub](https://hub.docker.com/mcp/server/hubspot/overview)

<ResponseField name="hubspot" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Hugging Face

Tools for interacting with Hugging Face models, datasets, research papers, and more.

[View on Docker Hub](https://hub.docker.com/mcp/server/hugging-face/overview)

<ResponseField name="huggingFace" type="object" />

## Hummingbot MCP: Trading Agent

Hummingbot MCP is an open-source toolset that lets you control and monitor your Hummingbot trading bots through AI-powered commands and automation.

[View on Docker Hub](https://hub.docker.com/mcp/server/hummingbot-mcp/overview)

<ResponseField name="hummingbot" type="object">
  <Expandable title="properties">
    <ResponseField name="apiUrl" type="string" required />

    <ResponseField name="hummingbotApiPassword" type="string" />

    <ResponseField name="hummingbotApiUsername" type="string" />
  </Expandable>
</ResponseField>

## Husqvarna Automower

MCP Server for huqsvarna automower.

[View on Docker Hub](https://hub.docker.com/mcp/server/husqvarna-automower/overview)

<ResponseField name="husqvarnaAutomower" type="object">
  <Expandable title="properties">
    <ResponseField name="clientId" type="string" required />

    <ResponseField name="husqvarnaClientSecret" type="string" required />
  </Expandable>
</ResponseField>

## Hyperbrowser

A MCP server implementation for hyperbrowser.

[View on Docker Hub](https://hub.docker.com/mcp/server/hyperbrowser/overview)

<ResponseField name="hyperbrowser" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Hyperspell

Hyperspell MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/hyperspell/overview)

<ResponseField name="hyperspell" type="object">
  <Expandable title="properties">
    <ResponseField name="collection" type="string" required />

    <ResponseField name="token" type="string" required />

    <ResponseField name="useResources" type="boolean" required />
  </Expandable>
</ResponseField>

## Iaptic

Model Context Protocol server for interacting with iaptic.

[View on Docker Hub](https://hub.docker.com/mcp/server/iaptic/overview)

<ResponseField name="iaptic" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" />

    <ResponseField name="appName" type="string" required />
  </Expandable>
</ResponseField>

## Inspektor Gadget

AI interface to troubleshoot and observe Kubernetes/Container workloads.

[View on Docker Hub](https://hub.docker.com/mcp/server/inspektor-gadget/overview)

<ResponseField name="inspektorGadget" type="object">
  <Expandable title="properties">
    <ResponseField name="gadgetImages" type="string">
      Comma-separated list of gadget images (trace\_dns, trace\_tcp, etc) to use, allowing control over which gadgets are available as MCP tools
    </ResponseField>

    <ResponseField name="kubeconfig" type="string" required>
      Path to the kubeconfig file for accessing Kubernetes clusters
    </ResponseField>
  </Expandable>
</ResponseField>

## Javadocs

Access to Java, Kotlin, and Scala library documentation.

[View on Docker Hub](https://hub.docker.com/mcp/server/javadocs/overview)

<ResponseField name="javadocs" type="object" />

## JetBrains

A model context protocol server to work with JetBrains IDEs: IntelliJ, PyCharm, WebStorm, etc. Also, works with Android Studio.

[View on Docker Hub](https://hub.docker.com/mcp/server/jetbrains/overview)

<ResponseField name="jetbrains" type="object">
  <Expandable title="properties">
    <ResponseField name="port" type="number" required />
  </Expandable>
</ResponseField>

## Kafka Schema Registry MCP

Comprehensive MCP server for Kafka Schema Registry operations. Features multi-registry support, schema contexts, migration tools, OAuth authentication, and 57+ tools for complete schema management. Supports SLIM\_MODE for optimal performance.

[View on Docker Hub](https://hub.docker.com/mcp/server/kafka-schema-reg-mcp/overview)

<ResponseField name="kafkaSchemaReg" type="object">
  <Expandable title="properties">
    <ResponseField name="registryUrl" type="string" required>
      Schema Registry URL
    </ResponseField>

    <ResponseField name="schemaRegistryPassword" type="string" />

    <ResponseField name="schemaRegistryUser" type="string" />

    <ResponseField name="slimMode" type="string">
      Enable SLIM\_MODE for better performance
    </ResponseField>

    <ResponseField name="viewonly" type="string">
      Enable read-only mode
    </ResponseField>
  </Expandable>
</ResponseField>

## Kagi search

The Official Model Context Protocol (MCP) server for Kagi search & other tools.

[View on Docker Hub](https://hub.docker.com/mcp/server/kagisearch/overview)

<ResponseField name="kagisearch" type="object">
  <Expandable title="properties">
    <ResponseField name="engine" type="string" required />

    <ResponseField name="kagiApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Keboola

Keboola MCP Server is an open-source bridge between your Keboola project and modern AI tools.

[View on Docker Hub](https://hub.docker.com/mcp/server/keboola-mcp/overview)

<ResponseField name="keboola" type="object">
  <Expandable title="properties">
    <ResponseField name="kbcStorageToken" type="string" required />

    <ResponseField name="kbcWorkspaceSchema" type="string" required />
  </Expandable>
</ResponseField>

## Kong Konnect

A Model Context Protocol (MCP) server for interacting with Kong Konnect APIs, allowing AI assistants to query and analyze Kong Gateway configurations, traffic, and analytics.

[View on Docker Hub](https://hub.docker.com/mcp/server/kong/overview)

<ResponseField name="kong" type="object">
  <Expandable title="properties">
    <ResponseField name="konnectAccessToken" type="string" required />

    <ResponseField name="region" type="string" required />
  </Expandable>
</ResponseField>

## Kubectl

MCP Server that enables AI assistants to interact with Kubernetes clusters via kubectl operations.

[View on Docker Hub](https://hub.docker.com/mcp/server/kubectl-mcp-server/overview)

<ResponseField name="kubectl" type="object">
  <Expandable title="properties">
    <ResponseField name="kubeconfig" type="string" required />
  </Expandable>
</ResponseField>

## Kubernetes

Connect to a Kubernetes cluster and manage it.

[View on Docker Hub](https://hub.docker.com/mcp/server/kubernetes/overview)

<ResponseField name="kubernetes" type="object">
  <Expandable title="properties">
    <ResponseField name="configPath" type="string" required>
      the path to the host .kube/config
    </ResponseField>
  </Expandable>
</ResponseField>

## Lara Translate

Connect to Lara Translate API, enabling powerful translation capabilities with support for language detection and context-aware translations.

[View on Docker Hub](https://hub.docker.com/mcp/server/lara/overview)

<ResponseField name="lara" type="object">
  <Expandable title="properties">
    <ResponseField name="accessKeySecret" type="string" />

    <ResponseField name="keyId" type="string" required />
  </Expandable>
</ResponseField>

## LINE

MCP server that integrates the LINE Messaging API to connect an AI Agent to the LINE Official Account.

[View on Docker Hub](https://hub.docker.com/mcp/server/line/overview)

<ResponseField name="line" type="object">
  <Expandable title="properties">
    <ResponseField name="channelAccessToken" type="string" />

    <ResponseField name="userId" type="string" required />
  </Expandable>
</ResponseField>

## LinkedIn

This MCP server allows Claude and other AI assistants to access your LinkedIn. Scrape LinkedIn profiles and companies, get your recommended jobs, and perform job searches. Set your li\_at LinkedIn cookie to use this server.

[View on Docker Hub](https://hub.docker.com/mcp/server/linkedin-mcp-server/overview)

<ResponseField name="linkedin" type="object">
  <Expandable title="properties">
    <ResponseField name="linkedinCookie" type="string" required />

    <ResponseField name="userAgent" type="string" required>
      Custom user agent string (optional, helps avoid detection and cookie login issues)
    </ResponseField>
  </Expandable>
</ResponseField>

## LLM Text

Discovers and retrieves llms.txt from websites.

[View on Docker Hub](https://hub.docker.com/mcp/server/llmtxt/overview)

<ResponseField name="llmtxt" type="object" />

## Maestro

A Model Context Protocol (MCP) server exposing Bitcoin blockchain data through the Maestro API platform. Provides tools to explore blocks, transactions, addresses, inscriptions, runes, and other metaprotocol data.

[View on Docker Hub](https://hub.docker.com/mcp/server/maestro-mcp-server/overview)

<ResponseField name="maestro" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKeyApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Manifold

Tools for accessing the Manifold Markets online prediction market platform.

[View on Docker Hub](https://hub.docker.com/mcp/server/manifold/overview)

<ResponseField name="manifold" type="object" />

## Mapbox

Transform any AI agent into a geospatially-aware system with Mapbox APIs. Provides geocoding, POI search, routing, travel time matrices, isochrones, and static map generation.

[View on Docker Hub](https://hub.docker.com/mcp/server/mapbox/overview)

<ResponseField name="mapbox" type="object">
  <Expandable title="properties">
    <ResponseField name="accessToken" type="string" required />
  </Expandable>
</ResponseField>

## Mapbox Developer

Direct access to Mapbox developer APIs for AI assistants. Enables style management, token management, GeoJSON preview, and other developer tools for building Mapbox applications.

[View on Docker Hub](https://hub.docker.com/mcp/server/mapbox-devkit/overview)

<ResponseField name="mapboxDevkit" type="object">
  <Expandable title="properties">
    <ResponseField name="mapboxAccessToken" type="string" required />
  </Expandable>
</ResponseField>

## Markdownify

A Model Context Protocol server for converting almost anything to Markdown.

[View on Docker Hub](https://hub.docker.com/mcp/server/markdownify/overview)

<ResponseField name="markdownify" type="object">
  <Expandable title="properties">
    <ResponseField name="paths" type="string[]" required />
  </Expandable>
</ResponseField>

## Markitdown

A lightweight MCP server for calling MarkItDown.

[View on Docker Hub](https://hub.docker.com/mcp/server/markitdown/overview)

<ResponseField name="markitdown" type="object">
  <Expandable title="properties">
    <ResponseField name="paths" type="string[]" required />
  </Expandable>
</ResponseField>

## Maven Tools

JVM dependency intelligence for any build tool using Maven Central Repository. Includes Context7 integration for upgrade documentation and guidance.

[View on Docker Hub](https://hub.docker.com/mcp/server/maven-tools-mcp/overview)

<ResponseField name="mavenTools" type="object" />

## Memory (Reference)

Knowledge graph-based persistent memory system.

[View on Docker Hub](https://hub.docker.com/mcp/server/memory/overview)

<ResponseField name="memory" type="object" />

## Mercado Libre

Provides access to Mercado Libre E-Commerce API.

[View on Docker Hub](https://hub.docker.com/mcp/server/mercado-libre/overview)

<ResponseField name="mercadoLibre" type="object">
  <Expandable title="properties">
    <ResponseField name="mercadoLibreApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Mercado Pago

Provides access to Mercado Pago Marketplace API.

[View on Docker Hub](https://hub.docker.com/mcp/server/mercado-pago/overview)

<ResponseField name="mercadoPago" type="object">
  <Expandable title="properties">
    <ResponseField name="mercadoPagoApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Metabase MCP

A comprehensive MCP server for Metabase with 70+ tools.

[View on Docker Hub](https://hub.docker.com/mcp/server/metabase/overview)

<ResponseField name="metabase" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="metabaseurl" type="string" required />

    <ResponseField name="metabaseusername" type="string" required />

    <ResponseField name="password" type="string" required />
  </Expandable>
</ResponseField>

## Minecraft Wiki

A MCP Server for browsing the official Minecraft Wiki!.

[View on Docker Hub](https://hub.docker.com/mcp/server/minecraft-wiki/overview)

<ResponseField name="minecraftWiki" type="object" />

## MongoDB

A Model Context Protocol server to connect to MongoDB databases and MongoDB Atlas Clusters.

[View on Docker Hub](https://hub.docker.com/mcp/server/mongodb/overview)

<ResponseField name="mongodb" type="object">
  <Expandable title="properties">
    <ResponseField name="mdbMcpConnectionString" type="string" required />
  </Expandable>
</ResponseField>

## MultiversX

MCP Server for MultiversX.

[View on Docker Hub](https://hub.docker.com/mcp/server/multiversx-mx/overview)

<ResponseField name="multiversxMx" type="object">
  <Expandable title="properties">
    <ResponseField name="network" type="string" required />

    <ResponseField name="wallet" type="string" required />
  </Expandable>
</ResponseField>

## Nasdaq Data Link

MCP server to interact with the data feeds provided by the Nasdaq Data Link. Developed by the community and maintained by Stefano Amorelli.

[View on Docker Hub](https://hub.docker.com/mcp/server/nasdaq-data-link/overview)

<ResponseField name="nasdaqDataLink" type="object">
  <Expandable title="properties">
    <ResponseField name="nasdaqDataLinkApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Needle

Production-ready RAG service to search and retrieve data from your documents.

[View on Docker Hub](https://hub.docker.com/mcp/server/needle-mcp/overview)

<ResponseField name="needle" type="object">
  <Expandable title="properties">
    <ResponseField name="needleApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Neo4j Cloud Aura Api

Manage Neo4j Aura database instances through the Neo4j Aura API.

[View on Docker Hub](https://hub.docker.com/mcp/server/neo4j-cloud-aura-api/overview)

<ResponseField name="neo4jCloudAuraApi" type="object">
  <Expandable title="properties">
    <ResponseField name="clientId" type="string" required />

    <ResponseField name="neo4jAuraClientSecret" type="string" />

    <ResponseField name="serverAllowOrigins" type="string" />

    <ResponseField name="serverAllowedHosts" type="string" />

    <ResponseField name="serverHost" type="string" />

    <ResponseField name="serverPath" type="string" />

    <ResponseField name="serverPort" type="string" />

    <ResponseField name="transport" type="string" />
  </Expandable>
</ResponseField>

## Neo4j Cypher

Interact with Neo4j using Cypher graph queries.

[View on Docker Hub](https://hub.docker.com/mcp/server/neo4j-cypher/overview)

<ResponseField name="neo4jCypher" type="object">
  <Expandable title="properties">
    <ResponseField name="database" type="string" />

    <ResponseField name="namespace" type="string" />

    <ResponseField name="neo4jPassword" type="string" />

    <ResponseField name="readOnly" type="boolean" />

    <ResponseField name="readTimeout" type="string" />

    <ResponseField name="responseTokenLimit" type="string" />

    <ResponseField name="serverAllowOrigins" type="string" />

    <ResponseField name="serverAllowedHosts" type="string" />

    <ResponseField name="serverHost" type="string" />

    <ResponseField name="serverPath" type="string" />

    <ResponseField name="serverPort" type="string" />

    <ResponseField name="transport" type="string" />

    <ResponseField name="url" type="string" required />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Neo4j Data Modeling

MCP server that assists in creating, validating and visualizing graph data models.

[View on Docker Hub](https://hub.docker.com/mcp/server/neo4j-data-modeling/overview)

<ResponseField name="neo4jDataModeling" type="object">
  <Expandable title="properties">
    <ResponseField name="serverAllowOrigins" type="string" required />

    <ResponseField name="serverAllowedHosts" type="string" required />

    <ResponseField name="serverHost" type="string" required />

    <ResponseField name="serverPath" type="string" required />

    <ResponseField name="serverPort" type="string" required />

    <ResponseField name="transport" type="string" required />
  </Expandable>
</ResponseField>

## Neo4j Memory

Provide persistent memory capabilities through Neo4j graph database integration.

[View on Docker Hub](https://hub.docker.com/mcp/server/neo4j-memory/overview)

<ResponseField name="neo4jMemory" type="object">
  <Expandable title="properties">
    <ResponseField name="database" type="string" />

    <ResponseField name="neo4jPassword" type="string" />

    <ResponseField name="serverAllowOrigins" type="string" />

    <ResponseField name="serverAllowedHosts" type="string" />

    <ResponseField name="serverHost" type="string" />

    <ResponseField name="serverPath" type="string" />

    <ResponseField name="serverPort" type="string" />

    <ResponseField name="transport" type="string" />

    <ResponseField name="url" type="string" required />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Neon

MCP server for interacting with Neon Management API and databases.

[View on Docker Hub](https://hub.docker.com/mcp/server/neon/overview)

<ResponseField name="neon" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Node.js Sandbox

A Node.js–based Model Context Protocol server that spins up disposable Docker containers to execute arbitrary JavaScript.

[View on Docker Hub](https://hub.docker.com/mcp/server/node-code-sandbox/overview)

<ResponseField name="nodeCodeSandbox" type="object" />

## Notion

Official Notion MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/notion/overview)

<ResponseField name="notion" type="object">
  <Expandable title="properties">
    <ResponseField name="internalIntegrationToken" type="string" required />
  </Expandable>
</ResponseField>

## Novita

Seamless interaction with Novita AI platform resources.

[View on Docker Hub](https://hub.docker.com/mcp/server/novita/overview)

<ResponseField name="novita" type="object" />

## NPM Sentinel

MCP server that enables intelligent NPM package analysis powered by AI.

[View on Docker Hub](https://hub.docker.com/mcp/server/npm-sentinel/overview)

<ResponseField name="npmSentinel" type="object" />

## Obsidian

MCP server that interacts with Obsidian via the Obsidian rest API community plugin.

[View on Docker Hub](https://hub.docker.com/mcp/server/obsidian/overview)

<ResponseField name="obsidian" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Okta

Secure Okta identity and access management via Model Context Protocol (MCP). Access Okta users, groups, applications, logs, and policies through AI assistants with enterprise-grade security.

[View on Docker Hub](https://hub.docker.com/mcp/server/okta-mcp-fctr/overview)

<ResponseField name="oktaMcpFctr" type="object">
  <Expandable title="properties">
    <ResponseField name="clientOrgurl" type="string" required>
      Okta organization URL (e.g., [https://dev-123456.okta.com](https://dev-123456.okta.com))
    </ResponseField>

    <ResponseField name="concurrentLimit" type="string">
      Maximum concurrent requests to Okta API
    </ResponseField>

    <ResponseField name="logLevel" type="string">
      Logging level for server output
    </ResponseField>

    <ResponseField name="oktaApiToken" type="string" />
  </Expandable>
</ResponseField>

## omi-mcp

A Model Context Protocol server for Omi interaction and automation. This server provides tools to read, search, and manipulate Memories and Conversations.

[View on Docker Hub](https://hub.docker.com/mcp/server/omi/overview)

<ResponseField name="omi" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## ONLYOFFICE DocSpace

ONLYOFFICE DocSpace is a room-based collaborative platform which allows organizing a clear file structure depending on users' needs or project goals.

[View on Docker Hub](https://hub.docker.com/mcp/server/onlyoffice-docspace/overview)

<ResponseField name="onlyofficeDocspace" type="object">
  <Expandable title="properties">
    <ResponseField name="baseUrl" type="string" required />

    <ResponseField name="docspaceApiKey" type="string" required />

    <ResponseField name="docspaceAuthToken" type="string" required />

    <ResponseField name="docspacePassword" type="string" required />

    <ResponseField name="docspaceUsername" type="string" required />

    <ResponseField name="dynamic" type="boolean" required />

    <ResponseField name="origin" type="string" required />

    <ResponseField name="toolsets" type="string" required />

    <ResponseField name="userAgent" type="string" required />
  </Expandable>
</ResponseField>

## OpenAPI Toolkit for MCP

Fetch, validate, and generate code or curl from any OpenAPI or Swagger spec - all from a single URL.

[View on Docker Hub](https://hub.docker.com/mcp/server/openapi/overview)

<ResponseField name="openapi" type="object">
  <Expandable title="properties">
    <ResponseField name="mode" type="string" required />
  </Expandable>
</ResponseField>

## OpenAPI Schema

OpenAPI Schema Model Context Protocol Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/openapi-schema/overview)

<ResponseField name="openapiSchema" type="object">
  <Expandable title="properties">
    <ResponseField name="SchemaPath" type="string" required />
  </Expandable>
</ResponseField>

## Airbnb Search

MCP Server for searching Airbnb and get listing details.

[View on Docker Hub](https://hub.docker.com/mcp/server/openbnb-airbnb/overview)

<ResponseField name="openbnbAirbnb" type="object" />

## OpenMesh

Discover and connect to a curated marketplace of MCP servers for extending AI agent capabilities.

[View on Docker Hub](https://hub.docker.com/mcp/server/openmesh/overview)

<ResponseField name="openmesh" type="object" />

## Openweather

A simple MCP service that provides current weather and 5-day forecast using the free OpenWeatherMap API.

[View on Docker Hub](https://hub.docker.com/mcp/server/openweather/overview)

<ResponseField name="openweather" type="object">
  <Expandable title="properties">
    <ResponseField name="owmApiKey" type="string" required />
  </Expandable>
</ResponseField>

## OpenZeppelin Cairo Contracts

Access to OpenZeppelin Cairo Contracts.

[View on Docker Hub](https://hub.docker.com/mcp/server/openzeppelin-cairo/overview)

<ResponseField name="openzeppelinCairo" type="object" />

## OpenZeppelin Solidity Contracts

Access to OpenZeppelin Solidity Contracts.

[View on Docker Hub](https://hub.docker.com/mcp/server/openzeppelin-solidity/overview)

<ResponseField name="openzeppelinSolidity" type="object" />

## OpenZeppelin Stellar Contracts

Access to OpenZeppelin Stellar Contracts.

[View on Docker Hub](https://hub.docker.com/mcp/server/openzeppelin-stellar/overview)

<ResponseField name="openzeppelinStellar" type="object" />

## OpenZeppelin Stylus Contracts

Access to OpenZeppelin Stylus Contracts.

[View on Docker Hub](https://hub.docker.com/mcp/server/openzeppelin-stylus/overview)

<ResponseField name="openzeppelinStylus" type="object" />

## Opik

Model Context Protocol (MCP) implementation for Opik enabling seamless IDE integration and unified access to prompts, projects, traces, and metrics.

[View on Docker Hub](https://hub.docker.com/mcp/server/opik/overview)

<ResponseField name="opik" type="object">
  <Expandable title="properties">
    <ResponseField name="apiBaseUrl" type="string" required />

    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="workspaceName" type="string" required />
  </Expandable>
</ResponseField>

## Opine

A Model Context Protocol (MCP) server for querying deals and evaluations from the Opine CRM API.

[View on Docker Hub](https://hub.docker.com/mcp/server/opine-mcp-server/overview)

<ResponseField name="opine" type="object">
  <Expandable title="properties">
    <ResponseField name="opineApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Oracle Database

Connect to Oracle databases via MCP, providing secure read-only access with support for schema exploration, query execution, and metadata inspection.

[View on Docker Hub](https://hub.docker.com/mcp/server/oracle/overview)

<ResponseField name="oracle" type="object">
  <Expandable title="properties">
    <ResponseField name="oracleConnectionString" type="string" required />

    <ResponseField name="oracleUser" type="string" required />

    <ResponseField name="password" type="string" required />
  </Expandable>
</ResponseField>

## OSP Marketing Tools

A Model Context Protocol (MCP) server that empowers LLMs to use some of Open Srategy Partners' core writing and product marketing techniques.

[View on Docker Hub](https://hub.docker.com/mcp/server/osp_marketing_tools/overview)

<ResponseField name="ospMarketingTools" type="object" />

## Oxylabs

A Model Context Protocol (MCP) server that enables AI assistants like Claude to seamlessly access web data through Oxylabs' powerful web scraping technology.

[View on Docker Hub](https://hub.docker.com/mcp/server/oxylabs/overview)

<ResponseField name="oxylabs" type="object">
  <Expandable title="properties">
    <ResponseField name="password" type="string" />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Paper Search

A MCP for searching and downloading academic papers from multiple sources like arXiv, PubMed, bioRxiv, etc.

[View on Docker Hub](https://hub.docker.com/mcp/server/paper-search/overview)

<ResponseField name="paperSearch" type="object" />

## Perplexity

Connector for Perplexity API, to enable real-time, web-wide research.

[View on Docker Hub](https://hub.docker.com/mcp/server/perplexity-ask/overview)

<ResponseField name="perplexityAsk" type="object">
  <Expandable title="properties">
    <ResponseField name="perplexityApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Program Integrity Alliance

An MCP server to help make U.S. Government open datasets AI-friendly.

[View on Docker Hub](https://hub.docker.com/mcp/server/pia/overview)

<ResponseField name="pia" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Pinecone Assistant

Pinecone Assistant MCP server.

[View on Docker Hub](https://hub.docker.com/mcp/server/pinecone/overview)

<ResponseField name="pinecone" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="assistantHost" type="string" required />
  </Expandable>
</ResponseField>

## ExecuteAutomation Playwright MCP

Playwright Model Context Protocol Server - Tool to automate Browsers and APIs in Claude Desktop, Cline, Cursor IDE and More 🔌.

[View on Docker Hub](https://hub.docker.com/mcp/server/playwright-mcp-server/overview)

<ResponseField name="playwright" type="object">
  <Expandable title="properties">
    <ResponseField name="data" type="string" required />
  </Expandable>
</ResponseField>

## Plugged.in MCP Proxy

A unified MCP proxy that aggregates multiple MCP servers into one interface, enabling seamless tool discovery and management across all your AI interactions. Manage all your MCP servers from a single connection point with RAG capabilities and real-time notifications.

[View on Docker Hub](https://hub.docker.com/mcp/server/pluggedin-mcp-proxy/overview)

<ResponseField name="pluggedinMcpProxy" type="object">
  <Expandable title="properties">
    <ResponseField name="pluggedinApiBaseUrl" type="string" required>
      Base URL for the Plugged.in API (optional, defaults to [https://plugged.in](https://plugged.in) for cloud or [http://localhost:12005](http://localhost:12005) for self-hosted)
    </ResponseField>

    <ResponseField name="pluggedinApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Polar Signals

MCP server for Polar Signals Cloud continuous profiling platform, enabling AI assistants to analyze CPU performance, memory usage, and identify optimization opportunities in production systems.

[View on Docker Hub](https://hub.docker.com/mcp/server/polar-signals/overview)

<ResponseField name="polarSignals" type="object">
  <Expandable title="properties">
    <ResponseField name="polarSignalsApiKey" type="string" required />
  </Expandable>
</ResponseField>

## PomoDash

Connect your AI assistant to PomoDash for seamless task and project management.

[View on Docker Hub](https://hub.docker.com/mcp/server/pomodash/overview)

<ResponseField name="pomodash" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## PostgreSQL readonly (Archived)

Connect with read-only access to PostgreSQL databases. This server enables LLMs to inspect database schemas and execute read-only queries.

[View on Docker Hub](https://hub.docker.com/mcp/server/postgres/overview)

<ResponseField name="postgres" type="object">
  <Expandable title="properties">
    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Postman

Postman's MCP server connects AI agents, assistants, and chatbots directly to your APIs on Postman. Use natural language to prompt AI to automate work across your Postman collections, environments, workspaces, and more.

[View on Docker Hub](https://hub.docker.com/mcp/server/postman/overview)

<ResponseField name="postman" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Pref Editor

Pref Editor is a tool for viewing and editing Android app preferences during development.

[View on Docker Hub](https://hub.docker.com/mcp/server/pref-editor/overview)

<ResponseField name="prefEditor" type="object" />

## Prometheus

A Model Context Protocol (MCP) server that enables AI assistants to query and analyze Prometheus metrics through standardized interfaces. Connect to your Prometheus instance to retrieve metrics, perform queries, and gain insights into your system's performance and health.

[View on Docker Hub](https://hub.docker.com/mcp/server/prometheus/overview)

<ResponseField name="prometheus" type="object">
  <Expandable title="properties">
    <ResponseField name="prometheusUrl" type="string" required>
      The URL of your Prometheus server
    </ResponseField>
  </Expandable>
</ResponseField>

## Puppeteer (Archived)

Browser automation and web scraping using Puppeteer.

[View on Docker Hub](https://hub.docker.com/mcp/server/puppeteer/overview)

<ResponseField name="puppeteer" type="object" />

## Python Refactoring Assistant

Educational Python refactoring assistant that provides guided suggestions for AI assistants.  Features: • Step-by-step refactoring instructions without modifying code • Comprehensive code analysis using professional tools (Rope, Radon, Vulture, Jedi, LibCST, Pyrefly) • Educational approach teaching refactoring patterns through guided practice • Support for both guide-only and apply-changes modes • Identifies long functions, high complexity, dead code, and type issues • Provides precise line numbers and specific refactoring instructions • Compatible with all AI assistants (Claude, GPT, Cursor, Continue, etc.)  Perfect for developers learning refactoring patterns while maintaining full control over code changes. Acts as a refactoring mentor rather than an automated code modifier.

[View on Docker Hub](https://hub.docker.com/mcp/server/mcp-python-refactoring/overview)

<ResponseField name="pythonRefactoring" type="object" />

## QuantConnect

The QuantConnect MCP Server is a bridge for AIs (such as Claude and OpenAI o3 Pro) to interact with our cloud platform. When equipped with our MCP, the AI can perform tasks on your behalf through our API such as updating projects, writing strategies, backtesting, and deploying strategies to production live-trading.

[View on Docker Hub](https://hub.docker.com/mcp/server/quantconnect/overview)

<ResponseField name="quantconnect" type="object">
  <Expandable title="properties">
    <ResponseField name="agentname" type="string" required />

    <ResponseField name="quantconnectapitoken" type="string" required />

    <ResponseField name="quantconnectuserid" type="string" required />
  </Expandable>
</ResponseField>

## Ramparts MCP Security Scanner

A comprehensive security scanner for MCP servers with YARA rules and static analysis capabilities.

[View on Docker Hub](https://hub.docker.com/mcp/server/ramparts/overview)

<ResponseField name="ramparts" type="object" />

## Razorpay

Razorpay's Official MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/razorpay/overview)

<ResponseField name="razorpay" type="object">
  <Expandable title="properties">
    <ResponseField name="keyId" type="string" required />

    <ResponseField name="keySecret" type="string" />
  </Expandable>
</ResponseField>

## Mcp reddit

A comprehensive Model Context Protocol (MCP) server for Reddit integration. This server enables AI agents to interact with Reddit programmatically through a standardized interface.

[View on Docker Hub](https://hub.docker.com/mcp/server/mcp-reddit/overview)

<ResponseField name="reddit" type="object">
  <Expandable title="properties">
    <ResponseField name="redditClientId" type="string" required />

    <ResponseField name="redditClientSecret" type="string" required />

    <ResponseField name="redditPassword" type="string" required />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Redis

Access to Redis database operations.

[View on Docker Hub](https://hub.docker.com/mcp/server/redis/overview)

<ResponseField name="redis" type="object">
  <Expandable title="properties">
    <ResponseField name="caCerts" type="string" required />

    <ResponseField name="caPath" type="string" required />

    <ResponseField name="certReqs" type="string" required />

    <ResponseField name="clusterMode" type="boolean" required />

    <ResponseField name="host" type="string" required />

    <ResponseField name="port" type="number" required />

    <ResponseField name="pwd" type="string" required />

    <ResponseField name="ssl" type="boolean" required />

    <ResponseField name="sslCertfile" type="string" required />

    <ResponseField name="sslKeyfile" type="string" required />

    <ResponseField name="username" type="string" required />
  </Expandable>
</ResponseField>

## Redis Cloud

MCP Server for Redis Cloud's API, allowing you to manage your Redis Cloud resources using natural language.

[View on Docker Hub](https://hub.docker.com/mcp/server/redis-cloud/overview)

<ResponseField name="redisCloud" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="secretKey" type="string" />
  </Expandable>
</ResponseField>

## Ref - up-to-date docs

Ref powerful search tool connets your coding tools with documentation context. It includes an up-to-date index of public documentation and it can ingest your private documentation (eg. GitHub repos, PDFs) as well.

[View on Docker Hub](https://hub.docker.com/mcp/server/ref/overview)

<ResponseField name="ref" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Remote MCP

Tools for finding remote MCP servers.

[View on Docker Hub](https://hub.docker.com/mcp/server/remote-mcp/overview)

<ResponseField name="remote" type="object" />

## Render

Interact with your Render resources via LLMs.

[View on Docker Hub](https://hub.docker.com/mcp/server/render/overview)

<ResponseField name="render" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Send emails

Send emails directly from Cursor with this email sending MCP server.

[View on Docker Hub](https://hub.docker.com/mcp/server/resend/overview)

<ResponseField name="resend" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" />

    <ResponseField name="replyTo" type="string" required>
      comma separated list of reply to email addresses
    </ResponseField>

    <ResponseField name="sender" type="string" required>
      sender email address
    </ResponseField>
  </Expandable>
</ResponseField>

## RISKEN

RISKEN's official MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/risken/overview)

<ResponseField name="risken" type="object">
  <Expandable title="properties">
    <ResponseField name="accessToken" type="string" required />

    <ResponseField name="url" type="string" required />
  </Expandable>
</ResponseField>

## Root.io Vulnerability Remediation MCP

MCP server that provides container image vulnerability scanning and remediation capabilities through Root.io.

[View on Docker Hub](https://hub.docker.com/mcp/server/root/overview)

<ResponseField name="root" type="object">
  <Expandable title="properties">
    <ResponseField name="apiAccessToken" type="string" required />
  </Expandable>
</ResponseField>

## WiseVision ROS2

Python server implementing Model Context Protocol (MCP) for ROS2.

[View on Docker Hub](https://hub.docker.com/mcp/server/ros2/overview)

<ResponseField name="ros2" type="object" />

## Rube

Access to Rube's catalog of remote MCP servers.

[View on Docker Hub](https://hub.docker.com/mcp/server/rube/overview)

<ResponseField name="rube" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Blazing-fast, asynchronous for seamless filesystem operations.

The Rust MCP Filesystem is a high-performance, asynchronous, and lightweight Model Context Protocol (MCP) server built in Rust for secure and efficient filesystem operations. Designed with security in mind, it operates in read-only mode by default and restricts clients from updating allowed directories via MCP Roots unless explicitly enabled, ensuring robust protection against unauthorized access. Leveraging asynchronous I/O, it delivers blazingly fast performance with a minimal resource footprint. Optimized for token efficiency, the Rust MCP Filesystem enables large language models (LLMs) to precisely target searches and edits within specific sections of large files and restrict operations by file size range, making it ideal for efficient file exploration, automation, and system integration.

[View on Docker Hub](https://hub.docker.com/mcp/server/rust-mcp-filesystem/overview)

<ResponseField name="rustMcpFilesystem" type="object">
  <Expandable title="properties">
    <ResponseField name="allowWrite" type="boolean" required>
      Enable read/write mode. If false, the app operates in read-only mode.
    </ResponseField>

    <ResponseField name="allowedDirectories" type="string[]" required>
      List of directories that rust-mcp-filesystem can access.
    </ResponseField>

    <ResponseField name="enableRoots" type="boolean" required>
      Enable dynamic directory access control via MCP client-side Roots.
    </ResponseField>
  </Expandable>
</ResponseField>

## SchemaCrawler AI

The SchemaCrawler AI MCP Server enables natural language interaction with your database schema using an MCP client in "Agent" mode. It allows users to explore tables, columns, foreign keys, triggers, stored procedures and more simply by asking questions like "Explain the code for the interest calculation stored procedure". You can also ask it to help with SQL, since it knows your schema. This is ideal for developers, DBAs, and data analysts who want to streamline schema comprehension and query development without diving into dense documentation.

[View on Docker Hub](https://hub.docker.com/mcp/server/schemacrawler-ai/overview)

<ResponseField name="schemacrawlerAi" type="object">
  <Expandable title="properties">
    <ResponseField name="generalInfoLevel" type="string" required>
      \--info-level How much database metadata to retrieve
    </ResponseField>

    <ResponseField name="generalLogLevel" type="string" />

    <ResponseField name="schcrwlrDatabasePassword" type="string" />

    <ResponseField name="schcrwlrDatabaseUser" type="string" />

    <ResponseField name="serverConnectionDatabase" type="string">
      \--database Database to connect to (optional)
    </ResponseField>

    <ResponseField name="serverConnectionHost" type="string">
      \--host Database host (optional)
    </ResponseField>

    <ResponseField name="serverConnectionPort" type="number">
      \--port Database port (optional)
    </ResponseField>

    <ResponseField name="serverConnectionServer" type="string" required>
      \--server SchemaCrawler database plugin
    </ResponseField>

    <ResponseField name="urlConnectionJdbcUrl" type="string" required>
      \--url JDBC URL for database connection
    </ResponseField>

    <ResponseField name="volumeHostShare" type="string" required>
      Host volume to map within the Docker container
    </ResponseField>
  </Expandable>
</ResponseField>

## Schogini MCP Image Border

This adds a border to an image and returns base64 encoded image.

[View on Docker Hub](https://hub.docker.com/mcp/server/schogini-mcp-image-border/overview)

<ResponseField name="schoginiMcpImageBorder" type="object" />

## ScrapeGraph

ScapeGraph MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/scrapegraph/overview)

<ResponseField name="scrapegraph" type="object">
  <Expandable title="properties">
    <ResponseField name="sgaiApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Scrapezy

A Model Context Protocol server for Scrapezy that enables AI models to extract structured data from websites.

[View on Docker Hub](https://hub.docker.com/mcp/server/scrapezy/overview)

<ResponseField name="scrapezy" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Securenote.link

SecureNote.link MCP Server - allowing AI agents to securely share sensitive information through end-to-end encrypted notes.

[View on Docker Hub](https://hub.docker.com/mcp/server/securenote-link-mcp-server/overview)

<ResponseField name="securenoteLink" type="object" />

## Semgrep

MCP server for using Semgrep to scan code for security vulnerabilities.

[View on Docker Hub](https://hub.docker.com/mcp/server/semgrep/overview)

<ResponseField name="semgrep" type="object" />

## Sentry (Archived)

A Model Context Protocol server for retrieving and analyzing issues from Sentry.io. This server provides tools to inspect error reports, stacktraces, and other debugging information from your Sentry account.

[View on Docker Hub](https://hub.docker.com/mcp/server/sentry/overview)

<ResponseField name="sentry" type="object">
  <Expandable title="properties">
    <ResponseField name="authToken" type="string" required />
  </Expandable>
</ResponseField>

## Sequa.AI

Stop stitching context for Copilot and Cursor. With Sequa MCP, your AI tools know your entire codebase and docs out of the box.

[View on Docker Hub](https://hub.docker.com/mcp/server/sequa/overview)

<ResponseField name="sequa" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />

    <ResponseField name="mcpServerUrl" type="string" required />
  </Expandable>
</ResponseField>

## Sequential Thinking (Reference)

Dynamic and reflective problem-solving through thought sequences.

[View on Docker Hub](https://hub.docker.com/mcp/server/sequentialthinking/overview)

<ResponseField name="sequentialthinking" type="object" />

## Short.io

Access to Short.io's link shortener and analytics tools.

[View on Docker Hub](https://hub.docker.com/mcp/server/short-io/overview)

<ResponseField name="shortIo" type="object">
  <Expandable title="properties">
    <ResponseField name="shortIoApiKey" type="string" required />
  </Expandable>
</ResponseField>

## SimpleCheckList

Advanced SimpleCheckList with MCP server and SQLite database for comprehensive task management.  Features: • Complete project and task management system • Hierarchical organization (Projects → Groups → Task Lists → Tasks → Subtasks) • SQLite database for data persistence • RESTful API with comprehensive endpoints • MCP protocol compliance for AI assistant integration • Docker-optimized deployment with stability improvements  **v1.0.1 Update**: Enhanced Docker stability with improved container lifecycle management. Default mode optimized for containerized deployment with reliable startup and shutdown processes.  Perfect for AI assistants managing complex project workflows and task hierarchies.

[View on Docker Hub](https://hub.docker.com/mcp/server/simplechecklist/overview)

<ResponseField name="simplechecklist" type="object" />

## Singlestore

MCP server for interacting with SingleStore Management API and services.

[View on Docker Hub](https://hub.docker.com/mcp/server/singlestore/overview)

<ResponseField name="singlestore" type="object">
  <Expandable title="properties">
    <ResponseField name="mcpApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Slack (Archived)

Interact with Slack Workspaces over the Slack API.

[View on Docker Hub](https://hub.docker.com/mcp/server/slack/overview)

<ResponseField name="slack" type="object">
  <Expandable title="properties">
    <ResponseField name="botToken" type="string" />

    <ResponseField name="channelIds" type="string" />

    <ResponseField name="teamId" type="string" required />
  </Expandable>
</ResponseField>

## SmartBear

MCP server for AI access to SmartBear tools, including BugSnag, Reflect, API Hub, PactFlow.

[View on Docker Hub](https://hub.docker.com/mcp/server/smartbear/overview)

<ResponseField name="smartbear" type="object">
  <Expandable title="properties">
    <ResponseField name="apiHubApiKey" type="string" required />

    <ResponseField name="bugsnagApiKey" type="string" required />

    <ResponseField name="bugsnagAuthToken" type="string" required />

    <ResponseField name="bugsnagEndpoint" type="string" required />

    <ResponseField name="pactBrokerBaseUrl" type="string" required />

    <ResponseField name="pactBrokerPassword" type="string" required />

    <ResponseField name="pactBrokerToken" type="string" required />

    <ResponseField name="pactBrokerUsername" type="string" required />

    <ResponseField name="reflectApiToken" type="string" required />
  </Expandable>
</ResponseField>

## SonarQube

Interact with SonarQube Cloud, Server and Community build over the web API. Analyze code to identify quality and security issues.

[View on Docker Hub](https://hub.docker.com/mcp/server/sonarqube/overview)

<ResponseField name="sonarqube" type="object">
  <Expandable title="properties">
    <ResponseField name="org" type="string" required>
      Organization key for SonarQube Cloud, not required for SonarQube Server or Community Build
    </ResponseField>

    <ResponseField name="token" type="string" required />

    <ResponseField name="url" type="string" required>
      URL of the SonarQube instance, to provide only for SonarQube Server or Community Build
    </ResponseField>
  </Expandable>
</ResponseField>

## SQLite (Archived)

Database interaction and business intelligence capabilities.

[View on Docker Hub](https://hub.docker.com/mcp/server/SQLite/overview)

<ResponseField name="sqlite" type="object" />

## StackGen

AI-powered DevOps assistant for managing cloud infrastructure and applications.

[View on Docker Hub](https://hub.docker.com/mcp/server/stackgen/overview)

<ResponseField name="stackgen" type="object">
  <Expandable title="properties">
    <ResponseField name="token" type="string" />

    <ResponseField name="url" type="string" required>
      URL of your StackGen instance
    </ResponseField>
  </Expandable>
</ResponseField>

## StackHawk

A Model Context Protocol (MCP) server for integrating with StackHawk's security scanning platform. Provides security analytics, YAML configuration management, sensitive data/threat surface analysis, and anti-hallucination tools for LLMs.

[View on Docker Hub](https://hub.docker.com/mcp/server/stackhawk/overview)

<ResponseField name="stackhawk" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Stripe

Interact with Stripe services over the Stripe API.

[View on Docker Hub](https://hub.docker.com/mcp/server/stripe/overview)

<ResponseField name="stripe" type="object">
  <Expandable title="properties">
    <ResponseField name="secretKey" type="string" required />
  </Expandable>
</ResponseField>

## Supadata

Official Supadata MCP Server - Adds powerful video & web scraping to Cursor, Claude and any other LLM clients.

[View on Docker Hub](https://hub.docker.com/mcp/server/supadata/overview)

<ResponseField name="supadata" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Suzieq MCP

MCP Server to interact with a SuzieQ network observability instance via its REST API.

[View on Docker Hub](https://hub.docker.com/mcp/server/suzieq/overview)

<ResponseField name="suzieq" type="object">
  <Expandable title="properties">
    <ResponseField name="apiEndpoint" type="string" required />

    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Task orchestrator

Model Context Protocol (MCP) server for comprehensive task and feature management, providing AI assistants with a structured, context-efficient way to interact with project data.

[View on Docker Hub](https://hub.docker.com/mcp/server/task-orchestrator/overview)

<ResponseField name="taskOrchestrator" type="object" />

## Tavily

The Tavily MCP server provides seamless interaction with the tavily-search and tavily-extract tools, real-time web search capabilities through the tavily-search tool and Intelligent data extraction from web pages via the tavily-extract tool.

[View on Docker Hub](https://hub.docker.com/mcp/server/tavily/overview)

<ResponseField name="tavily" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Teamwork

Tools for Teamwork.com products.

[View on Docker Hub](https://hub.docker.com/mcp/server/teamwork/overview)

<ResponseField name="teamwork" type="object">
  <Expandable title="properties">
    <ResponseField name="twMcpBearerToken" type="string" required />
  </Expandable>
</ResponseField>

## Telnyx

Enables interaction with powerful telephony, messaging, and AI assistant APIs.

[View on Docker Hub](https://hub.docker.com/mcp/server/telnyx/overview)

<ResponseField name="telnyx" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Tembo

MCP server for Tembo Cloud's platform API.

[View on Docker Hub](https://hub.docker.com/mcp/server/tembo/overview)

<ResponseField name="tembo" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Hashicorp Terraform

The Terraform MCP Server provides seamless integration with Terraform ecosystem, enabling advanced automation and interaction capabilities for Infrastructure as Code (IaC) development.

[View on Docker Hub](https://hub.docker.com/mcp/server/terraform/overview)

<ResponseField name="terraform" type="object" />

## Text-to-GraphQL

Transform natural language queries into GraphQL queries using an AI agent. Provides schema management, query validation, execution, and history tracking.

[View on Docker Hub](https://hub.docker.com/mcp/server/text-to-graphql/overview)

<ResponseField name="textToGraphql" type="object">
  <Expandable title="properties">
    <ResponseField name="graphqlApiKey" type="string" required />

    <ResponseField name="graphqlAuthType" type="string" required>
      Authentication method for GraphQL API
    </ResponseField>

    <ResponseField name="graphqlEndpoint" type="string" required />

    <ResponseField name="modelName" type="string" required>
      OpenAI model to use
    </ResponseField>

    <ResponseField name="modelTemperature" type="number" required>
      Model temperature for responses
    </ResponseField>

    <ResponseField name="openaiApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Tigris Data

Tigris is a globally distributed S3-compatible object storage service that provides low latency anywhere in the world, enabling developers to store and access any amount of data for a wide range of use cases.

[View on Docker Hub](https://hub.docker.com/mcp/server/tigris/overview)

<ResponseField name="tigris" type="object">
  <Expandable title="properties">
    <ResponseField name="awsAccessKeyId" type="string" required />

    <ResponseField name="awsEndpointUrlS3" type="string" required />

    <ResponseField name="awsSecretAccessKey" type="string" />
  </Expandable>
</ResponseField>

## Time (Reference)

Time and timezone conversion capabilities.

[View on Docker Hub](https://hub.docker.com/mcp/server/time/overview)

<ResponseField name="time" type="object" />

## Triplewhale

Triplewhale MCP Server.

[View on Docker Hub](https://hub.docker.com/mcp/server/triplewhale/overview)

<ResponseField name="triplewhale" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Unreal Engine

A comprehensive Model Context Protocol (MCP) server that enables AI assistants to control Unreal Engine via Remote Control API. Built with TypeScript and designed for game development automation.

[View on Docker Hub](https://hub.docker.com/mcp/server/unreal-engine-mcp-server/overview)

<ResponseField name="unrealEngine" type="object">
  <Expandable title="properties">
    <ResponseField name="logLevel" type="string">
      Logging level
    </ResponseField>

    <ResponseField name="ueHost" type="string" required>
      Unreal Engine host address. Use: host.docker.internal for local UE on Windows/Mac Docker, 127.0.0.1 for Linux without Docker, or actual IP address (e.g., 192.168.1.100) for remote UE
    </ResponseField>

    <ResponseField name="ueRcHttpPort" type="string" required>
      Remote Control HTTP port
    </ResponseField>

    <ResponseField name="ueRcWsPort" type="string" required>
      Remote Control WebSocket port
    </ResponseField>
  </Expandable>
</ResponseField>

## VeyraX

VeyraX MCP is the only connection you need to access all your tools in any MCP-compatible environment.

[View on Docker Hub](https://hub.docker.com/mcp/server/veyrax/overview)

<ResponseField name="veyrax" type="object">
  <Expandable title="properties">
    <ResponseField name="apiKey" type="string" required />
  </Expandable>
</ResponseField>

## Vizro

provides tools and templates to create a functioning Vizro chart or dashboard step by step.

[View on Docker Hub](https://hub.docker.com/mcp/server/vizro/overview)

<ResponseField name="vizro" type="object" />

## Vuln nist

This MCP server exposes tools to query the NVD/CVE REST API and return formatted text results suitable for LLM consumption via the MCP protocol. It includes automatic query chunking for large date ranges and parallel processing for improved performance.

[View on Docker Hub](https://hub.docker.com/mcp/server/vuln-nist-mcp-server/overview)

<ResponseField name="vulnNist" type="object" />

## Wayfound MCP

Wayfound’s MCP server allows business users to govern, supervise, and improve AI Agents.

[View on Docker Hub](https://hub.docker.com/mcp/server/wayfound/overview)

<ResponseField name="wayfound" type="object">
  <Expandable title="properties">
    <ResponseField name="mcpApiKey" type="string" required />
  </Expandable>
</ResponseField>

## Webflow

Model Context Protocol (MCP) server for the Webflow Data API.

[View on Docker Hub](https://hub.docker.com/mcp/server/webflow/overview)

<ResponseField name="webflow" type="object">
  <Expandable title="properties">
    <ResponseField name="token" type="string" required />
  </Expandable>
</ResponseField>

## Wikipedia

A Model Context Protocol (MCP) server that retrieves information from Wikipedia to provide context to LLMs.

[View on Docker Hub](https://hub.docker.com/mcp/server/wikipedia-mcp/overview)

<ResponseField name="wikipedia" type="object" />

## WolframAlpha

Connect your chat repl to wolfram alpha computational intelligence.

[View on Docker Hub](https://hub.docker.com/mcp/server/wolfram-alpha/overview)

<ResponseField name="wolframAlpha" type="object">
  <Expandable title="properties">
    <ResponseField name="wolframApiKey" type="string" required />
  </Expandable>
</ResponseField>

## YouTube transcripts

Retrieves transcripts for given YouTube video URLs.

[View on Docker Hub](https://hub.docker.com/mcp/server/youtube_transcript/overview)

<ResponseField name="youtubeTranscript" type="object" />

## Zerodha Kite Connect

MCP server for Zerodha Kite Connect API - India's leading stock broker trading platform. Execute trades, manage portfolios, and access real-time market data for NSE, BSE, and other Indian exchanges.

[View on Docker Hub](https://hub.docker.com/mcp/server/zerodha-kite/overview)

<ResponseField name="zerodhaKite" type="object">
  <Expandable title="properties">
    <ResponseField name="kiteAccessToken" type="string">
      Access token obtained after OAuth authentication (optional - can be generated at runtime)
    </ResponseField>

    <ResponseField name="kiteApiKey" type="string" required>
      Your Kite Connect API key from the developer console
    </ResponseField>

    <ResponseField name="kiteApiSecret" type="string" />

    <ResponseField name="kiteRedirectUrl" type="string">
      OAuth redirect URL configured in your Kite Connect app
    </ResponseField>
  </Expandable>
</ResponseField>
