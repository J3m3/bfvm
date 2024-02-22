[
  NOTE: provide null character end of the input
  ex. echo "4213\0" | cargo run -q -- ./example/bsort.bf
]

[bsort.b -- bubble sort
(c) 2016 Daniel B. Cristofani
http://brainfuck.org/]

>>,[>>,]<<[
[<<]>>>>[
<<[>+<<+>-]
>>[>+<<<<[->]>[<]>>-]
<<<[[-]>>[>+<-]>>[<<<+>>>-]]
>>[[<+>-]>>]<
]<<[>>+<<-]<<
]>>>>[.>>]

[This program sorts the bytes of its input by bubble sort.]