<!-- cargo-rdme start -->

Macro to extract data from deeply nested types representing GraphQL results

# Suggested workflow

1. Generate query types using [cynic] and its [generator]
1. Use [insta] to define an inline snapshot test so that the query string is visible in the
   module that defines the query types
1. Define an `extract` function that takes the root query type and returns the data of interest
1. Inside `extract`, use `extract!` as `extract!(data => { ... })`
1. Inside the curly braces, past the query string from the snapshot test above
1. Change all node names from `camelCase` to `snake_case`
1. Add `?` after the nodes that are nullable
1. Add `[]` after the nodes that are iterable

[cynic]: https://cynic-rs.dev/
[generator]: https://generator.cynic-rs.dev/
[insta]: https://insta.rs/

<!-- cargo-rdme end -->
