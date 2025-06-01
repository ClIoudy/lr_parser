use macros::build_parser;

#[test]
fn test_table() {

    build_parser! {
        #S: B -> "a", B;
        #S: A -> "a";
        B: C -> "c", B;
        B: D -> "d";
    }

}