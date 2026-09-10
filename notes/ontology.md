# os

```
                             extends
                   Interface <------ Interface
                    ^  ^  ^
          implements|  |  |constraints
                    |  |  +----------> Action Type
                    |  +-------------> Interface Link
                    |
Ontology -> Object Type <-----> Link Type <-----> Object Type
                |
                +----> Property ----> Data Type
                |          |
                |          +--------> Ontology Value Type
                |          |
                |          +--------> Specialized property service
                |
                +----> Object
                |          |
                |          +--------> Linked Object traversal
                |
                +----> Object Set <----> Interface
                           |
                           +-----------> Link traversal
                           +-----------> Other Object Sets
                           +-----------> Objects/Aggregations
```
