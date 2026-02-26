# Manage Language Settings

## Overview

Developers can configure language settings for Gmail accounts using the Settings API, which determines the language displayed in Gmail's user interface.

## Key Resources

- **Get language settings**: Use the `getLanguage` method
- **Update language settings**: Use the `updateLanguage` method
- **Reference documentation**: Settings reference guide

## Display Language Format

Language preferences follow the RFC 3066 Language Tag format standard. Not all Gmail clients support identical language sets. When a user's preferred language isn't available on a particular client, the system automatically defaults to the closest supported variant (or a reasonable default).

For the most current list of supported languages, refer to the language dropdown menu in Gmail settings or consult the language settings help article.

## Supported Languages Reference

The documentation provides a comprehensive table mapping languages across three columns:

- Language name (in English)
- Display name (as shown in Gmail settings)
- RFC 3066 language tag

Examples include:
- English (US): `en`
- Spanish (Latin America): `es-419`
- Chinese (Simplified): `zh-CN`
- Portuguese (Brazil): `pt-BR`

The table encompasses over 100 language options spanning multiple scripts and regional variants, from Afrikaans (`af`) through Korean (`ko`).
