# FAQ

#### How do I upgrade a dependency?

Just run `dmenv lock` again. If something breaks, either fix your code or
use more precise version specifiers in `setup.py`, like `foobar < 2.0`.

### How do I skip a dependency on certain platform or Python version ?

Use [environment markers](https://www.python.org/dev/peps/pep-0508/#environment-markers) in
the `requirements.lock` file. For instance:

```text
black==18.6b4 ; python_version >= '3.6'
```

Note: this means you have to patch the `requirements.lock` file by hand every time
you run `dmenv lock`.

#### How do I depend on a git specific repo/branch?

Use a git URL in the `requirements.lock` file, where the part after `#egg=`
matches the name of the dependency in the `setup.py` file:

```text
https://gitlab.com/foo/bar@my-branch#egg=bar
```

Note: this means you have to patch the `requirements.lock` file by hand every time
you run `dmenv lock`.

An other way of achieving the same result without having to patch the `requirements.lock` file by hand is
to use a local pipy mirror.

#### How do I use an other Python interpreter?

You can either:

* Modify your PATH environment variable so that it appears there. (For instance, with [pyenv](https://github.com/pyenv/pyenv)).
* Prefix all the `dmenv` commands with a `--python /path/to/other/python` flag.


#### Why Rust?

* Because it has excellent support for what we need: manipulate paths and run commands in a cross-platform way
* Because it's my second favorite language
* Because distribution is really easy
* Because by *not* using Python at all `dmenv` is less likely to break if something on your system changes.
