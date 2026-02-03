from __future__ import annotations

import argparse
import base64
import json
import os
import subprocess
import tempfile
import time
import urllib.request
from pathlib import Path
from zipfile import ZipFile
import smtplib
from email.message import EmailMessage


def _load_env(path: Path) -> None:
    if not path.exists():
        return
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        os.environ.setdefault(key, value)


def build_sample_pdf(path: Path, text: str) -> None:
    header = b"%PDF-1.4\n"

    def obj(num: int, body: str) -> bytes:
        return f"{num} 0 obj\n{body}\nendobj\n".encode("utf-8")

    stream_text = f"BT\n/F1 24 Tf\n72 120 Td\n({text}) Tj\nET\n"
    stream_obj = f"<< /Length {len(stream_text)} >>\nstream\n{stream_text}endstream"

    objects = [
        obj(1, "<< /Type /Catalog /Pages 2 0 R >>"),
        obj(2, "<< /Type /Pages /Kids [3 0 R] /Count 1 >>"),
        obj(
            3,
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Contents 4 0 R "
            "/Resources << /Font << /F1 5 0 R >> >> >>",
        ),
        obj(4, stream_obj),
        obj(5, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>"),
    ]

    offsets = [0]
    pos = len(header)
    for entry in objects:
        offsets.append(pos)
        pos += len(entry)

    xref_pos = pos
    xref_lines = [f"xref\n0 {len(objects) + 1}\n", "0000000000 65535 f \n"]
    for offset in offsets[1:]:
        xref_lines.append(f"{offset:010d} 00000 n \n")
    xref = "".join(xref_lines).encode("utf-8")

    trailer = (
        f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R >>\n"
        f"startxref\n{xref_pos}\n%%EOF\n"
    ).encode("utf-8")

    pdf_bytes = header + b"".join(objects) + xref + trailer
    path.write_bytes(pdf_bytes)


def build_sample_docx(path: Path, text: str) -> None:
    content_types = """<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>
"""

    rels = """<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>
"""

    document = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>{text}</w:t></w:r></w:p>
  </w:body>
</w:document>
"""

    with ZipFile(path, "w") as archive:
        archive.writestr("[Content_Types].xml", content_types)
        archive.writestr("_rels/.rels", rels)
        archive.writestr("word/document.xml", document)


def postmark_request(url: str, token: str, payload: dict | None = None, method: str = "GET") -> dict:
    data = None
    headers = {
        "Accept": "application/json",
        "X-Postmark-Server-Token": token,
    }
    if payload is not None:
        data = json.dumps(payload).encode("utf-8")
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, data=data, method=method, headers=headers)
    with urllib.request.urlopen(req, timeout=30) as resp:
        body = resp.read().decode("utf-8")
    return json.loads(body)


def wait_for_ngrok_url(timeout_s: int = 30) -> str:
    for _ in range(timeout_s):
        try:
            with urllib.request.urlopen("http://127.0.0.1:4040/api/tunnels", timeout=5) as resp:
                data = json.loads(resp.read().decode("utf-8"))
            for tunnel in data.get("tunnels", []):
                if tunnel.get("proto") == "https":
                    return tunnel.get("public_url")
        except Exception:
            pass
        time.sleep(1)
    raise RuntimeError("ngrok tunnel not available")


def poll_workspace(root: Path, timeout_s: int = 120) -> Path:
    for _ in range(timeout_s):
        if root.exists():
            dirs = [p for p in root.iterdir() if p.is_dir()]
            if dirs:
                latest = max(dirs, key=lambda p: p.stat().st_mtime)
                reply = latest / "email_reply.md"
                if reply.exists():
                    return latest
        time.sleep(1)
    raise RuntimeError("Timed out waiting for email_reply.md")


def poll_outbound(token: str, recipient: str, subject_hint: str, timeout_s: int = 90) -> dict | None:
    for _ in range(timeout_s):
        outbound = postmark_request(
            f"https://api.postmarkapp.com/messages/outbound?recipient={recipient}&count=50&offset=0",
            token,
        )
        messages = outbound.get("Messages", [])
        for msg in messages:
            subject = msg.get("Subject") or ""
            if subject_hint in subject:
                return msg
        time.sleep(1)
    return None


def main() -> None:
    parser = argparse.ArgumentParser(description="Run real-email end-to-end test via Postmark inbound webhook")
    parser.add_argument("--from", dest="from_addr", default="agent@dowhiz.com")
    parser.add_argument("--to", dest="to_addr", default=None, help="Recipient address (e.g. agent@dowhiz.com)")
    parser.add_argument("--subject", default="Real email MVP test")
    parser.add_argument("--webhook-port", type=int, default=9000)
    parser.add_argument(
        "--send-method",
        choices=["smtp-inbound", "postmark-api"],
        default="smtp-inbound",
        help="smtp-inbound sends directly to Postmark inbound hash address; postmark-api sends via Postmark outbound API.",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    _load_env(repo_root / ".env")

    token = os.getenv("POSTMARK_SERVER_TOKEN")
    if not token:
        raise SystemExit("POSTMARK_SERVER_TOKEN not set in .env")

    server_info = postmark_request("https://api.postmarkapp.com/server", token)
    inbound_address = server_info.get("InboundAddress")
    if not inbound_address:
        raise SystemExit("Postmark server does not have an inbound address configured")

    previous_hook = server_info.get("InboundHookUrl", "")

    workspace_root = Path(tempfile.mkdtemp(prefix="dowhiz_real_workspaces_"))

    env = os.environ.copy()
    env["WORKSPACE_ROOT"] = str(workspace_root)
    env["OUTBOUND_MODE"] = "postmark"

    webhook_proc = subprocess.Popen(
        ["python", "-m", "mvp.email_pipeline.postmark_webhook_server", "--port", str(args.webhook_port)],
        cwd=str(repo_root),
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )

    ngrok_proc = subprocess.Popen(
        ["ngrok", "http", str(args.webhook_port), "--log=stdout"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )

    try:
        time.sleep(2)
        public_url = wait_for_ngrok_url()
        hook_url = f"{public_url}/postmark/inbound"

        postmark_request(
            "https://api.postmarkapp.com/server",
            token,
            payload={"InboundHookUrl": hook_url},
            method="PUT",
        )

        with tempfile.TemporaryDirectory() as tmpdir:
            tmp_path = Path(tmpdir)
            pdf_path = tmp_path / "sample.pdf"
            docx_path = tmp_path / "sample.docx"
            build_sample_pdf(pdf_path, "DoWhiz PDF attachment")
            build_sample_docx(docx_path, "DoWhiz DOCX attachment")

            if args.send_method == "postmark-api":
                to_addr = args.to_addr
                if not to_addr:
                    raise RuntimeError("--to is required for postmark-api sending.")
                attachments = [
                    {
                        "Name": pdf_path.name,
                        "Content": base64.b64encode(pdf_path.read_bytes()).decode("ascii"),
                        "ContentType": "application/pdf",
                    },
                    {
                        "Name": docx_path.name,
                        "Content": base64.b64encode(docx_path.read_bytes()).decode("ascii"),
                        "ContentType": "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                    },
                ]
                payload = {
                    "From": args.from_addr,
                    "To": to_addr,
                    "Subject": args.subject,
                    "TextBody": "Please process these attachments and reply in the same thread.",
                    "Attachments": attachments,
                }
                postmark_request("https://api.postmarkapp.com/email", token, payload=payload, method="POST")
            else:
                msg = EmailMessage()
                msg["From"] = args.from_addr
                msg["To"] = inbound_address
                msg["Subject"] = args.subject
                msg.set_content("Please process these attachments and reply in the same thread.")

                msg.add_attachment(
                    pdf_path.read_bytes(),
                    maintype="application",
                    subtype="pdf",
                    filename=pdf_path.name,
                )
                msg.add_attachment(
                    docx_path.read_bytes(),
                    maintype="application",
                    subtype="vnd.openxmlformats-officedocument.wordprocessingml.document",
                    filename=docx_path.name,
                )

                with smtplib.SMTP("inbound.postmarkapp.com", 25, timeout=20) as smtp:
                    smtp.send_message(msg)

        workspace = poll_workspace(workspace_root)
        reply_path = workspace / "email_reply.md"
        reply_text = reply_path.read_text(encoding="utf-8")

        matched = poll_outbound(token, args.from_addr, args.subject, timeout_s=90)

        print("Real email test summary:")
        print("  Inbound address:", inbound_address)
        print("  Webhook URL:", hook_url)
        print("  Workspace:", workspace)
        print("  Reply length:", len(reply_text))
        if matched:
            print("  Outbound message status:", matched.get("Status"))
            print("  Outbound subject:", matched.get("Subject"))
        else:
            latest = postmark_request(
                f"https://api.postmarkapp.com/messages/outbound?recipient={args.from_addr}&count=1&offset=0",
                token,
            ).get("Messages", [])
            latest_msg = latest[0] if latest else None
            print("  Outbound message status: not found")
            if latest_msg:
                print("  Latest outbound subject:", latest_msg.get("Subject"))
            raise RuntimeError("Outbound reply not found in Postmark search.")

    finally:
        try:
            postmark_request(
                "https://api.postmarkapp.com/server",
                token,
                payload={"InboundHookUrl": previous_hook},
                method="PUT",
            )
        except Exception:
            pass

        if webhook_proc.poll() is None:
            webhook_proc.terminate()
        if ngrok_proc.poll() is None:
            ngrok_proc.terminate()

        if webhook_proc.stdout:
            webhook_proc.stdout.close()
        if ngrok_proc.stdout:
            ngrok_proc.stdout.close()


if __name__ == "__main__":
    main()
