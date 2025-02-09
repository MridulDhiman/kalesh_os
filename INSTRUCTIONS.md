
### Build and Compile for bare metal target

```bash
#!/bin/bash
rustup add target thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

### compile it for the Linux host system

```bash
cargo rustc -- -C link-arg=-nostartfiles
```