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

## thoughts from the hostpital

Most nodes in a json schema are a schema, this is expressed in the intermediate model, where every node is a schema.

In rust not every type is a schema, there are mote types than schemas. One example is types. A schema may be many simple types, in rust, every simple type is a new type.

So first we create an arena based o the intermediate model that has all types that need to be generated. Then we can easily extract the requirements via selectors.

### simple types

Every simple type on a schema will create a new type that represents the simple type. The node that contains the simple type will be an enum of these types.

Validation rules are on the node type.

### properties

Properties are their own schema already and there for their own type, but there we also create a type for the property on the parent schema. This is usually an alias to the actual property type. But when intersection happens (via all-of or any-of) this type is an intersection.

### map keys (additional items)

The key of a map should be a new type. This key can have validation and we should be consequent wit the new type pattern.

### other sub-types

we could create other sub-types if needed, for instance for the one-of, any-of, all-of, if-then-else types. This does not seem to be useful right now, but it might be at some point.
