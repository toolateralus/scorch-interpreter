#prints total lines of code in project for fun :D
find . -name "*.rs" -exec cat {} \; | wc -l