# RedisLite 🦀🚀

A lightweight, single-threaded Redis-like key-value store written in Rust, powered by Tokio for async I/O.
Built for learning, experimenting, and extending — not for production (yet)!.

We've adopted the official RESP(Redis Serialization Protocol) Specification to ensure seamless communication in our system

Features:

    Basic Redis protocol (RESP) support
    In-memory key-value store
    Asynchronous handling of multiple clients using tokio

Implemented Data Structures

    Strings
        SET <key> <value>, GET <key>, DEL <key>
        TTL <key>

    Lists
        LPUSH <key> <value>, RPUSH <key> <value>
        LPOP <key>, RPOP <key>
        LRANGE <key> <start> <stop>

    Sets
        SADD <key> <member>, SREM <key> <member>
        SISMEMBER <key> <member>, SMEMBERS <key>

    Sorted Sets
        ZADD <key> <score> <member>, ZREM <key> <member>
        ZRANGE <key> <start> <stop> [WITHSCORES]

Getting Started
Prerequisites

    Rust 1.65 or newer
    Cargo package manager

Installation

    git clone https://github.com/ADRSH99/RedisLite
    cd RedisLite
    cargo build --release


