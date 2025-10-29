## essential
- [x] "inline" lexer 
- [x] rustc overflows its stack when chaining start symbols
- [x] alphabet (for use in lexer)

- [ ] grammar parsing error handling (e.g. not a valid graph/mentioned but missing definition of non-terminal)
  - partly already implemented via missing-enum error messages

## cool features
- [ ] allow for named fields on variants (all fields need names then)
  - maybe in struct like syntax
  -> struct-like enum variant
- [ ] regex-like features like * signaling Vec fields in enum