
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

### Build and compile the custom kernel with it's own target triple specification
```bash
cargo build --target x86_64-kalesh_os.json
```

### Boot executable disk image in QEMU virtual machine

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-kalesh_os.bin
```



