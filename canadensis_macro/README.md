# Macros to generate code from UAVCAN DSDL

## Dependencies

The generated code depends on these libraries:
```toml
half = "1.7.1"
heapless = "0.7.7"
zerocopy = "0.6.0"
canadensis_encoding = "0.2.0" # (version may be incorrect)
canadensis_core = "0.2.0" # (version may be incorrect)
```

For testing, it also depends on this, which should be under `dev-dependencies`:
```toml
memoffset = "0.6.4"
```