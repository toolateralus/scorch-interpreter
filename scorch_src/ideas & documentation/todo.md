

# todo
- implement conditional / relational expressions;

# our <= and => operators have special purpose, not yet implemented. packing & extracting
- implement operators (|, ||, &, &&, ==, !=, <, >, <=, >=)
- implement extended assignment operators (+=, -=, *=, /=, ^= , &=, %=)

#ideas ranked from nearest to being completed to furthest.
- access modifiers (data mutability) (const, mutable) {
    - const will be default, but can be specified for clarity.
    - mutable can be used to make a variable mutable... woah
}

- access modifiers (data accessibility) (public, private) {
    - if we have structs or objects we definitely want these.
}

- objects, operator overrides, arrays, function ([], ()) {
    - do we want arrays and methods built into the language in rust, or should
    we make a base object type that everything inherits from that we can implement operators in 
    and just make it 'in language'?
}


### repeat keyword

### this is mostly implemented, just not the return value. returns are nyi in general.

- conditionless while loops
- while loops that can return value on break, avoid annoying declarations.
```
result := repeat i < 2500000 {
    x = 1
    z = x + y * z
    v = z * z / z
    v = v + i
    x = 1
    z = x + y * z
    v = z * z / z
    v = v + i
    x = 1
    z = x + y * z
    v = z * z / z
    v = v + i
    
    if i > 25000 && v != -1 {
        break x
    }
}

// conditional void returning while loop
repeat i < 2500000 {
    println('hi', i)
}

repeat {
    println('hi')
}
```