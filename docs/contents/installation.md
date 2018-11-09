# Installing dmenv

## Using the pre-compiled binaries

The easiest way is to download the matching binary from the [releases page](https://github.com/TankerHQ/dmenv/releases) for your platform and put it
somewhere on in your $PATH:

### Linux, macOS

```console
cd ~/.local/bin
curl-L https://github.com/TankerHQ/dmenv/releases/download/v<version>/dmenv-<platform> -o dmenv
chmod u+x dmenv
```

### Windows

Download the `dmenv-windows.exe` file and save it for instance in `c:\path\to\python\Scripts\dmenv.exe`.

## Installing from source

If you prefer, you can also [install rust](https://www.rust-lang.org/en-US/install.html) and install dmenv with `cargo install dmenv`.
