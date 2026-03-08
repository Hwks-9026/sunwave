--- Help --
The sunwave language is used by binding values to identifiers.
use the walrus operator ':=' to do this.
ex.
    x := 4;

Any line missing an ending semicolon will print out information 
as it executes, similarly to matlab.
ex.
    >> x := 4
    x assigned to [x: 4]

Define a lambda (an anonymous function) with standard closure syntax.
You can also bind it to an identifier.
ex. 
    |x| x^2;
    f := |x| x^2;

Sunwave supports conditionals via the turnary operator:
ex.
    x := 1;
    y := x > 0 ? 1 : 2;
    y
    [y: 1]

Sunwave has 5 keywords. three of them are related to scope.

module: define a module with a name and a scope.
export: export a value from a module to make it accessible from outside.
access exported values using a '.'
ex. 
    module constants { 
        PI := 3.1415926;
        export PI;
    }
    constants.PI
    [PI: 3.1415926]
    

import: load a file path (directory or file [.sw not neccesary]) as a module,
bound to the identifier of the name of the file.
ex.
    import \"std/math\"
    calc := std.math.calculus;
    module functions {
        poly := |x| x < 4 ? {
            y := x * x;
            y
        } : {
            y := x * x * x;
            y
        }
        export poly;
    }
    result := calc.int(functions.poly, 0, 2)

The final two keywords are 'loop' and 'recur'.
The best way to explain these keywords is through an example:
ex.
    loop(i := 1) { 
        i > 3 ? i : recur(i + 1) 
    }

in this example, loop initializes the identifier i to a vlaue of 1.
it also marks the place for recur to jump back to. Inside the block,
the value of i is checked, and if greater than 3 is returned.

This combination of keywords allows for simplifying repeated work without
using recursion, which would otherwise quickly fill up the call stack and 
crash the interpreter.

You can run sunwave with a file path as the first command line argument,
and it will execute that file instead of providing a repl.
Note: multi-line statements and defining modules are only supported in files.

Happy Sunwaving!
