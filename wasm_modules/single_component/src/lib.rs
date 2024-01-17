wit_bindgen::generate!({
    path: "wit/component.wit",
    exports: {
        "test:guest/add": Add
    }
});

struct Add;

impl exports::test::guest::add::Guest for Add {
    fn addtwonumbers(x: u32, i: u32) -> u32 {
        let var_name = x+i;
        var_name
    }
}