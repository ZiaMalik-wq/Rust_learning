fn main() {

    // Variables are immutable by default , we can make the immutabe by adding 'mut' keyword.

    let mut x = 5;
    
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");

    // Constant Declaration
    /*
     We can't add mut keyword to the constant.
     They are always immutable.
     The type of value must be annotated.
     Constants  may be set only to a constant expression, not the result of a value that could only be computed at runtime
     */
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    /*
     we can use the let keyword to overshadow the previous variable of the same name.
     */

    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /*
     The diff between mut and shadowing is that we're effictvely
     creating a new variable when we use let keyword, so we can 
     change the type of the value but reuse the same name.
     */

     /* 
      Shadowing thus spares us from having to come up with    
      different names, such as spaces_str and spaces_num;  instead, we can reuse the simpler spaces 
     */
     let spaces = "  ";
     let spaces = spaces.len();
     println!("{}", spaces)
}
