This freeware clones NYT Spelling Bee for the CLI. In zsh or bash terminals, after cloning this repo, build from the source by doing the following:

```sh
cd path/to/libre_bee # replace with actual path
sudo dnf install cargo # install cargo if you have not already
cargo build --release # build a release version in the target directory
```

You can then run it by:

```sh
./target/release/libre_bee # in the same directory
```

More information and help to be provided at a later date.