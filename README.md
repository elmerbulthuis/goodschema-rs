# jns42-generator, Rust edition

...

## flattening types, thoughts

So what is the problem. Well we can express union types in rust, as enums. We cannot express intersection types.

In JsonSchema there are 4 ways to compose types. OneOf, AnyOf, AllOf and via ref.

OneOf is like a union, so in rust, like an enum. We implement the the TryFrom trait for all of the subtypes. They will return the subtype.

AnyOf is type that is at least one of its subtypes. This could be represented in rust as a struct that has members that are allow of the subtypes, but optional. Then we implement the TryFrom and AsRef traits for all of the subtypes. They will return the subtype.

AllOf is where things get interesting. This is like an intersection type. Properties of objects should be merged and if we have an AllOf with conflicting types then the type may never exist. For instance id we would make an AllOf with a number and a string type, then this type can never be. For every subtype a From and an AsRef is implemented for the AllOf type.

ref works the same way AllOf does, only one type can be referenced via ref. This could be considered inheritance.

Another way to deal with AnyOf is to consider it more like AllOf. So to merge all properties, but make them optional. In case of conflicting types this would be solved with an enum.
