# Contributing to `gdext`

At this stage, we appreciate if users experiment with the library, use it in small projects and report issues and bugs they encounter.

If you plan to make bigger contributions, make sure to discuss them in a [GitHub issue] first. Since the library is evolving quickly, this avoids that multiple people work on the same thing or implement features in a way that doesn't work with other parts. Also don't hesitate to talk to the developers in the `#contrib-gdext` channel on [Discord]!

## Check script

The script `check.sh` in the project root can be used to mimic a CI run locally. It's useful to run this before you commit, push or create a pull request:

```
$ check.sh
```

At the time of writing, this will run formatting, clippy, unit tests and integration tests. More checks may be added in the future. Run `./check.sh --help` to see all available options.

If you like, you can set this as a pre-commit hook in your local clone o