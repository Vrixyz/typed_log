# Typed logging

When making a library, an author knows what information make sense to log. Often, that information is typed.

Rust's log facade is forcing Rust's users to use a text format for logging.

While text works to convey information, it may not be the best way to do so.

Tools like [Sentry](https://sentry.io) enable you to use those text logs quite comfortably,
but advanced logging visualization requires custom approaches, often involving expensive an error-prone text deserialization.

[Rerun](https://rerun.io/) logging for a geometry crate for example, could help with understanding algorithms, or track bugs more easily.

This crate offers an alternative approach to logging, where library author is in charge of what makes sense to log, and users are in charge on how to log it.

## WIP

This crate is very WIP, do not use, but! it may very well have more features at some point:
- macros à la log crate:
  - to opt out of logging on release builds
  - to filer logs depending on environment variables
- tracing compatibility
  - span information is great and may unlock parallel logging visualizations...
