use macros::build_parser;

build_parser! {
    #S: B -> "a", B;
    #S: A -> "a";
    B: C -> "c", B;
    B: D -> "d";
}


#[test]
fn test_table() {


}