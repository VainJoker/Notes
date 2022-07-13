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
Deref可以为结构体实现解引用
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
Deref 是个特征,可以在适当的时候自动调用对实现了这个特征的类型进行解引用,
      缺点是使代码可读性降低

      总结一下:一个类型为T的对象foo,如果T:Deref<Target=U>,那么foo的引用&foo在调用时会自动转为&U.

## Drop
      Drop特征也是智能指针的必备特征之一

      Drop顺序
      变量按逆序,结构体内部按顺序

      Rust会自动为每个变量创建Drop特征但是
      Rust不允许显式的调用析构函数,Drop特征的drop方法不会夺走其所有权,导致不安全代码.
      只有极少数的情况需要drop回收

## Rc与Arc
      Rust的所有权机制要求一个值只有一个所有者,但是在有些情况下,需要多个所有者的存在
      Rust通过引用计数的方式允许一个数据资源在统一时刻拥有多个所有者.

      当我们希望在堆上分配一个对象供程序的多个部分使用且无法确定那个部分最后一个结束时,就可以使用Rc成为数据只的所有者

      ```Rust
      use std::rc::Rc;
      fn main(){
        let a = Rc::new(String::from("hello"));
        let b = Rc::clone(&a);
        assert_eq!(2,Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a),Rc::strong_count(&b));
      }
```

Rc/Arc是不可变引用,无法修改他指向的值,只能读取,一旦最后一个拥有者消失资源就会自动被回收


## Cell和RefCell

Cell和RefCell 可以在不可变引用的同时修改目标数据,
  Cell和REfCell功能上没有区别,Cell<T>适用于T实现了Copy的情况

  ```Rust
  use std::cell:Cell;
  fn main(){
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println("{},{}"one,two);
  }
```

我们需要解决的是可变不可变引用共存导致的问题,此时就需要借助RefCell
但是RefCell并不能解决可变不可变引用共存的问题,只是将编译器的错误,推迟到了运行时,
  RefCell是用于你确信代码是正确的但编译器发生了误判

  ```Rust
  use std::cell:Cell;
#[cerive(Debug)]
  struct Person{
name:String,
       age:Cell<u8>,
  }
```

