# Mini Kafka: project goal

## Big goal

Build a reliable, single-node message broker in Rust that separate producer and consumer programs can use over a small, documented network protocol. Producers publish records to partitioned topics. Consumers read ordered records from an offset and can resume after disconnecting. The broker keeps acknowledged records across restarts and handles concurrent clients without corrupting data or exhausting resources.

The purpose is to learn Rust by designing and writing the broker myself, while working through real broker concerns: ownership of shared state, concurrency, disk I/O, recovery, protocol design, and failure handling.

## Checkpoints

1. **End-to-end flow:** Run a broker process, publish records from a client, and read them from another client through a documented protocol. Start with one in-memory log.
2. **Durable log:** Assign offsets on append, store records on disk, and recover the log after a restart. Define when an append is acknowledged and what that acknowledgement guarantees.
3. **Topics and partitions:** Support multiple topics and partitions. Preserve order within each partition and let producers choose a partition. Consumers select a topic, partition, and starting offset.
4. **Concurrent operation:** Serve multiple producers and consumers safely. Handle malformed requests, disconnected clients, slow readers, and bounded memory use.
5. **Consumer progress:** Let consumers save and resume their position. Demonstrate replay and recovery after a consumer restart.

## Finish line

From separate client processes, publish and read records across multiple partitions, restart the broker and a consumer, and continue from saved offsets without losing acknowledged records. Tests exercise ordering, restart recovery, concurrent access, invalid input, and resource limits.

Kafka wire compatibility, replication, distributed coordination, and a production deployment are outside this goal. A custom protocol keeps the work focused on broker behavior.

I will write the Rust implementation. AI may help with design decisions, code review, and understanding compiler or test feedback.
