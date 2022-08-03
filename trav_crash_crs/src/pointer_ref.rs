//Reference pointers point to a resource in memory

pub fn run() {
    // Primitive Array
    let array1 = [1, 2, 3];
    let array2 = array1;
    println!("{:?}", (array1, array2));
/*
With non-primitives, if you assign another variable to a piece of data, the first 
variable will no longer hold that value. You'll need to use a reference (&) to point 
to the resource    
*/
    //Vector
    let vector1 = vec![1, 2, 3];
    let vector2 = &vector1;
    println!("{:?}", (&vector1, vector2));
}