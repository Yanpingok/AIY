module a::b;

fun f() {
    let x = aiy::dynamic_field::borrow<vector<u8>, u64>(&parent, b"");
    let x = ::aiy::dynamic_field::borrow<vector<u8>, u64>(&parent, b"");
}
