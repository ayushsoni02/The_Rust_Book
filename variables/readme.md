Constants :

Like immutable variables, constants are values that are bound to a name and are not allowed to change.
First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
Rust’s naming convention for constants is to use all uppercase with underscores between words.

Shadowing :

We can shadow a variable by using the same variable’s name and repeating the use of the let keyword.

we’re not allowed to mutate a variable’s type, but we can change the value it refers to.

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.