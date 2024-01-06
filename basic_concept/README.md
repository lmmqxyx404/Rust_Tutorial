# common scenario
## 1. A is a struct. And A is a member of B.
But sometimes, instance of A could be mut. The relative instance B could alse be changed.
A should be setted as a `Rc<RefCell<A>>`.

## 2. about struct and ownership
struct T can borrow part ownership to a new variable.