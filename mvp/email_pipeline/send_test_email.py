from __future__ import annotations

import argparse
import smtplib
import tempfile
from email.message import EmailMessage
from email.utils import make_msgid
from pathlib import Path
from zipfile import ZipFile


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


def main() -> None:
    parser = argparse.ArgumentParser(description="Send a test email to the local SMTP pipeline")
    parser.add_argument("--smtp-host", default="127.0.0.1")
    parser.add_argument("--smtp-port", type=int, default=8025)
    parser.add_argument("--from", dest="from_addr", default="agent@dowhiz.com")
    parser.add_argument("--to", dest="to_addr", default="deep-tutor@deep-tutor.com")
    parser.add_argument("--subject", default="MVP pipeline test")
    parser.add_argument("--no-attachments", action="store_true")
    args = parser.parse_args()

    msg = EmailMessage()
    msg["From"] = args.from_addr
    msg["To"] = args.to_addr
    msg["Subject"] = args.subject
    msg["Message-ID"] = make_msgid(domain=args.from_addr.split("@")[-1])
    msg.set_content("Hi! Please process these attachments and reply in the same thread.")

    if not args.no_attachments:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp_path = Path(tmpdir)
            pdf_path = tmp_path / "sample.pdf"
            docx_path = tmp_path / "sample.docx"
            build_sample_pdf(pdf_path, "DoWhiz PDF attachment")
            build_sample_docx(docx_path, "DoWhiz DOCX attachment")

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

            with smtplib.SMTP(args.smtp_host, args.smtp_port) as smtp:
                smtp.send_message(msg)
    else:
        with smtplib.SMTP(args.smtp_host, args.smtp_port) as smtp:
            smtp.send_message(msg)

    print("Sent test email to", args.to_addr)


if __name__ == "__main__":
    main()
