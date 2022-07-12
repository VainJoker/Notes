# 智能指针
指针是一个包含内存地址的变量,该内存地址引用或者指向了另外的数据.
Rust中最常见的指针类型是引用,通过&表示.不过他还有一个更深的含义,借用其他变量的值.
智能指针虽然也是指针,但他是一个复杂的家伙,通过比引用更复杂的数据结构包含比引用更多的信息.
引用和智能指针的另一个不同在于前者仅借用了数据,而后者可以拥有他们指向的数据,然后再为其他人服务.

## Box
Box<T> 是Rust中最常见的智能指针.他允许你将一个值分配到栈上,然后在栈上保留一个智能指针指向堆上的数据.
特意将数据分配在堆上,
  数据较大,不想在转移所有权时进行数据拷贝
  类型的大小在编译器无法确定,但是我们又需要固定大小的类型
  特征对象,用于说明对象实现了一个特征而不是某个特定的类型

  ```Rust
  fn main(){
    let a = Box::new(3);
    let arr = [0;1000];
    let arr1 = arr;
    println("{:?}",arr.len())
      println("{:?}",arr1.len())
      let arr = Box::new([0;1000]);
    let arr1 = arr;
    println("{:?}",arr1.len())
  }
enum List{
  Cons(i32,Box<List>),
  Nil,
}
```

Box::leak 他可以消费掉Box并且强制目标从内存中泄漏,例如可以吧一个String类型变成一个'static的&str类型

## Deref
常规解引用*
智能指针Deref可以为结构体实现解引用
```Rust
use std::ops:Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
  fn new(x:T) -> MyBox<T>{
    MyBox(x)
  }
}
impl<T> Deref for MyBox<T>{
  type Target = T;
  fn deref(&self) -> &Self::Target{
    &self.0
  }
}
```
