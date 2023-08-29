# jns42-generator, Rust edition

...

references:

-   [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
-   [`Hash`](https://doc.rust-lang.org/std/hash/index.html).

## types

Rust types generated from the json schema specification are represented as new-types. This make (de)serialization easy and allows us to [parse-don't-validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/). The `Deref` trait is implemented for every new-type to allow for better ergonomics.

### never

This is the `false` type in Json Schema. In rust this is implemented as a struct that never validates. So deserialization of this type will never happen!

### any

We call the `true` type in Json Schema the `any` type. This type represents the `std::any::Any` type in rust. We need to write custom (de)serialization logic for this type.

### null

The `null` type is represented as the unit type, `()` in rust. Custom validation is not available.

### boolean

The `boolean` type is represented as a rust `bool`. This type may allowed values specified as options.

### integer

The integer type is a represented as `i64` in rust. Validation is

-   allowed values (options)
-   minimum and maximum value (inclusive or exclusive)
-   multiple of validation

### number

Custom validation is the same as with the integer type

### string

The string type is a represented as `String` in rust. Validation is

-   allowed values (options)
-   minimum and maximum length
-   pattern validation (regular expression)

### tuple

The tuple type is represented as a rust tuple. The tuple has the following validation:

-   uniqueness, no 2 items in the tuple are the same.

### array

The array type can be represented in rust as a `Vec`.

-   minimum and maximum items
-   unique items

Unique items is not enforced via a `BTreeSet` or a `HashSet`, if wou would do this we would have to implement `Ord` or `Hash` on all generated types.

### object

Objects are represented as rust `struct`'s. Fields that are not required are `Option`'s.

### record

Record types are `HashMap`'s in rust, the key being a `String`. Custom validation is:

-   minimum and maximum properties

### one-of

...

### any-of

...

### all-of

...

### if-then-else

...
