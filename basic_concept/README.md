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

## 3. organize the code
pub(crate) mod modname
pub use modname.
change the order

## 4.about Cell UnsafeCell RefCell
- Cell: use replace fn to replace the old value.And is only for Copy trait
- UnsafeCell provice an unsafe fn so that can transfer a shared ref to a mut ref.
- RefCell provide the clone trait for interior mutability.

## 5.trait safety

## 6.Pin

# common difference
## data race
简单来讲就是因为读写锁允许在仅读时可以"不持有锁", 而读操作会获取到受保护数据的不可变引用&T . 对于RefCell(使用了UnsafeCell)这种类型不满足Sync的值, 允许通过&self这种方法来修改值, 因此读写锁要求保护的数据的类型自身必须满足Sync. 而Mutex的读/写共享内存操作都需要线程已经"持有锁", 因此就算要保护的数据的类型不是Sync的(仅通过不可变引用就能修改值), 也因为"无论读写都要先持有锁"这个规则, 使得不会发生data race.

## copy and clone

## send and sync

## cell and refcell

## rc and arc

## move and share
因为 &mut T 的 Send 意味着 move，而 &T 的 Send 意味着 share。要想多线程共享 &T， T 就必须 Sync。