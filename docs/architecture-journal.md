# Architecture journal

This records decisions behind the broker. [GOAL.md](GOAL.md) defines the checkpoints and three-broker finish line. Update this page when a decision changes; do not turn it into an implementation guide.

## What we are building

Producer processes append byte records to topic partitions. Consumer processes read records by offset. Records are ordered within a partition. Later checkpoints add durable storage, consumer groups that divide partitions among workers, and replication across three brokers with one-broker failure tolerance.

```text
producer --publish--> TCP handler --> broker --> partition log
consumer <--record--- TCP handler <-- broker <-- partition log
```

For example, publishing the bytes for `hello` to an empty partition returns offset `0`. Reading offset `0` returns those bytes. Reading offset `1` reports that no record exists there. The in-memory checkpoint loses those bytes if the broker restarts; later guarantees depend on the acknowledgement rules we define for durable storage and replication.

## Responsibilities in the first broker

- A TCP handler reads one client's requests and writes its responses. It does not own records.
- The broker finds the requested topic and partition and delegates each operation to its log.
- A partition log owns its records and next offset. Assigning an offset and appending its record form one operation.

```text
two producers --> shared broker state --> one partition log
                                     offset 0, then offset 1
```

The order of two simultaneous publishes is unspecified. Their offsets must be distinct, and a reader must see records in offset order.

Later, consumers own the work they perform on records. Group coordination assigns partitions to workers; it does not execute their work. Replication adds a copy of each partition log on other brokers. Only an authorized broker may assign offsets for a partition at a given time. The exact coordination and recovery rules remain open until those checkpoints.

## Decisions we have made

- Records contain bytes. The broker does not interpret the payload as text.
- Client handlers may run concurrently from the first networked version. They share broker state through `Arc<Mutex<...>>`. `Arc` shares ownership; `Mutex` protects access.
- Hold the lock only while looking up and operating on a log. Release it before waiting for network input or writing a response.
- Start with one in-memory partition log, while keeping handler, broker, and log responsibilities separate. Add disk storage and then multiple logs inside those boundaries.
- Requests should identify a topic and partition from the start, even when only one is supported. A read also names an offset. A successful publish returns its assigned offset.
- Network requests use a fixed four-byte big-endian length prefix. Reject request bodies larger than 1 MiB before reading or allocating the body. Payloads are raw bytes in a binary protocol, not JSON text.
- The long-term target is three brokers that tolerate one broker failure. This target does not imply that the in-memory or single-broker checkpoints provide the same guarantee.

The first lock can cover the whole broker. If independent partitions need more concurrency later, move locking into each partition without changing the client protocol or log rules.

## Decisions to make when needed

- Before building the TCP path, finish the request and response shapes and error replies. TCP carries bytes, not complete requests.
- At the durable-log checkpoint, define exactly when a publish reply is sent and what it guarantees after a broker crash.
- Before accepting unbounded concurrent clients, set connection limits and timeouts.
- At the consumer-progress checkpoint, decide where positions are saved and what happens if a consumer fails between processing a record and saving its position.
- At the consumer-group checkpoint, define membership, partition assignment, worker failure detection, reassignment, and how a replacement resumes. A group should assign each partition to one worker at a time, but failed work may need to run again.
- At the replication checkpoint, define leadership, how records and saved consumer positions survive one broker failure, which acknowledgements are safe, failover, and protection against a former leader accepting writes. Test these rules under failure before claiming the finish-line guarantee.

## Starting point

- **Initial checkpoint:** 1. The repository had no implementation when this plan was written. Determine later progress from the code and tests.
- **Proof for checkpoint 1:** Run a broker; publish from one client process; read the same bytes and offset from another; report an absent offset cleanly. Allow the clients to connect at the same time.
- **Next implementation step:** Build the one in-memory partition log, then connect it to the broker's TCP path. Framing and the request-size limit are already decided; finish the remaining protocol details when the TCP path needs them.
- **For the AI coach:** Let Abhishek write the Rust. Explain one boundary or decision at a time, then give one small task. Keep later checkpoint work out of the current task while keeping the three-broker finish line visible.
