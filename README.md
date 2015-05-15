# rust-atom

[![atom_syndication on Crates.io](https://meritbadge.herokuapp.com/atom_syndication)](https://crates.io/crates/atom_syndication)

[Documentation](https://vtduncan.github.io/rust-atom/)

Library for serializing the Atom web content syndication format

## Examples

### Writing

```rust
use atom::{Feed, Entry};

let entry = Entry {
    id: String::from("urn:uuid:4ae8550b-2987-49fa-9f8c-54c180c418ac"),
    title: String::from("Ford hires Elon Musk as CEO"),
    updated: String::from("2019-04-01T07:30:00Z"),
    ..Default::default()
};

let feed = Feed {
    id: String::from("urn:uuid:b3420f84-6bdf-4f46-a225-f1b9a14703b6"),
    title: String::from("TechCrunch"),
    updated: String::from("2019-04-01T07:30:00Z"),
    entries: vec![entry],
    ..Default::default()
};

let atom_string = feed.to_string();
```

### Reading

```rust
use atom::Feed;

let atom_str = r#"
<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <id>urn:uuid:b3420f84-6bdf-4f46-a225-f1b9a14703b6</id>
  <title>TechCrunch</title>
  <updated>2019-04-01T07:30:00Z</updated>
  <entry>
    <id>urn:uuid:4ae8550b-2987-49fa-9f8c-54c180c418ac</id>
    <title>Ford hires Elon Musk as CEO</title>
    <updated>2019-04-01T07:30:00Z</updated>
  </entry>
</feed>
"#;

let feed = atom_str.parse::<Feed>().unwrap();
```
