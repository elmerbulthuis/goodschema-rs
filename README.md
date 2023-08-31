# jns42-generator, Rust edition

references:

-   [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
-   [`Hash`](https://doc.rust-lang.org/std/hash/index.html).

## types

Rust types generated from the json schema specification are represented as new-types. This make (de)serialization easy and allows us to [parse-don't-validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/). The `Deref` trait is implemented for every new-type to allow for better ergonomics.

Composed types could be flattened, so if wou would have an all-of of all-ofs, then we could flatten them into one big all-of. Same goes for any-of and one-of.

Merging is an important part of generating types for rust. Merging may happen when using all-of or super types, or when a composed type is defined on a base type. Remember that an piece of json is valid when all schema's are considered valid. So when merging we should consider this.

types will be merged as followed:

-   same types will merge perfectly with same types
-   if one of the types is never, then the result is always never
-   if one of the types is any, then this is removed from the result

if the result of a merge is an empty list of types, then the result is any.

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

Unique items is not enforced via a `BTreeSet` or a `HashSet`, validating duplicate types would be impossible if we would use sets. Instead, when validating for uniqueness we can walk the vector and keep a set to check for uniqueness.

### object

Objects are represented as rust `struct`'s. Fields that are not required are `Option`'s.

### record

Record types are `HashMap`'s in rust, the key being a `String`. Custom validation is:

-   minimum and maximum properties

### one-of

One of is represented as an enum in rust. `TryFrom` and `From` is implemented for all types in the one-of.

This pseudo json

```json
{
    "a": {
        "types": ["string"]
    },
    "b": {
        "types": ["string"]
    },
    "c": {
        "oneOf": ["a", "b"]
    }
}
```

would be

```rust
struct A(String);
struct B(String);
#[serde(untagged)]
enum C {
    A(A)
    B(B)
}
impl TryFrom<C> for A {
    type Error = ();
    fn try_from(value: C) -> Result<Self, Self::Error> {
        match value {
            C::A(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl From<A> for C {
    fn from(value: A) -> Self {
        Self::A(value)
    }
}
// same TryFrom and From for type B
```

### any-of

All types are merged and put in an object that has all members as optional types.

This pseudo json

```json
{
    "string-type": {
        "types": ["string"]
    },
    "a": {
        "types": ["object"],
        "required": :["a"],
        "properties": {
            "a": "string-type"
        }
    },
    "b": {
        "types": ["object"],
        "properties": {
            "b": "string-type"
        }
    },
    "c": {
        "anyOf": ["a", "b"]
    }
}
```

would be

```rust
struct StringType(String);
struct A {
    pub a: StringType,
}
struct B {
    pub b: Option<StringType>,
}
struct C {
    a: Option<StringType>,
    b: Option<StringType>,
}
impl TryFrom<C> for A {
    type Error = ();
    fn try_from(value: C) -> Result<Self, Self::Error> {
        let a = value.a.ok_or(())?;
        Ok(Self { a })
    }
}
impl From<A> for C {
    fn from(value: A) -> Self {
        let a = Some(value.a);
        Self { a, b: None }
    }
}
impl TryFrom<C> for B {
    type Error = ();
    fn try_from(value: C) -> Result<Self, Self::Error> {
        let b = value.b;
        Ok(Self { b })
    }
}
impl From<B> for C {
    fn from(value: B) -> Self {
        let b = value.b;
        Self { a: None, b }
    }
}

```

### all-of

All of should result in a merge of all the types. This could also result in weird behavior if two types, like number and string, are merged. This would result in a never type, validation would always fail. `From` is implemented to convert from the all-of type.

This pseudo json

```json
{
    "string-type": {
        "types": ["string"]
    },
    "a": {
        "types": ["object"],
        "required": :["a"],
        "properties": {
            "a": "string-type"
        }
    },
    "b": {
        "types": ["object"],
        "properties": {
            "b": "string-type"
        }
    },
    "c": {
        "allOf": ["a", "b"]
    }
}
```

would be

```rust
struct StringType(String);
struct A {
    a: StringType,
}
struct B{
    b: Option<StringType>,
};
struct C {
    a: StringType,
    b: Option<StringType>,
}
impl From<C> for A {
    fn from(value: C) -> Self {
        Self {
            a: value.a
        }
    }
}
// same for type B
```

### if-then-else

If then else. Could result in two objects and an enum. If and then and the base schema are merged, this is one object. The second object is the base schema merged with the else schema. `From` and `TryFrom` are implemented. This type is a bit like the one of type.

The else and then schemas extend the base schema, that is why we merge them. The if schema is merges with the then schema because they both need to validate. If they fail then we validate the else schema.

This pseudo json

```json
{
    "string-type": {
        "types": ["string"]
    },
    "a": {
        "types": ["object"],
        "properties": {
            "a": "string-type",
            "b": "string-type"
        },
        "types": ["object"],
        "if": {
            "required": ["a"]
        },
        "then": {
            "required": ["b"]
        },
        "else": {
            "properties": {
                "c": "string-type"
            }
        }
    }
}
```

would result in the following rust

```rust
struct StringType(String);
struct AThen {
    a: StringType,
    b: StringType,
}
struct AElse{
    a: Option<StringType>,
    b: Option<StringType>,
    c: Option<StringType>,
};
#[serde(untagged)]
enum A {
    AThen(AThen),
    AElse(AElse),
}
impl TryFrom<A> for AThen {
    type Error = ();
    fn try_from(value: A) -> Result<Self, Self::Error> {
        match value {
            C::AThen(value) => Ok(value),
            _ => Err(()),
        }
    }
}
impl From<AThen> for A {
    fn from(value: AThen) -> Self {
        Self::AThen(value)
    }
}
// same TryFrom and From for type AElse

```

### not

Not only affects validation and may affect option-ness of a member of a structure.

### super-type

Super types behave a bit like type inheritance. Also they behave a bit like all-of. `From` is implemented.

This pseudo json

```json
{
    "string-type": {
        "types": ["string"]
    },
    "a": {
        "types": ["object"],
        "properties": {
            "a": "string"
        }
    },
    "b": {
        "super": "a",
        "types": ["object"],
        "properties": {
            "b": "string"
        }
    }
}
```

would be

```rust
struct StringType(String);
struct A {
    a: StringType,
}
struct B{
    a: StringType,
    b: StringType,
};
impl From<B> for A {
    fn from(value: B) -> Self {
        Self {
            a: value.a
        }
    }
}
```
