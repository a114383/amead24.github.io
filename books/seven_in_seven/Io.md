Io - Day 1
04/07/2019

As a side note, the Io REPL was kind of a nightmare that I wish I could find some better documentation on.  No cls or ctrl+l, left/right keys, or up/down for history.

**1. Run an Io program from a file.**

I kinda wish it would have just told us this because it was super helpful and googling for Io answers is fairly non-existant
```
doFile("file.io")
```

**2. Execute the code in a slot given its name.**

This seems pretty strait forward, and there's plenty of examples in the book so I feel like i'm missing what the question is asking.
```
foo := Object clone
foo print := method("test function" println)

foo print
==> test function
```

Nothing in this first chapter seemed radically insightful, however the more I played around with toy examples I started to realize the bigger picture.  Originally I had thought about Objects as just another name for a class, but after playing around more I started to realize it was similiar enough to my usual languages that I could write decent code, but if I wanted to really make the best of the language I needed to fundamentally rethink how I was using it.


Io - Day 2
04/14/2019

**1. A Fibonacci sequence starts with two 1s. Each subsequent number is the sum of the two numbers that came before: 1, 1, 2, 3, 5, 8, 13, 21, and so on. Write a program to find the nth Fibonacci number. fib(1) is 1, and fib(4) is 3. As a bonus, solve the problem with recursion and with loops.**

The recursive problem was easier than the loop problem because the terminal kept returning nil.  After reading backwards a bit, I realized I was using the wrong assignment operator.  This suprised me that it could still execute rather than throw an error message.
```
rfib := method(n, if(n<=2, 1, fib(n-1) + fib(n-2)))
lfib := method(n, f := 0, n1 := 1, n2 := 0, for(i, n, 1, -1, n2 = f + n1; f = n1; n1 = n2) f)
```

**2.  How would you change / to return 0 if the denominator is zero?**

This one took some serious trial and error.  My first attempt didn't work out:
```
/ := method(n, d, if(d==0, 0, n/d))
```
But this got me thinking in Python you don't overload operators by editing both the left and the right, instead the left is calling the operator on the right, ex: `def __operator__(self, right)`.  Which we see if we test out:
```
Io> 5 /

  Exception: argument 0 to method '/' must be a Number, not a 'nil'
  ---------
  message '/' in 'Command Line' on line 1
```
It's looking for that single right parameter!

Again reading backwards I found that to save a slot you need `getSlot("/")`, which then told me I was on the right track for updating the operator:
```
Number oldDiv := Number getSlot("/")
Number / = method(d, if(d==0, 0, oldDiv(d)))
```

**3. Write a program to add up all of the numbers in a two-dimensional array.**

The syntax here really threw me off, and I could only get the right arguments to send to the correct message with semicolons.  
```
# for example
List sum := method(
    total := 0
    self foreach(lst, total = total + lst sum)
    total
)
# ==> Number doesn't respond to self
```
If someone knows how to avoid this i'd be happy to hear. 

Regardless I did this both ways, making a function, and updating the slot below:
```
sum2D := method(array, total := 0; array foreach(x, total = total + x sum); total)
sum2D(list(1,2,3), list(4,5,6)) # ==> 21
```

One thing that I really like about Io so far is that it's really easy to inspect objects and their methods.  Having learned about `getSlot` in the previous problem I used it to see how the standard library does it:
```
List getSlot("sum")
==> method(self reduce(+))

# This got me poking around ... List proto ...
# And viola! ... List flatten
List sum := method(self flatten reduce(+))
list(list(1,2,3), list(4,5,6)) sum # ==> 21
```

**4. Add a slot called myAverage to a list that computes the average of all the numbers in a list. What happens if there are no numbers in a list? (Bonus: Raise an Io exception if any item in the list is not a number.).**

Adding a slot for `myAverage` is easy now that we tackled adding a slot, and an few built in methods for lists in the last problem:
```
List avg := method(
    if(self size == 0, return nil,
            self flatten foreach(x, if(x type != "Number", Exception raise("Not a number!")));
            self flatten average 
    )
)
```

**5. Write a prototype for a two-dimensional list. The dim(x, y) method should allocate a list of y lists that are x elements long. set(x, y, value) should set a value, and get(x, y)  return that value.**
```
Array := Object clone

Array dim := method(x, y,
    self data := list setSize(y);
    for(i, 0, y - 1, self data atPut(i, list setSize(x)))
)

Array set := method(x, y, value,
    self data at(y) atPut(x, value)
)

Array get := method(x, y,
    self data at(y) at(x)
)

a := Array clone
a dim(2, 2)    # ==> list(list(nil, nil), list(nil, nil))
a set(0, 0, 1) # ==> list(list(1, nil), list(nil, nil))
a get(0, 0)    # ==> 1
```

**6. Bonus: Write a transpose method so that (new_matrix get(y, x)) ==
matrix get(x, y) on the original list.**

Originally I thought that there should be a way to do this with the caller/sender functionality, but I couldn't figure it out until I read the next chapter.

```

```

Io - Day 3
04/20/2019

**1. Enhance the XML program to add spaces to show the indentation structure.**
```
Builder := Object clone

Builder indent := 0

Builder forward := method(
    writeln(" " repeated(indent), "<", call message name, ">")
    self indent = self indent + 2
    call message arguments foreach(arg,
        content := self doMessage(arg);
        if (content type == "Sequence", writeln(content))
    )
    self indent = self indent - 2;
    writeln(" " repeated(indent), "</", call message name, ">")
) 
```

**2. Create a list syntax that uses brackets.**
```
```

**3. Enhance the XML program to handle attributes: if the first argument is a map (use the curly brackets syntax), add attributes to the XML program.**
```
Builder forward := method(
    write(" " repeated(indent), "<", call message name, " ")
    call message arguments foreach(arg,
        content := self doMessage(arg);
        if (content type == "Map", 
            content foreach(key, value, write(key, "=\"", value, "\""))
        )
    )         
    writeln(">") 
    . . .
)

Builder body(title({"font-weight": "bold"}, "heres a title"), div("and content")) 

==> doFile("builder.io")
<body <title font-weight="bold">
heres a title
</title>
<div >
and content
</div>
>
 <title font-weight="bold">
heres a title
 </title>
 <div >
and content
 </div>
</body>
```

This one took some tinkering, one thing that I wasn't understanding was that the map wasn't getting evaluated at first.  If printed out the messages before checking if they were a `Map` I saw this:
`<body list(title(curlyBrackets(setValue(""font-weight"", "bold")), "heres a title"), div("and content"))>`

Ultimately I still wasn't able to get it, it's nearing the end of the week and all I can think about is what language is next.  I'll try to circle back at a later time.

Lastly, I just want to say this was super fun.  I think the authors made a good choice of picking Io over Javascript, it's strange enough that turning to SO or Google was less preferable than reading through the documentation and the chapter a few times which turned up some additional functionality that helped in later problems.  Overall a bigger picture emerged out of all this.  Compared to the Ruby chapter where I was able to crystalize the benefits and use cases for OOP that I was already roughly familiar, once I started seeing "everything as messages" I felt confident that I learned a new paradigm.  And with most amorphous topics, knowing what something isn't really helps to know what it is.