

zfs list

$zfs + 2


rot13 "abracadabra"


// list all dir entries
ls all

// list entries in the all directory
ls #all

// or, shortcut to interpreting arg as string
ls 'all

The ' symbol mean: interprete the immediately
following token as a string.


ifconfig re0 inet 10.0.0.4 


Problem: command arg collides in unix namespace.
In unix namespace can be:
  file, but the command knows whether it expects a filename or not.
    not always: ls all (is "all" the dir name or an argument.)
    Problem with variadic functions. But variadic is needed.
  executable in path, but the command knows whether it expects an
  executable or not.
  
Solution:
  "#" symbol is accessor for unix namespace.
  or
  "#" is accessor for executables
  "'" is accessor for filesys
  
  for executable in current dir
  'cargo create freebsd-pkg
  
  '/bin/echo "hello!"


```

/// Constant, immutable.
PI := 3.14
  
/// Private to file/module.
_hidden := 42
  
description := "This file shows of some examples of Cajal code. "
               "Multiline string are done like this"

  
  
/// Naive fibonacci implementation.
fn fibo(max) {
    m := 1
    n := 1
    r := 0
    while r <= max {
        r = n + m
        m = n
        n = r
    }
    r
}

  
  
/// Demonstrating switch statements.
fn greet(s) {
    switch s {
    
        "Hello!" :
            "Hey! How are you?"
        "Get lost!" :
            "I got lost in your mother last night."
    }
}

  
  
class Animal {

}
  
  

class Zebra extends Animal {

}
  
  
class Owl extends Animal {

}
```








