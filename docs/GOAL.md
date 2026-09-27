# Mini Kafka: project goal

## Big goal

Build a small distributed message broker in Rust that separate producer and consumer programs use through a documented network protocol. Producers append byte records to partitioned topics. Consumers read ordered records by offset, replay them, and resume after disconnecting. Consumer groups divide partitions among workers. Three brokers replicate records so the system continues after one broker fails without losing records it has acknowledged under its durability guarantee.

The purpose is to learn Rust by designing and writing the broker myself. The work should exercise ownership of shared state, types and errors, concurrency, disk I/O, recovery, network protocols, and coordination across machines. Each checkpoint must produce observable behavior before the next one adds complexity.

## Architecture direction

Start with concurrent TCP client handlers sharing one broker's state. The broker routes requests to partition logs; each log owns record order and offset assignment. Begin with one in-memory log, then add disk storage, topics, and partitions within those responsibilities. Consumer coordination and replication build on the working single-broker system. The [architecture journal](architecture-journal.md) records decisions already made and the next design step.

The three-broker finish line guides the work. Decide protocol details, acknowledgement guarantees, group coordination, and replication rules at their relevant checkpoints. Keep this roadmap stable; update the architecture journal when an agreed architecture decision changes.

## Checkpoints

1. **End-to-end flow:** Run one broker, publish bytes from one client process, and read them by offset from another through a documented protocol. Use the planned shared-state model and one in-memory log. A publish reply at this stage does not promise survival across restarts.
2. **Durable log:** Store records on disk, recover the log after a restart, and define when an append is acknowledged and what that acknowledgement guarantees on this single broker.
3. **Topics and partitions:** Support multiple topics and partitions. Preserve order within each partition and let producers choose a partition. Consumers select a topic, partition, and starting offset.
4. **Concurrent operation:** Test and harden concurrent producers and consumers. Handle malformed requests, disconnected clients, slow readers, and bounded use of connections and memory.
5. **Consumer progress:** Let consumers save and resume their positions. Demonstrate replay and recovery after a consumer restart. Define what saving a position means when processing fails.
6. **Consumer groups:** Coordinate multiple workers so each partition has one assigned worker within a group. Reassign partitions when a worker leaves or stops responding. Independent groups can read the same records for different purposes. Define how a reassigned worker resumes and when processing may repeat.
7. **Replication and failover:** Run three brokers, replicate partition records, and recover from one broker failing. Define which broker may assign offsets, when a publish is acknowledged, how a replacement takes over, and how an old broker is prevented from accepting writes after losing authority.

## Finish line

Run separate producer and consumer processes against three brokers. Publish and read records across multiple partitions, restart a consumer and resume from saved offsets, then stop one broker. The remaining brokers must take over and continue serving without losing acknowledged records or the saved positions needed to resume. Tests exercise ordering, replay, restart recovery, worker reassignment, broker failure, concurrent access, invalid input, and resource limits.

Kafka wire compatibility, a production deployment, and support for failures of multiple brokers at once are outside this goal. A custom protocol keeps the work focused on broker behavior. Exact delivery and acknowledgement guarantees must be stated explicitly rather than assumed from the name "Kafka."

I will write the Rust implementation. AI may help with design decisions, code review, and understanding compiler or test feedback.
