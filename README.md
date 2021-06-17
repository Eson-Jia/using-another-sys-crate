# Using another sys crate

使用 `links`关键字的时候,crates 可能会设置一些元数据,这些元数据可以被依赖该`crate`的其他 crate 读取. 这提供了一个在 crate 之间交换信息的机制.在这里示例中,我们将会创建一个 C 库,这个库通过
libz-sys crate 来使用 zlib. 如果你有一个依赖于`zlib`的 C 库,你可以利用`libz-sys`crate来自动查找或构建它.这对于跨平台支持非常有用,例如通常不安装 zlib 的 windows 平台.
libz-sys 设置 include 元数据可以告诉其他包去哪里寻找 zlib 头文件.我们的编译脚本可以通过`DEP_Z_INCLUDE`环境变量来读取这些元数据.

```toml
# Cargo.toml

[package]
name = "zuser"
version = "0.1.0"
edition = "2018"

[dependencies]
libz-sys = "1.0.25"

[build-dependencies]
cc = "1.0.46"
```

```rust
//build.rs
fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("src/zuser.c");
    if let Some(include) = std::env::var_os("DEP_Z_INCLUDE") {
        cfg.include(include);
    }
    cfg.compile("zuser");
    println!("cargo:rerun-if-changed=src/zuser.c");
}
```

`libz-sys`做完这些繁重的工作之后,C 的源码现在可以包含`zlib`的头文件了,即使系统没有安装`zlib`也应该能够找到这些头文件.

[src/zuser.c](./src/zuser.c)

## debug

### 无法编译

完成以上代码之后,发现代码无法编译,运行`cargo build`有以下报错:

```shell
   Compiling using-another-sys-crate v0.1.0 (/home/ubuntu/CLionProjects/using-another-sys-crate)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-Wl,--eh-frame-hdr" "-L" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.0.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.1.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.2.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.3.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.4.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.5.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.6.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.7.rcgu.o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.using_another_sys_crate.64wdhohf-cgu.8.rcgu.o" "-o" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps/using_another_sys_crate-a7c246bd5da78ed4.3vprg6cpjquj15gd.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/deps" "-L" "/home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/build/using-another-sys-crate-41dc6df5b45edb3e/out" "-L" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lzuser" "-Wl,--no-whole-archive" "-Wl,--start-group" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-0a9489cf400f65e4.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-ff5dc44c66f8c479.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-3317c66a83501f9c.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-5d8dec11fc25537d.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-3af2a9328550e2a6.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-3092e2ecef0f49f1.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-29b776c021389465.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-e8873bd287db0d28.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-1171b49d77e47426.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-c29894d22dc88b51.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-796a7750df3d8218.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-ff7772d803d3e0de.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-dae3eac9cfa44200.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-0fa02f580e987af5.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-7f7254233be843ed.rlib" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-1395c6db3d116086.rlib" "-Wl,--end-group" "/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b4bd87926720b651.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: /home/ubuntu/CLionProjects/using-another-sys-crate/target/debug/build/using-another-sys-crate-41dc6df5b45edb3e/out/libzuser.a(zuser.o): In function `version':
          /home/ubuntu/CLionProjects/using-another-sys-crate/src/zuser.c:7: undefined reference to `zlibVersion'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: could not compile `using-another-sys-crate`

To learn more, run the command again with --verbose.
```