# Prototype Assets

This folder contains prototype files for each of the examples. The reason we separate each into its own folder rather
than all together is that Bevy's reflection relies on absolute module path names. This means that we can't "share" types
between examples (`a::Foo` is not the same as `b::Foo`).

For your own projects, this won't be much of a problem since they likely share the same crate.

Another important note, the `.prototype.` is required in all file names in order for the asset loader to recognize them
as prototype files. By default, this crate supports YAML, JSON, and RON (the latter two locked behind a feature). So
you'll have to, for example, name them `foo.prorotype.json`.

If you prefer to use a custom extension, you can specify that using the `ProtoConfig`. However, keep in mind the
extension rule with templates: templates that do not have a extension take on the extension of the current file. Also
note that this will only apply to custom prototypical types (not the default `Prototype` struct).