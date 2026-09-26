# Architecture journal

This records the decisions behind the broker. [GOAL.md](GOAL.md) defines the checkpoints and finish line. Update this page when a decision changes; do not turn it into an implementation guide.

## What we are building

A single-node broker accepts records from producer processes. Consumer processes read records by offset. Records are ordered within a partition. Eventually, acknowledged records survive a broker restart and consumers can resume from saved offsets.

```text
producer ──publish──> TCP handler ──> broker ──> partition log
consumer <──record─── TCP handler <── broker <── partition log
```

For example, publishing the bytes for `hello` to an empty partition returns offset `0`. Reading offset `0` returns those bytes. Reading offset `1` reports that no record exists there.

## Responsibilities

- A TCP handler reads one client's requests and writes its responses. It does not own records.
- The broker finds the requested topic and partition and delegates each operation to its log.
- A partition log owns its records and next offset. Assigning an offset and appending its record form one operation.

```text
two producers ──> shared broker state ──> one partition log
                                    offset 0, then offset 1
```

The order of two simultaneous publishes is unspecified. Their offsets must be distinct, and a reader must see records in offset order.

## Decisions we have made

- Records contain bytes. The broker does not interpret the payload as text.
- Client handlers may run concurrently from the first networked version. They share broker state through `Arc<Mutex<...>>`. `Arc` shares ownership; `Mutex` protects access.
- Hold the lock only while looking up and operating on a log. Release it before waiting for network input or writing a response.
- Start with one in-memory partition log, while keeping handler, broker, and log responsibilities separate. Add disk storage and then multiple logs inside those boundaries.
- Requests should identify a topic and partition from the start, even when only one is supported. A read also names an offset. A successful publish returns its assigned offset.

The first lock can cover the whole broker. If independent partitions need more concurrency later, move locking into each partition without changing the client protocol or log rules.

## Decisions to make when needed

- Define the wire framing, request and response shapes, error replies, and maximum request size before building the TCP path. TCP carries bytes, not complete requests.
- At the durable-log checkpoint, define exactly when a publish reply is sent and what it guarantees after a crash. Do not claim durability for the in-memory version.
- Set connection limits and timeouts before accepting unbounded concurrent clients.

## Current journal entry

- **Checkpoint:** 1. The repository has no implementation yet.
- **Proof:** Run a broker; publish from one client process; read the same bytes and offset from another; report an absent offset cleanly. Allow the clients to connect at the same time.
- **Next design step:** Agree on one publish request and response, including how the receiver finds the end of a request and rejects an oversized one.
- **For the AI coach:** Let Abhishek write the Rust. Explain one boundary or decision at a time, then give one small task. Check code and tests before updating this entry. Keep later checkpoint work out of the current task.
