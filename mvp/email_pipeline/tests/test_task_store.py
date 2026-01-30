from __future__ import annotations

import unittest
from unittest import mock
from datetime import datetime
from types import SimpleNamespace

from mvp.email_pipeline import task_store
from mvp.email_pipeline.task_store import EmailTask, TaskStatus, TaskStore


class _DuplicateKeyError(Exception):
    pass


class FakeCollection:
    def __init__(self) -> None:
        self.docs = {}

    def create_index(self, *_args, **_kwargs) -> None:
        return None

    def insert_one(self, doc):
        _id = doc.get("_id")
        if _id in self.docs:
            raise _DuplicateKeyError("duplicate")
        self.docs[_id] = dict(doc)
        return SimpleNamespace(inserted_id=_id)

    def find_one(self, query):
        if "_id" in query:
            return self.docs.get(query["_id"])
        for doc in self.docs.values():
            if _match(doc, query):
                return doc
        return None

    def update_one(self, query, update):
        doc = None
        if "_id" in query:
            candidate = self.docs.get(query["_id"])
            if candidate and _match(candidate, query):
                doc = candidate
        else:
            for candidate in self.docs.values():
                if _match(candidate, query):
                    doc = candidate
                    break
        if not doc:
            return SimpleNamespace(modified_count=0)

        if "$set" in update:
            doc.update(update["$set"])
        if "$inc" in update:
            for key, value in update["$inc"].items():
                doc[key] = doc.get(key, 0) + value
        if "$push" in update:
            for key, value in update["$push"].items():
                doc.setdefault(key, []).append(value)
        return SimpleNamespace(modified_count=1)

    def find(self, query):
        docs = [doc for doc in self.docs.values() if _match(doc, query)]
        return FakeCursor(docs)

    def count_documents(self, query):
        return len([doc for doc in self.docs.values() if _match(doc, query)])


class FakeCursor:
    def __init__(self, docs):
        self.docs = list(docs)

    def sort(self, key, direction):
        reverse = direction < 0
        self.docs.sort(key=lambda doc: doc.get(key), reverse=reverse)
        return self

    def limit(self, count):
        self.docs = self.docs[:count]
        return self

    def __iter__(self):
        return iter(self.docs)


class FakeDB:
    def __init__(self) -> None:
        self.collection = FakeCollection()

    def __getitem__(self, name):
        return self.collection


class FakeMongoClient:
    def __init__(self, _uri):
        self.db = FakeDB()

    def __getitem__(self, _name):
        return self.db


def _match(doc, query):
    for key, value in query.items():
        if isinstance(value, dict) and "$nin" in value:
            if doc.get(key) in value["$nin"]:
                return False
            continue
        if doc.get(key) != value:
            return False
    return True


class TaskStoreTests(unittest.TestCase):
    def setUp(self) -> None:
        self.mongo_patch = mock.patch.object(task_store, "MongoClient", FakeMongoClient)
        self.error_patch = mock.patch.object(task_store, "DuplicateKeyError", _DuplicateKeyError)
        self.mongo_patch.start()
        self.error_patch.start()

    def tearDown(self) -> None:
        self.mongo_patch.stop()
        self.error_patch.stop()

    def test_create_and_duplicate(self) -> None:
        store = TaskStore("mongodb://fake")
        task = EmailTask(
            message_id="<id@example.com>",
            postmark_message_id=None,
            content_hash="hash",
            from_address="sender@example.com",
            to_addresses=["receiver@example.com"],
            subject="Hi",
            status=TaskStatus.PENDING,
            attempts=0,
            max_retries=2,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
        )
        self.assertTrue(store.create_task(task))
        self.assertFalse(store.create_task(task))

    def test_mark_processing_and_failed(self) -> None:
        store = TaskStore("mongodb://fake")
        task = EmailTask(
            message_id="<id2@example.com>",
            postmark_message_id=None,
            content_hash="hash2",
            from_address="sender@example.com",
            to_addresses=["receiver@example.com"],
            subject="Hi",
            status=TaskStatus.PENDING,
            attempts=0,
            max_retries=1,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
        )
        store.create_task(task)
        self.assertTrue(store.mark_processing(task.message_id))
        store.mark_failed(task.message_id, "err1")
        stored = store.get_task(task.message_id)
        self.assertEqual(stored.status, TaskStatus.PENDING)
        self.assertTrue(store.mark_processing(task.message_id))
        store.mark_failed(task.message_id, "err2")
        stored = store.get_task(task.message_id)
        self.assertEqual(stored.status, TaskStatus.FAILED)

    def test_mark_completed_and_stats(self) -> None:
        store = TaskStore("mongodb://fake")
        task = EmailTask(
            message_id="<id3@example.com>",
            postmark_message_id=None,
            content_hash="hash3",
            from_address="sender@example.com",
            to_addresses=["receiver@example.com"],
            subject="Hi",
            status=TaskStatus.PENDING,
            attempts=0,
            max_retries=2,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
        )
        store.create_task(task)
        store.mark_processing(task.message_id)
        store.mark_completed(task.message_id, "<reply@example.com>", "/tmp/workspace")
        stored = store.get_task(task.message_id)
        self.assertEqual(stored.status, TaskStatus.COMPLETED)
        stats = store.get_stats()
        self.assertEqual(stats["completed"], 1)


if __name__ == "__main__":
    unittest.main()
