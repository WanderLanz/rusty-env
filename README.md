# rusty-env

List environment variables
available to a rust process, for lazy rustaceans.

Simply because some environment variables are not universal and this,
obviously, works everywhere rust (with [clap](https://crates.io/crates/clap)) can.

There probably already exists a much better way to do this, but I am lazy and couldn't find it within ~5 minutes of searching.

## Usage

```bash
$ rusty-env
FOO="This is a value over 10 characters"
BAR="7 chars"
```

only keys

```bash
$ rusty-env -k
FOO
BAR
```

truncate values

```bash
$ rusty-env -v 10
FOO="This is a ..."
BAR="7 chars"
```

replace ellpsis in truncated values with specified string

```bash
$ rusty-env -v 10 -s "🦀"
FOO="This is a 🦀"
BAR="7 chars"
```
