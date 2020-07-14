# 信号
1. 概念: 时变的物理量 等同于时间函数  $f(t)$ 连续,$f(k)$离散
2. 分类: 
   1. 连续信号:定义域连续 / 离散信号:定义域离散
   2. 确知信号:有函数式 / 随机信号:无函数式
   3. 周期信号 / 非周期信号
   4. 实信号: 取值为实数 / 复信号: 取值为负数
   5. 功率信号:能量无限信号 / 能量信号:能量有限信号

## 信号的基本运算

1. 信号的加/乘 : 信号对应时刻的加或者乘
2. 反转 : 
    1. $f(-k)$
    2. $f(-t)$
3. 平移: 
    1. 负向移动:$f(t+t0)$ 
    2. 正向移动:$f(t-t0)$ 
4. 尺度变换 $f(at)$
    1. a>1,压缩
    2. 0<a<1,展宽
5. 微积分 / 积分


## 基本信号

#### 阶跃函数 
$\epsilon(t)=
    \begin{cases}
    0,& t<0\\
    1,& t>0
    \end{cases}
$
---
1. 用来描述信号的开始时刻
2. 用来描述基本信号

#### 冲激函数

$\delta(t)=
\begin{cases}
0,& t\neq0\\
\infty,& t=0
\end{cases}$
--
作用时间极短，作用力极大的物理量

满足对普通函数f(t)存在取样作用的一类函数均称为冲激函数

所有窄脉宽中脉宽->0的极限

面积始终为1:  
$\int_{-\infty}^{+\infty}$$\delta(t)d(t) = 1$


1. 性质:

    $f(t)\delta(t) = f(0)\delta(t)$

    $f(t)\delta(t-t0) = f(0)\delta(t-t0)$

    $f(t)\delta(t+t0) = f(0)\delta(t+t0)$


2. 作用:
    描述间断点的导数

3. 冲激函数的广义函数定义:
    对检验函数的映射

    $\int_{-\infty}^{+\infty}$$f(t)\delta(t)dt$
    = $\int_{-\infty}^{+\infty}$$f(0)\delta(t)dt
    = f(0)$

    $\int_{-\infty}^{+\infty}$$f(t)\delta(t-t0)d(t) = f(t0)$

4. 冲激函数的导数

    冲激偶函数:
    $\delta'(t)$

    $f(t)\delta'(t)=f(0)\delta'(t)-f'(t)\delta(t)$

    $\int_{-\infty}^{+\infty}$$f(t)\delta'(t) = -f'(0)$

    $\int_{-\infty}^{+\infty}$$f(t)\delta'(t-t0) = -f'(t0)$


5. 积分\导数

    冲激函数是阶跃函数的导数:  
    $\delta(t)=\frac{\mathrm{d} \epsilon(t)}{\mathrm{d} t}$

    阶跃函数的冲激函数的积分:  
    $\epsilon(t)=\int_{-\infty}^{t}\delta(\tau)d\tau$

    $\int_{-\infty}^{t}\delta(t)dt=
    \begin{cases}
        0,t<0\\
        1,t>1
    \end{cases}$
    
6. 尺度变换
    $\delta(at)= \frac{1}{|a|}\delta(t)$
