; ModuleID = 'program'
source_filename = "program"

define void @main() {
entry:
  %calltmp = call i32 @get_funcy(i1 false)
  ret void
}

declare i32 @get_funcy()
