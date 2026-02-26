# Official Postmark Libraries

Postmark provides officially maintained libraries for seamless integration with your applications. These are the recommended solutions for implementing Postmark's email services.

## Rails Integration

### Postmark Rails Gem

The dedicated Rails gem enables drop-in integration with ActionMailer. Configuration requires:

1. Add to Gemfile: `gem 'postmark-rails'`
2. Store your Server API token in `config/secrets.yml`
3. Set delivery method in `config/application.rb`

Basic usage allows sending HTML emails with tracking enabled through ActionMailer's standard methods.

**Repository**: [https://github.com/activecampaign/postmark-rails](https://github.com/activecampaign/postmark-rails)

## Ruby Implementation

### Postmark Ruby Gem

For standalone Ruby applications, this gem provides direct API access. Installation via Bundler creates an `ApiClient` instance for sending messages programmatically.

**Repository**: [https://github.com/activecampaign/postmark-gem](https://github.com/activecampaign/postmark-gem)

## .NET Development

### Postmark.NET NuGet Package

Available through NuGet package manager, this library supports .NET projects. A strongly-named variant exists to prevent dependency conflicts.

**Installation**: `Install-Package Postmark`

Features include message creation with attachments, headers, and open tracking.

**Package Details**: [https://www.nuget.org/packages/Postmark/](https://www.nuget.org/packages/Postmark/)

## Java Support

### Postmark Java Library

Maven-compatible library for Java 8+ projects. Simplifies API communication for sending individual messages and managing templates.

**Wiki Documentation**: [https://github.com/activecampaign/postmark-java/wiki](https://github.com/activecampaign/postmark-java/wiki)

## PHP Integration

### Postmark PHP Library

Composer-based package available on Packagist. Provides straightforward client methods for email transmission.

**Installation**: `composer require wildbit/postmark-php`

**Packagist Page**: [https://packagist.org/packages/wildbit/postmark-php](https://packagist.org/packages/wildbit/postmark-php)

## Craft CMS Plugin

### Postmark Craft Plugin

Available through Craft Plugin Store and Composer. Configuration happens within the admin interface settings panel, supporting custom message streams.

**Plugin Store**: [https://plugins.craftcms.com/postmark](https://plugins.craftcms.com/postmark)

## Node.js Solutions

### Postmark Node.js Library

NPM-distributed package for JavaScript environments. Optional Nodemailer integration available for alternative usage patterns.

**Installation**: `npm install postmark --save`

**NPM Package**: [https://www.npmjs.com/package/postmark](https://www.npmjs.com/package/postmark)

## Command Line Interface

### Postmark CLI

Node.js-based command-line tool requiring installation via npm globally. Enables template management, server administration, and email sending without programming.

**Installation**: `npm i postmark-cli -g`

**GitHub Wiki**: [https://github.com/activecampaign/postmark-cli/wiki](https://github.com/activecampaign/postmark-cli/wiki)

## WordPress Support

### Postmark for WordPress Plugin

Replaces default WordPress mail function with Postmark delivery. Installation through admin dashboard with verification testing built-in.

**WordPress Plugin Directory**: [https://wordpress.org/plugins/postmark-approved-wordpress-plugin/](https://wordpress.org/plugins/postmark-approved-wordpress-plugin/)

## Build Tool Integration

### Postmark Grunt Plugin

NPM package for Grunt task runners. Enables email sending within build processes.

**Installation**: `npm install grunt-postmark --save`

**NPM Documentation**: [https://www.npmjs.com/package/grunt-postmark](https://www.npmjs.com/package/grunt-postmark)

## Automation Platform

### Postmark Zapier Actions

Native integration supporting both standard email and templated message sending. Configurable message stream selection within workflow builders.

**Zapier Integration**: [https://zapier.com/apps/postmark/integrations](https://zapier.com/apps/postmark/integrations)

---

## Additional Resources

- [Community-maintained libraries](../community-libraries/README.md)
- [Code examples for inbound email parsing](../../user-guide/processing-email/README.md)
- Contribution opportunities are documented separately in the developer portal
