# common scenario
## 1. A is a struct. And A is a member of B.
But sometimes, instance of A could be mut. The relative instance B could alse be changed.
A should be setted as a `Rc<RefCell<A>>`.

## 2. about struct and ownership
struct T can borrow part ownership to a new variable.
Rust区分了借用(borrow)和拥有(own)两个概念：
 - 如果你是这个值的owner，那么你可以对它做任何事，并且负责最后释放它；
 - 如果你只是共享借用（引用），那么你只能读，不能改变它；
 - 如果你拥有的是可变引用，那么可以修改这个值，但你不能move或者destroy这个值(统称转移ownership)。
