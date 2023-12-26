; ModuleID = 'program'
source_filename = "program"

define void @main() {
entry:
}

define i32 @get_funcy() {
entry:
  ret i32 250
  ret i32 0
  %calltmp = call i32 @get_funcy(i1 false)
  ret void
}
