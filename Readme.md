# SNL Compiler

SNL Compiler frontend implemented in rust and c(with ffi).

## Structure

- [x] snl-lexer: Lexer written in Rust
    - [x] Tokenize
    - [x] Comment supoort
- [x] snl-rdp: Recursive descent parser written in Rust
    - [x] Support all SNL syntax rules
- [ ] snl-ll1: LL(1) parser written in Rust.
- [ ] snl-semantic: Semantic Analysis part written in Rust
    - [ ] Construct symbol table
        - [x] Type
        - [x] Variable
        - [ ] Procedure
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
        - [ ] Call parameter type mismatch
        - [ ] Call parameter count mismatch
        - [ ] Invalid bool expression
        - [x] Invalid read type
        - [x] Invalid write type
- [x] snl-utils: Some common parts
- [ ] snlc: Simple representation program, uses all the librarys above

## Credits

- http://jsmachines.sourceforge.net/machines/ll1.html
- https://vanya.jp.net/vtree/
