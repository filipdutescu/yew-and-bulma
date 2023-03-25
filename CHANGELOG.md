# 0.2.0 (2023-03-25)

The major point of this release is to add all of the [Bulma elements](https://bulma.io/documentation/elements/).
To go alongside those, various examples were also added, to provide a nice starting point and
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
* Add a generic color helper, in addition to the existing text and background color ones
* Update the `ClassBuilder` to support the generic color helper and toggle the light variants
  of colors
* Add a generic size utility, which defines the most commonly used Bulma sizes

## Fixes

* Fix the documentation for constants as some did not explain correctly use cases and meaning
