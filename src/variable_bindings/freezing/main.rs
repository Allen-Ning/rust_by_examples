fn main() {
    let mut _test = 7i32;
    {
        // Shadowing by immutable `_test`
        let _test = _test;

        // Error! `_test` is frozen in this scope
        _test = 50;
        // FIXME ^ Comment out this line

        // `_test` goes out of scope
    }

    // Ok! `_test` is not frozen in this scope
    _test = 3;
}

