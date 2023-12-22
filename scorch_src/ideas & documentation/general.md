
### types 
- types are basic, `Float`, `String`, `Bool`, `None`, `Fn`, are basically the extent of it right now

- both explicitly & implicitly typed variables retain their mutability & type for the duration of their lifetime, it cannot be reassigned.
- the `Dynamic` type allows it's value to be excluded from the type system, and reassignment of type is possible.



### declaration 


#### implicit

the `:=` is an implicit declaration operator.
it can be used to declare functions, fields, variables, and default parameters.

see functions for more on the specifics of using this in that context, but other than that
generally you can just provide a value as such.

```
i := 0
x := i * 100
```


#### explicit
`identifier : TypeName = value` declares an explicitly typed field with the provided value.
`identifier : TypeName` declares an implicitly typed field with the default value for that type. ie 0, "", false


#### assignment 
the `=` can only assign fields that have already been declared, and that arent 'const'.
currently, `:=` is required for functions even during reassignment.

#### arrays

implicitly typed array declaration with default values.
note the [...] is just a literal, and can be used in expressions, although it often creates a new array on evaluation.
```
people := ['mary', 'jim', 'billy']
println(people == ['mary', 'jim', 'billy'])
```

##### explicit definition

```
arr : Array = [0, 1, 2, 3, 'right now, arrays don' have type restrictions to their elements.']
```

##### accessing array elements 
all access is bounds checked and terminates the runtime if violated.
at least while we have no error system.
```
my_variable := array[0]
```
##### setting array elements
```
array[0] = 100
array[1] = 205
```


### const / var 
everything is const by default, adding var to a declaration will make a reference mutable.

for example: 
```
// declares a const Fn() -> None
main := {
	//...
}
// error : redefinition of a immutable.
main := { 
	return 'hi'
}
```

instead, 
```
var main := {
	//...
}
main := {
	return 'hi'
}
println(main())
```

### conditionals
else does not currently require an if following it to do a comparison.
this is not a planned feature.

```
if true {
   ... 
} else false {
   ...
}
```

## functions
#### declaration : 
---
parameterless, implicit return type.
`func_name := {}`

parameterized, implicit return type.
`func_name := (a: String, b: String) {}`  

parameterized, explicit return type.
`func_name : Fn(a: String, b: Float) -> bool {}`


##### function calls
--- 
simple c style function calls
right now, parameters may not wrap. newlines are fragile still, the parser is not fleshed out.
```
function_name(parameter, array[0], other_function())
result := get_value()
_ = discard_value_keep_side_effect()
```

##### function pointer
- you can obtain a reference to a function simply by using the identifier.
they behave just like functions themselves, although theres no way to check type / parameter types right now.
it will just fail at runtime if provided the wrong types/ parameter count.