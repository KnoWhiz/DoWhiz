> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# BYOC (Bring Your Own Cloud)

BYOC (Bring Your Own Cloud) allows you to deploy E2B sandboxes to your own cloud infrastructure within your VPC.

BYOC is currently only available for AWS. We are working on adding support for Google Cloud and Azure.

<Note>
  BYOC is offered to enterprise customers only.
  If you’re interested in BYOC offering, please book a call with our team [here](https://e2b.dev/contact) or contact us at [enterprise@e2b.dev](mailto:enterprise@e2b.dev).
</Note>

## Architecture

Sandbox templates, snapshots, and runtime logs are stored within the customer's BYOC VPC.
Anonymized system metrics such as cluster memory and cpu are sent to the E2B Cloud for observability and cluster management purposes.

All potentially sensitive traffic, such as sandbox template build source files,
sandbox traffic, and logs, is transmitted directly from the client to the customer's BYOC VPC without ever touching the E2B Cloud infrastructure.

### Glossary

* **BYOC VPC**: The customer's Virtual Private Network where the E2B sandboxes are deployed. For example your AWS account.
* **E2B Cloud**: The managed service that provides the E2B platform, observability and cluster management.
* **OAuth Provider**: Customer-managed service that provides user and E2B Cloud with access to the cluster.

<Frame>
  <img src="https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=435653373dd380b97b00d7e7da0db893" data-og-width="2156" width="2156" data-og-height="1528" height="1528" data-path="images/byoc-architecture-diagram.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?w=280&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=8b70998a0049cdbb8712c3751a77aef8 280w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?w=560&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=111468ec9be3693756442b9533028d39 560w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?w=840&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=b3938fe998be63e027c03785d0d15ced 840w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?w=1100&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=cb1f2a11d33a243d197e16a002d110b9 1100w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?w=1650&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=c5cf0d0410c2861e3c467df71db44222 1650w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/byoc-architecture-diagram.png?w=2500&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=131e234a6ba2d3149c20914f6b6082af 2500w" />
</Frame>

### BYOC cluster components

* **Orchestrator**: Represents a node that is responsible for managing sandboxes and their lifecycle. Optionally, it can also run the template builder component.
* **Edge Controller**: Routes traffic to sandboxes, exposes API for cluster management, and gRPC proxy used by E2B control plane to communicate with orchestrators.
* **Monitoring**: Collector that receives sandbox and build logs and system metrics from orchestrators and edge controllers. Only anonymized metrics are sent to the E2B Cloud for observability purposes.
* **Storage**: Persistent storage for sandbox templates, snapshots, and runtime logs. Image container repository for template images.

## Onboarding

Customers can initiate the onboarding process by reaching out to us.
Customers need to have a dedicated AWS account and know the region they will use.
After that, we will receive the IAM role needed for managing account resources.
For AWS account quota limits may need to be increased.

Terraform configuration and machine images will be used for provisioning BYOC cluster.
When provisioning is done and running, we will create a new team under your E2B account that can be used by SDK/CLI the same way as it is hosted on E2B Cloud.

## FAQ

<AccordionGroup>
  <Accordion title="How is cluster monitored?">
    Cluster is forwarding anonymized metrics such as machine cpu/memory usage to E2B Control plane for advanced observability and alerting.
    The whole observability stack is anonymized and does not contain any sensitive information.
  </Accordion>

  <Accordion title="Can cluster automatically scale?">
    A cluster can be scaled horizontally by adding more orchestrators and edge controllers.
    The autoscaler is currently in V1 not capable of automatically scale orchestrator nodes that are needed for sandbox spawning.
    This feature is coming in the next versions.
  </Accordion>

  <Accordion title="Are sandboxes accessible only from a customer’s private network?">
    Yes. Load balancer that is handling all requests coming to sandbox can be configured as internal and VPC peering
    with additional customer’s VPC can be configured so sandbox traffic can stay in the private network.
  </Accordion>

  <Accordion title="How control plane secure communication is ensured?">
    Data sent between the E2B Cloud and your BYOC VPC is encrypted using TLS.

    VPC peering can be established to allow direct communication between the E2B Cloud and your BYOC VPC.
    When using VPC peering, the load balancer can be configured as private without a public IP address.
  </Accordion>
</AccordionGroup>
