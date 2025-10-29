## essential
- [ ] grammar parsing error handling (e.g. not a valid graph/mentioned but missing definition of non-terminal)
- [ ] alphabet (for use in lexer)
- [x] default for map repr
- [x] expected as match tokenstream
- [x] actions as match tokenstreams
- [x] build_rule as match tokenstreams
  - [x] match symbol
  - [x] match name
  - [x] return #symbol_ident::#name_ident(#elements)
  - [x] where elements is variant_id.length() amount of pop_downcast(&mut elements)?
  - [x] where pop_downcast\<T>(elements: &mut Vec\<Box\<dyn Any>>) -> T { elements.pop()?.downcast()? }
- [x] maybe use BTreeMap/-Set instead of partial-ord and sort() hacks for states?

## later
- [ ] allow for named fields on variants (all fields need names then)
  - maybe in struct like syntax
  -> struct-like enum variant
- [ ] regex-like features like * signaling Vec fields in enum
  