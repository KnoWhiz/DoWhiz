# Synchronizing Clients with Gmail

## Overview

This guide explains how to keep your client synchronized with Gmail using two primary strategies: full synchronization and partial synchronization. Full syncs are needed initially, while partial syncs offer a lighter-weight alternative for subsequent updates.

## Full Synchronization

### Purpose

Required on first connection to Gmail or when partial synchronization isn't available.

### Procedure

The process involves three main steps:

1. **Retrieve Message IDs**: Call `messages.list` to get the first page of message identifiers.

2. **Batch Retrieve Messages**: Create a batch request of `messages.get` calls for returned messages. The guide recommends:
   - Use `format=FULL` or `format=RAW` for initial retrieval
   - Cache results to avoid redundant operations
   - Use `format=MINIMAL` for previously cached messages (reduces response size)

3. **Merge and Store**: Incorporate updates into cached data and preserve the `historyId` from the most recent message for future partial synchronization.

### Alternative Approach

The `Threads` resource methods can be used equivalently, which may be preferable for thread-focused applications.

## Partial Synchronization

### Method

Use the `history.list` method with a `startHistoryId` parameter to retrieve changes since the last sync.

### Benefits

Significantly reduces bandwidth and processing compared to full synchronization.

### Workflow

Obtain and store the `historyId` from full or partial syncs to supply as `startHistoryId` for future operations.

## Limitations

History records typically remain available for at least one week but availability varies. If the supplied `startHistoryId` falls outside the available range, the API returns an HTTP 404 error, requiring a full synchronization.
