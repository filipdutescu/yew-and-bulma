# 0.4.0 (2023-04-11)

The major point of this release is to add all of the
[Bulma layout elements](https://bulma.io/documentation/layout/). To go alongside
those, various examples were also added, to provide a nice starting point and
reference.

Here is a more detailed list of changes:

## Features

* Implement Yew components for all [Bulma layout elements](https://bulma.io/documentation/layout/):
  * Add the [container element](https://bulma.io/documentation/layout/container/)
  * Add the [level element](https://bulma.io/documentation/layout/level/)
  * Add the [media elements](https://bulma.io/documentation/layout/media-object/)
  * Add the [hero element](https://bulma.io/documentation/layout/hero/)
  * Add the [section element](https://bulma.io/documentation/layout/section/)
  * Add the [footer element](https://bulma.io/documentation/layout/footer/)
  * Add the [tile element](https://bulma.io/documentation/layout/tiles/)

## Other

* Fix the example page titles to name the actual showcased element
* Update all examples to use the container element where needed, to look a bit
  prettier
* Add an [`xtask`][xtask] crate for generating and showing code coverage. Code
  coverage will be added to each PR and can be checked at-glance in the README

# 0.3.0 (2023-04-02)

The main change brought by this release is the addition of
[Column components][0.3.0-col]. Some internal changes are made to improve QoL
for development, such as updates to the CI actions and the addition of a
[macro crate](https://crates.io/crates/yew-and-bulma-macros).

## Features

* Add the [Column components][0.3.0-col]
* Convert to a workspace
* Add the [yew-and-bulma-macros](https://crates.io/crates/yew-and-bulma-macros)
  crate, mostly to provide base component properties (for now)
* Add [HTML events][0.3.0-ev] to all component properties
* Forbid unsafe code from both crates
* Set viewport as the key in viewport combination for properties (ie size,
  display)

## Other improvements

* Add commit lints
* Add issue templates
* Check nightly errors as well

[0.3.0-col]: https://bulma.io/documentation/columns/
[0.3.0-ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element#events

# 0.2.1 (2023-03-25)

The previous version broke examples due to an increment of the crate version.
This release addresses the issue by updating the examples.

## Fixes

* Examples use the proper crate version

# 0.2.0 (2023-03-25)

The major point of this release is to add all of the
[Bulma elements](https://bulma.io/documentation/elements/). To go alongside
those, various examples were also added, to provide a nice starting point and
reference.

Here is a more detailed list of changes:

## Features

* Implement Yew components for all [Bulma elements](https://bulma.io/documentation/elements/):
  * Add the [block element](https://bulma.io/documentation/elements/block/)
  * Add the [box element](https://bulma.io/documentation/elements/box/)
  * Add the [button element](https://bulma.io/documentation/elements/button/)
  * Add the [content element](https://bulma.io/documentation/elements/content/)
  * Add the [delete element](https://bulma.io/documentation/elements/delete/)
  * Add the [icon element](https://bulma.io/documentation/elements/icon/)
  * Add the [image element](https://bulma.io/documentation/elements/image/)
  * Add the [notification element](https://bulma.io/documentation/elements/notification/)
  * Add the [progress bar element](https://bulma.io/documentation/elements/progress/)
  * Add the [table elements](https://bulma.io/documentation/elements/table/)
  * Add the [tag element](https://bulma.io/documentation/elements/tag/)
  * Add the [title elements](https://bulma.io/documentation/elements/title/)
* Add a generic color helper, in addition to the existing text and background
  color ones
* Update the `ClassBuilder` to support the generic color helper and toggle the
  light variants of colors
* Add a generic size utility, which defines the most commonly used Bulma sizes

## Fixes

* Fix the documentation for constants as some did not explain correctly use
cases and meaning
