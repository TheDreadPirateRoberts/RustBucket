
# Setting up a development environment

This assumes Visual Studio Code on Windows.

## Installing Visual Studio Code

Download and install Visual Studio Code from the [download page](https://code.visualstudio.com/Download).

## Installing Rust and RLS

Follow the [instructions](https://www.rust-lang.org/en-US/install.html) for running `rustup-init` using the default `stable` toolchain.  You'll may need to install the Visual C++ build tools.

Once Rust is installed, install [RLS](https://github.com/rust-lang-nursery/rls), which is used by Visual Studio Code for Intellisense, go to definition, etc.

Update `rustup` and your `stable` toolchain:

```
rustup self update
rustup update stable
```

Install the RLS components:
```
rustup component add rls-preview
rustup component add rust-analysis
rustup component add rust-src
```

## Configuring Visual Studio Code

Open Visual Studio Code and click the Extenstions button in the Side Bar.  Search for and install the `Rust (rls)` extension.

This project has Visual Studio Code workspace settings that configure RLS to use the stable toolchain rather than it's default of nightly, when you open the project if the installation has been successful you will see RLS status information in the editor's status bar.

You can run Cargo's build task using `CTRL` + `Shift` + `b` or run the tests by opening the command pallete (`CTRL` + `Shift` + `p`) and entering `run test task`.

### Debugging

Install the `C/C++` Visual Studio Code extension from the Extensions tab, this enables debugging of executable files.

*Temporary step until rustc includes debugging information in executables*:
 Copy the two files from `%HOMEPATH%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\etc` to `%HOMEPATH%\.vscode\extensions\ms-vscode.cpptools-0.12.2\debugAdapters\vsdbg\bin\Visualizers`.

This project includes a `lauch.json` with a debug task pre-configured to target `target/debug/main.exe` and to re-build the project before starting the debugger.

Add a breakpoint to `src\bin\main.rs` and press `F5` to start debugging.  If all has gone well you should stop on the breakpoint.
