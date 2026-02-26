# What Are Highlights? - Gmail Markup Documentation

## Overview

Gmail Highlights is a feature that surfaces key information and actions from emails as easy-to-see chips displayed above messages. As a sender, you can elevate important details using structured data markup.

## How Highlights Work

An important feature of Gmail is **Highlights**. As the name implies, they surface key information and actions from an email and display them as easy-to-see chips in an email message.

Examples include:
- Flight confirmations showing trip summaries with check-in options
- Order receipts displaying product images and delivery dates

## Supported Highlight Types

The markup system supports the following content categories:

1. **Bus Reservation**
2. **Car Rental Reservation**
3. **Train Reservation**
4. **Flight Reservation** - includes real-time status and Check In actions
5. **Order** - includes View Order actions
6. **Parcel Delivery** - includes Track Package actions and status badges
7. **Hotel Reservation**
8. **Restaurant Reservation**
9. **Invoice**
10. **Ticketed Event Reservation** - includes View Tickets actions

## Key Features by Type

**Transportation Reservations**: Display departure/arrival information with destination images; flight highlights support Check In actions and real-time status updates.

**Orders & Delivery**: Show cost, estimated delivery, and product images; parcel delivery includes real-time shipment status.

**Reservations**: Hotel, restaurant, and event highlights display booking details with specified images.

## Implementation

Declare highlights using structured data markup. Refer to the markup reference documentation for specific schema requirements for each highlight type.
