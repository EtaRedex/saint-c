use saint_c;

fn main() {
    let char_ptr = saint_c::pointer_box(saint_c::s_char);
    let char_ptr_ptr_ty = saint_c::pointer_box(char_ptr);
    let arg1 = saint_c::argument(saint_c::int, "argc");
    let arg2 = saint_c::argument(char_ptr_ptr_ty, "argv");
    let args = vec![arg1, arg2];
    let l_block = saint_c::block::new()
        .addStatment(
            saint_c::function_call(
                "puts",
                vec![saint_c::string_literal("hello world !".into())],
            )
    );

    let struct_def = saint_c::cstruct("Pair")
        .add_field(saint_c::int, "a")
        .add_field(saint_c::int, "b");

    let main_function = saint_c::function(saint_c::void, "main", args, l_block);

    println!("{}\n", main_function.emit());
}
