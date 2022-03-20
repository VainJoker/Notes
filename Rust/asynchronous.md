# Rust 异步

## async/await 入门
aysnc 将代码转化为哟个实现了Future trait的状态机
block_on阻塞当前线程，直到提供的Future执行
与block_on不同.await不会阻塞当前线程直到Future执行

## Future trait 
是异步计算，可以产生一个值，表示目前不可用的值
```Rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake:fn()) -> Poll<Self::Output>;
}

enum Poll<T>{
    Ready(T),
    Pending
}
```
Future 代表着一种你可以检验其是否完成的操作，
Future 可以通过poll函数来去的进展，针对Future你唯一可做的事就是用poll来敲他，直到一个值掉出来
当Future准备取得更多进展的时候调用poll,通过wake()函数，执行其可以确切的直到哪些函数准备好被poll

## Waker
Future在第一次poll的时候通常无法完成任务，每次Future被poll,都是作为任务的一部分，任务就是被提交给执行者顶层的Future
Waker提供了wake()方法，可以用来被告诉执行者，相关的任务应该被唤醒，
当wake()被调用，执行者直到Waker所关联的任务已经准备好去的更多进展，Future应该被再次poll
Waker实现了clone(),可以复制和存储

## executor
