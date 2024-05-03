use nodewithrust::add;
fn main() {
    let result=add(10,20);
    println!("10 + 20 Result is {}",result);
}

// crate- is the thing that the cargo is exporting so its the library type that we have bydefault crate will expose rlib so rust library that allow me to use things kind of internally so i need if i want that main file to continue to compile and work i need to have that rlib in that place so i can call from main but i need to to do create this cdylib so this c style dynamic library and thats the libarary file that nodejs can going to use to interact
// i am calling from the main file so i need rlib if u r exposing module directly and u want to use in nodejs and u havent any rust calling programs and obivously u wouldnt need to use rlib and u could keep it as cdylib