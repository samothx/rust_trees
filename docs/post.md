# Do I need unsafe code to code an efficient tree in rust ?
In my endeavour to improve my skills regarding the rust ownership rules I ventured into 
a little programming experiment with the ultimate goal to create a simple binary tree, 
and then morph it into a Red-Black-Tree which is basically the same only with some 
auto-balancing mechanisms. 

I had some initial problems when I tried to make my operations non-recursive but those 
could be overcome with the help of this forum. 

Then it got rather tough when I tried to implement the deletion of nodes in the tree. 
I have not yet come up with a non-recursive implementation and the recursive implementation 
only works with a lot of code and with some inefficient workarounds that - while safe 
according to the borrow-checker - are not really safe at all. 

The problem is that for the deletion of a node I need to access mutable references to 
the parent (to remove the sibling) the node itself (extract its siblings) and possibly the 
nodes siblings (if the parent had two siblings before I have to reinsert one of the 
nodes sibling somewhere down the tree). 

As the parent has a reference to the node and the node has references to its siblings it 
seems that it is the agenda of the borrow checker to make it impossible to work with more 
that one mutable reference to this chain of nodes at a time.

So my solution has been to sever the references (they live in Option's so I use
take().unwrap() ) between the parent, the node and its siblings while working my way 
down the tree and restoring them if necessary after finishing my manipulations.

That appears to work but the downside are the inefficient and dangerous take().unwrap() 
operations.   
Inefficient because they are unnecessary other than to pacify the borrow checker and 
dangerous because I sever the links in my tree while working on it and if something 
goes wrong I would have to try and restore all severed references on the way out or 
leave the tree in an invalid state. 

The only ways I can think of working around this would be to use some kind of internal 
mutability which would likely make the code more messy than it already is or to just 
make my delete operation unsafe and then code with pointers as I would do in C/C++. 

I am kind of disappointed that rust makes these operations that are otherwise only mildly 
complex so hard. I have coded all of this in other languages in an hour or so where I 
have been pondering about this for days with rust.

Specially with rust a candidate to be the language to replace C in the linux kernel 
I am asking myself how that is supposed to work - the Linux kernel being full of linked 
lists and tree structures. 

I am asking myself if I am overlooking some obvious pattern or is it really so hard to 
code certain simple things in rust ?

