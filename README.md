XML Struct Gen
==============

This is a system for working with structured XML formats from Rust in a type-safe way.
It takes a sample XML document as input, and generates structs to represent that data,
along with code to use xml-rs to serialize/deserialize that data to XML.

It is also pragmatic that the generated code may need to process data that has elements/
attributes that it does not recognise, and it can handle these safely and round-trip them
losslessly if necessary.

## Limitations

* This is not a general-purpose DOM library for handling arbitrary or unstructured
  data - this is for dealing with well-structured XML formats.
* Does not attempt to handle data where text and elements appear as children of
  the same element
* Does not currently maintain ordering of different types of elements, though ordering of
  elements of the same type is maintained.
  * For example, on a podcast, the ordering of the `description` sub-element and `pubDate`
   sub-element will always be serialized in the order defined by the generated code.
  * However, the ordering of the episodes will be maintained.
  * I have some ideas for potential fixes or workarounds so if this is relevant for a
    format you are working with, please file an issue!
* There may be bugs or incorrect behaviours especially around namespaces.
* Generated code depends on the [xml-struct-types crate](https://crates.io/crates/xml-struct-types)
  which contains some error types and util fn's and as of yet no guarantees are made about semver or
  forward/backward compatibility.

## Usage

Install the generator

    cargo install xml-struct-gen@0.0.1-alpha

Add the dependency types

    cargo add xml-struct-types@0.0.1-alpha

Generate some structs! You can specify multiple sample documents and
types will be gathered from each one.

    xml-struct-gen --input podcast1.xml \
        --input podcast2.xml \
        --output src/podcast_structs.rs

Import the structs into your project

```rust
#[allow(dead_code, unused)]
pub mod podcast_structs;
```

Generate the XML for an aweomse podcast

```rust
    let rss_doc = RssDocument {
        channel_elems: vec![Channel {
            rss_channel_title_elems: vec![RssChannelTitle {
                value: Some("My awesome podcast".into()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    rss_doc.write_document(&mut stdout()).unwrap();
```
