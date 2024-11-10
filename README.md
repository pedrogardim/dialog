# Dialog

Dialog is an open-source, horizonally scalable, distributed, and fault-tolerant chat backend and general porpuse realt-time messaging system.

## Architecture

Dialog is ment to be a distributed system, with multiple nodes that can be added or removed at any time. This technological stack is highly inspired by the one used by Discord. The system is composed of three main components:

- **Gateway server**: The Rust gateway service is responsible for handling the messages. It is a websockets server that the users connect to. The service is also responsible for handling authentication and authorization. The Rust service is subscribed to a Redis pubsub, which is used to synchronize messages between the Rust services. The messages are stored in ScyllaDB.
- **Message Broker Layer**: It is used to synchronize messages between the gateway services. It is a message broker that allows the gateways to communicate with each other. We use Redis Cluster pubsub as the message broker.
- **Distributed Data Storage Cluster**: We use ScyllaDB as the storage layer. It's a distributed NoSQL database that is used to store the messages. It is a highly available and fault-tolerant database that can scale horizontally.

## Variants
- **Mini**: A simple version of Dialog, with only one Rust service instance. This variant is useful for smaller apllications, and can run on a single Docker container. The messages are stored in a SQLite database or on a external SQL service.

- **Standard**: The standard version of Dialog, on which every component is horizontally scallable. This variant is useful for applications that require high availability and fault tolerance. The messages are stored in a ScyllaDB database.

- **Cloud**: Managed cloud version of Dialog. With Dialog Cloud, you don't have to worry about managing the infrastructure, and you can focus on building your application, while having a highly available and fault-tolerant chat backend.
