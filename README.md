# Tarrow

A simple library for versioned storage of data blobs (designed with small files in mind), backed 
by Postgres.

Uses a versioned storage mechanism largely inspired by Git (see section "Design Goals & 
Features" for deviation), with an experimental goal of nominating the database schema as a 
public API.

**Author** Rohan Prabhu <rohan@rohanprabhu.com>

> **NOTE** This is mainly an educational project - the main goal was to author this library as a 
> way to  learn rust, given which, till this particular notice stays, I wouldn't really 
> recommend using this in any project except for learning.
> 
> There are a few places in the code which are prefixed with "EDU" - these are essentially 
> comments for self that remind me of things that I somehow got to work, but without having an 
> understanding of why one solution worked over the other. The "EDU" comments are simply 
> reminders for me to go back and understand the issue at hand.

## Expected Usage

```rust
use std::fs;

fn main() -> Result<()> {
   let tarrow = Tarrow::new();
 
   // Add a file 'README.md', and 'src/script.js'. Delete the directory 'tmp' and 
   // the file 'src/temp.js'
   //
   // Note - Tarrow::add and Tarrow::remove can be used in place of `!` and `-` respectively
   let tarrow_tree = tarrow.store_tree(TarrowRef::from("main"), TarrowTree::new([
      !("README.md", Tarrow::blob_from_string("Initial content of README.md")),
      !("src", TarrowTree::new([
         !("script.js", fs::read_to_string("./script.js")),
         -("temp.js")
      ])),
      -("tmp"),
   ]));
   
   // Update 'README.md`
   let tarrow_tree_revised = tarrow.store_tree(TarrowRef::from("main"), TarrowTree::new([
      !("README.md", Tarrow::blob_from_string("Updated content of README.md"))
   ]));
   
   println!("Old revision - {:?}", &tarrow_tree.content_address_sha256);
   println!("New revision - {:?}", &tarrow_tree_revised.content_address_sha256);
   
   // Any of the following will output 'Updated content of README.md'
   //    Get the latest 'README.md'
   String::from(
      tarrow.get_node_from_ref(TarrowRef::from("main"), "README.md")
         .as_blob()
         .unwrap()
   );
   
   //    Get the value of 'README.md' by directly using the tree's content address
   String::from(
      tarrow.get_node_from_content_address(&tarrow_tree_revised.content_address_sha256, "README.md")
         .as_blob()
         .unwrap()
   );
   
   // Any of the following will output 'Initial content of README.md'
   //    Get the previous value of 'README.md' by referencing a delta for a ref
   //    Both methods should be supported:
   //         TarrowRef::from("main@1")
   //         TarrowRef::from("main").before(1)
   String::from(
      tarrow.get_node_from_ref(TarrowRef::from("main").before(1), "README.md")
              .as_blob()
              .unwrap()
   );
   
   //    Get the value of 'README.md' by directly using the old tree's content address
   String::from(
      tarrow.get_node_from_content_address(&tarrow_tree.content_address_sha256, "README.md")
              .as_blob()
              .unwrap()
   );

   Ok()
}
```

## Design Goals & Features

1. One of the major ideas of tarrow is to consider the DB schema as part of the public API 
   itself. What this would mean is that after a specific final version, all future versions of 
   tarrow will aim for backward compatibility of the table schema, constraints, naming, etc. Any 
   direct queries to the database skipping any library methods will be entirely supported.
2. Tarrow largely uses the Git ObjectDb and Tree model for storing versions of data blobs, 
   with some deviations:
   1. Tarrow does not store tree entries in the ObjectDb - Tree and Tree Entries are stored in 
      their dedicated tables
   2. Tarrow does not use content addresses for referencing objects - it instead uses internal 
      identifiers. The public API continues to use content addresses for referencing blobs and trees
   3. Tarrow does not use commit objects. Ref revisions are maintained and any commit data can 
      technically be stored in the metadata
   4. Following from (3), Tarrow doesn't support branches as well, at least not in the sense 
      that git users might expect - refs are supported and can be treated as branch heads, but 
      the lineage of a branch is not followed via a commit lineage. This decision although taken 
      for the sake of simplicity, is not be finalized - as not having commit objects might 
      create issues in terms of extending the functionality of Tarrow and this view is 
      largely subject to revision.
   5. Tarrow simply supports refs, and refs directly point to trees. A reflog is supported
3. Tarrow intends to support multiple SQL databases
   1. Tarrow internally uses Diesel for all storage, so it certainly should be possible to not 
      restrict the storage backend to be Postgres. However, the first mature version that will 
      include db schema as part of the public API will only consider Postgres
   2. There is no expectation of the SQL schema being part of the public API to be dialect-agnostic
4. Content addresses are always typed as `[u8; 32]` (and internally as `Vec<u8>` before 
   validations checks are performed) - the responsibility of converting it to a display-friendly 
   format (usually a hex-encoded string) lies with the consuming client.
