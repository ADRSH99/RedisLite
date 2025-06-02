# RedisLite

RESP (Redis Serialization Protocol) Specification

We've adopted the official RESP to ensure seamless communication in our system


Features:
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

git clone https://github.com/Matrx123/redis-like-clone
cd redis-like-clone
cargo build --release


