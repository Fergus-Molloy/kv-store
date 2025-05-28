# KV Store

Basic implementation of a key-value store in rust, served over TCP

## Protocol
messages must be prefixed with their size represented as a big endian u32.

### Put

```
P|<key>|<value>
```

### Get

```
G|<key>
```
