

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


### struct declaration

since implicit functions occupy the syntax

```
identifier := { body.. }
```

and since struct bodies are non executable, meaning
they don't get called like a function for side effect or result,
and they just represent the structure of the object,
we could use a clearer and moer distinctive syntax for struct declarations.

```
vector : struct = |
    x : Float = 0
    y : Float = 1
|

vector := |
    x := 1.0
    y := 2.0
|
```
### array syntax

```cpp
    
col := [] // implicit type

col := [](250) // array length 200

col := Float[] // specify type

col := Float[](200) // hash table

col := Float[struct] // hash table

col := Float[struct](200) // hash with init len

col := [0, 1, 2, 3, 4] // initializers

col ~= element // push or add an element to the end.

value := col~~   // pop or remove and return an element from the end

// for each item in collection, call print with the value(s) as arguments.
col => print

// foreach item in collection, call this anonymous function.
col => (i : Float) {
    //.. do something
}

// basically a  'for (Float i = 0; i < 200; ++i)' with this syntax
[200] => (i : Float) {
    
}





```