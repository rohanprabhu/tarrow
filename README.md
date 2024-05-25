# Tarrow

A simple library for versioned storage of data blobs (designed with small files in mind), backed by Postgres.

This is mainly an educational project - the main goal was to author this library as a way to 
learn rust, given which, till this particular notice stays, I wouldn't really recommend using 
this in any project except for learning.

## Expected Usage


```rust
// Make a nice little example here
let tarrow = Tarrow::new();
tarrow.store_tree(TarrowTree::new([
    ("README.md", Blob("File contents of README.md".to_buffer())),
    ("src", TarrowTree::new([
        ("script.js", Blob(""))
    ]))
]))
```

## Design Goals & Features

1. One of the major ideas of tarrow is to consider the DB schema as part of the public API 
   itself. What this would mean is that after a specific final version, all future versions of 
   tarrow will aim for backward compatibility of the table schema, constraints, naming, etc. Any 
   direct queries to the database skipping any library methods will be entirely supported.
2. Tarrow largely uses the Git ObjectDb and TreeDb model for storing versions of data blobs, 
   with some deviations:
   1. Tarrow does not store tree entries in the ObjectDb - Tree and Tree Entries are stored in 
      their dedicated tables
   2. Tarrow does not use content addresses for referencing objects - it instead uses internal 
      identifiers. The public API continues to use content addresses for referencing blobs and trees
   3. Tarrow does not use commit objects. Ref revisions are maintained and any commit data can 
      technically be stored in the metadata
   4. Tarrow simply supports refs, and refs directly point to trees. A reflog is supported
3. Tarrow intends to support multiple SQL databases
   1. Tarrow internally uses Diesel for all storage, so it certainly should be possible to not 
      restrict the storage backend to be Postgres. However, the first mature version that will 
      include db schema as part of the public API will only consider Postgres
   2. There is no expectation of the SQL schema being part of the public API to be dialect-agnostic

**Author** Rohan Prabhu <rohan@rohanprabhu.com>