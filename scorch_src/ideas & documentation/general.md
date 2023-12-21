- all numbers are currently f64/doubles. type name 'float'

- newlines are expression terminators, like semi colon in C.

- := is implicit assignment
- varname : type = value is explicit assignment

- if else use parentheses-less comparisons.
```

// else does not currently require an if to do a comparison.
// this is not a planned feature.
if true {
   ... 
} else false {
   ...
}
```

- func_name := {} is a paramless fn
- func_name := (a: String, b: String) {}  is a parameterized fn

accum := ''

print_lines := (str : string, x : float, y : float, z : float) {
	loop := {
		repeat i < x {
accum = accum + '
[this is a line]' + '
' + str
		}
		repeat i < y {
			println(accum)
		}
	}
	
	repeat v < z {
		loop()
	}
}

print_lines("hello world", 100, 100, 50)