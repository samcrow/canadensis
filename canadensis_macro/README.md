# Macros to generate code from Cyphal DSDL

## Dependencies

The generated code depends on these libraries:
```toml
half = "2.2"
heapless = "0.7.7"
zerocopy = "0.6.0"
canadensis_encoding = "0.3.0" # (version may be incorrect)
canadensis_core = "0.3.0" # (version may be incorrect)
```

For testing, it also depends on this, which should be under `dev-dependencies`:
```toml
memoffset = "0.8.0"
```
