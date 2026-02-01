# Test Your Schemas - Gmail Documentation

## Overview

This guide outlines tools and methods for adding structured data to emails and validating its accuracy.

## Self Testing

Developers can validate markup functionality by sending test emails to themselves. The system automatically bypasses registration requirements when both sender and recipient are identical accounts. However, DKIM or SPF authentication is still required for self-testing.

After completing end-to-end testing through this approach, developers should consult the registration documentation to prepare for production deployment.

## Available Testing Tools

### Structured Data Markup Helper

The Structured Data Markup Helper tool supports markup validation for these reservation types:
- Event Reservation
- Flight Reservation
- Lodging Reservation
- Restaurant Reservation

### Email Markup Tester

This tool enables developers to paste markup and validate it through a dedicated interface. The validation process:

- Extracts structured data with complete property details for each entity
- Provides specific error messages for invalid input
- Displays results in an organized format for review

## Next Steps

Once markup validation is complete, developers should reference the registration documentation to move their implementation into production environments.
