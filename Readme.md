# SNL Compiler

SNL Compiler frontend implemented in rust and c(with ffi).

## Structure

- [x] snl-lexer: Lexer written in Rust
    - [x] Tokenize
    - [x] Line & Column
    - [x] Comment
- [x] snl-rdp: Recursive descent parser written in Rust
- [x] snl-ll1: LL(1) parser written in Rust.
- [x] snl-semantic: Semantic Analysis part written in Rust
    - [x] Construct symbol table
        - [x] Type
        - [x] Variable
        - [x] Procedure
    - [ ] Semantic errors
        - [x] Duplicated identifier
        - [x] Undefined identifier
        - [x] Undefined type
        - [x] Uncompatable type
        - [x] Invalid variable represent
        - [x] Invalid array definition
        - [ ] Array index outbound
        - [x] Unexpected array index
        - [x] Invalid field index type
        - [x] Undefined record field
        - [x] Assign type mismatch
        - [x] Invalid assignee
        - [x] Call parameter type mismatch
        - [x] Call parameter count mismatch
        - [x] Invalid bool expression
        - [x] Invalid read type
        - [x] Invalid write type
- [x] snl-utils: Some common parts
- [ ] snlc: Simple representation program, uses all the librarys above

## Credits

- http://jsmachines.sourceforge.net/machines/ll1.html
- https://vanya.jp.net/vtree/
