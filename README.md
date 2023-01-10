# Teo FFI

Compile the dynamic library.
```
cargo build
```

Compile the foreign program.
```shell
gcc example.c -o example -lteo_ffi -L./target/debug
```

Run.
```shell
./example
```
