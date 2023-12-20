

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