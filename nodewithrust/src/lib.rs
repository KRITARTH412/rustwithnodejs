#[no_mangle] //no_mangle means let to know the compiler that the function name should not be change 
//extern means that the function is implemented or call from any other language also in another language
pub extern fn add(a:i32,b:i32)->i32{
    a+b
}

//we can now go toml file so that this package expose outside of the world