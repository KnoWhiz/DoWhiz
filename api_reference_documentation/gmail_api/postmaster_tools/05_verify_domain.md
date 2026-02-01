# Verify Authentication Domain

## Overview

This documentation explains how to verify authentication domains registered with the Gmail Postmaster Tools API. You can verify either a specific domain or all domains at once.

## Verify a Specific Domain

To verify a single domain, use the `domains.get()` method with the domain name.

### Java Example

```java
/**
 * Gets a specific domain registered by the client.
 *
 * @param service Authorized Gmail PostmasterTools API instance.
 * @param domainName The fully qualified domain name.
 * @return The domain
 * @throws IOException
 */
public static Domain getDomain(PostmasterTools service, String domainName) throws IOException {
  String query = String.format("domains/%s", domainName);
  Domain domain = service.domains().get(query).execute();
  System.out.println(domain.toPrettyString());
  return domain;
}
```

### Python Example

```python
def get_domain(service, domain_name):
  """Gets a specific domain registered by the client.

  Args:
    service: Authorized Gmail PostmasterTools API instance.
    domain_name: The fully qualified domain name.

  Returns:
    The domain.
  """
  try:
      query = 'domains/' + domain_name
      domain = service.domains().get(name=query).execute()
      print(domain)
      return domain
  except errors.HttpError as err:
      print('An error occurred: %s' % err)
```

## Verify All Domains

To retrieve all registered domains, use the `domains.list()` method.

### Java Example

```java
/**
 * Lists the domains that have been registered by the client.
 *
 * @param service Authorized Gmail PostmasterTools API instance.
 * @return Response message for ListDomains.
 * @throws IOException
 */
public static ListDomainsResponse listDomains(PostmasterTools service) throws IOException {
  ListDomainsResponse listDomainsResponse = service.domains().list().execute();
  for (Domain domain : listDomainsResponse.getDomains()) {
    System.out.println(domain.toPrettyString());
  }
    return listDomainsResponse;
}
```

### Python Example

```python
def list_domains(service):
  """Lists the domains that have been registered by the client.

  Args:
    service: Authorized Gmail PostmasterTools API instance.

  Returns:
    Response message for ListDomains.
  """
  try:
      domains = service.domains().list().execute()
      if not domains:
          print('No domains found.')
      else:
          print('Domains:')
          for domain in domains['domains']:
              print(domain)
      return domains
  except errors.HttpError as err:
      print('An error occurred: %s' % err)
```
