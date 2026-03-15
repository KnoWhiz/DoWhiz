#!/usr/bin/env python3
"""
Quick E2E test for Service Account + DWD.
Tests: create doc, add content, share with user.

Usage:
    pip install google-auth google-api-python-client
    python test_google_e2e.py /path/to/service-account.json oliver@dowhiz.com ellen@dowhiz.com
"""

import sys
import json

def main():
    if len(sys.argv) < 4:
        print("Usage: python test_google_e2e.py <service-account.json> <impersonate-email> <share-with-email>")
        print("Example: python test_google_e2e.py ./sa.json oliver@dowhiz.com ellen@dowhiz.com")
        sys.exit(1)

    sa_file = sys.argv[1]
    impersonate_email = sys.argv[2]
    share_with_email = sys.argv[3]

    try:
        from google.oauth2 import service_account
        from googleapiclient.discovery import build
    except ImportError:
        print("Installing dependencies...")
        import subprocess
        subprocess.check_call([sys.executable, "-m", "pip", "install",
                              "google-auth", "google-api-python-client", "-q"])
        from google.oauth2 import service_account
        from googleapiclient.discovery import build

    SCOPES = [
        'https://www.googleapis.com/auth/documents',
        'https://www.googleapis.com/auth/drive',
    ]

    print(f"1. Loading service account from: {sa_file}")
    credentials = service_account.Credentials.from_service_account_file(
        sa_file,
        scopes=SCOPES,
        subject=impersonate_email
    )
    print(f"   [OK] Will impersonate: {impersonate_email}")

    print("\n2. Creating Google Doc...")
    docs_service = build('docs', 'v1', credentials=credentials)
    doc = docs_service.documents().create(
        body={'title': 'E2E Test - Service Account + DWD'}
    ).execute()
    doc_id = doc['documentId']
    print(f"   [OK] Created doc: https://docs.google.com/document/d/{doc_id}")

    print("\n3. Adding content to doc...")
    requests = [
        {
            'insertText': {
                'location': {'index': 1},
                'text': 'Hello! This document was created by Oliver using Service Account + DWD.\n\nThe token never expires!'
            }
        }
    ]
    docs_service.documents().batchUpdate(
        documentId=doc_id,
        body={'requests': requests}
    ).execute()
    print("   [OK] Content added")

    print(f"\n4. Sharing doc with {share_with_email}...")
    drive_service = build('drive', 'v3', credentials=credentials)
    permission = {
        'type': 'user',
        'role': 'writer',
        'emailAddress': share_with_email
    }
    drive_service.permissions().create(
        fileId=doc_id,
        body=permission,
        sendNotificationEmail=True
    ).execute()
    print(f"   [OK] Shared with {share_with_email}")

    print(f"\n5. Verifying document owner...")
    file_info = drive_service.files().get(fileId=doc_id, fields="owners").execute()
    owners = file_info.get('owners', [])
    owner_email = owners[0].get('emailAddress', 'unknown') if owners else 'unknown'
    print(f"   Document owner: {owner_email}")

    print("\n" + "="*60)
    print("E2E TEST PASSED!")
    print("="*60)
    print(f"\nDoc URL: https://docs.google.com/document/d/{doc_id}")
    print(f"Owner: {owner_email}")
    print(f"Shared with: {share_with_email}")
    print("\nYou can delete this test doc manually.")

if __name__ == '__main__':
    main()
