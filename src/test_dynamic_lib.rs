fn _function_to_call() -> int {
    return 16;
}

#[no_mangle]
pub static function_to_call: fn() -> int = _function_to_call;
