# videostream-sys

Low-level Rust bindings for the DeepView VideoStream Library "VSL".  To use VideoStream from Rust please refer to the videostream-rs project.

# Compiling

This crate requires libvideostream to be installed but provides the videostream.h so the -dev package is not required.  The Rust bindings are generated from the bundled videostream.h and are updated by the package maintainers.

# Cross Compiling

When cross-compiling please ensure your `$HOME/.cargo/config.toml` has the following, otherwise you can expect compilation errors.

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

# Support

Professional support for DeepView AI Middleware, including this library, is provided by Au-Zone Technologies through the [DeepView Support Portal][1].

[1]: https://support.deepviewml.com
