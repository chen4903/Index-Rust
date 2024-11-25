trait Nice {
    fn nice(&self);
    fn change(&mut self);
    fn consume(self);
}

struct Hello;
// 为Hello类型实现trait，则生成一组与Hello有关的方法
impl Nice for Hello {
    fn nice(&self) {}

    fn change(&mut self) {}

    fn consume(self) {}
}

struct Good;
impl Nice for Good {
    fn nice(&self) {}

    fn change(&mut self) {}

    fn consume(self) {}
}
impl Good {
    fn good(&self) {}
}

struct World;
impl Nice for World {
    fn nice(&self) {}

    fn change(&mut self) {}

    fn consume(self) {}
}
impl World {
    fn nice(&self) {}
    fn good(&self) {}
}

struct Bad;
trait Nicer {
    fn nice(&self) {}
}
impl Nice for Bad {
    fn nice(&self) {}

    fn change(&mut self) {}

    fn consume(self) {}
}
impl Nicer for Bad {
    fn nice(&self) {}
}

fn main() {
    let mut h = Hello;
    h.nice();
    h.change();
    h.consume();

    let mut g = Good;
    g.good();
    g.nice();
    g.change();
    <Good as Nice>::nice(&g);
    g.consume();

    /*

        playground::main:
        pushq	%rax
        leaq	5(%rsp), %rdi
        callq	<playground::Hello as playground::Nice>::nice
        leaq	5(%rsp), %rdi
        callq	<playground::Hello as playground::Nice>::change
        callq	<playground::Hello as playground::Nice>::consume
        leaq	6(%rsp), %rdi
        callq	playground::Good::good
        leaq	6(%rsp), %rdi
        callq	<playground::Good as playground::Nice>::nice
        leaq	6(%rsp), %rdi
        callq	<playground::Good as playground::Nice>::change
        callq	<playground::Good as playground::Nice>::consume
        leaq	7(%rsp), %rdi
        callq	playground::World::good
        leaq	7(%rsp), %rdi
        callq	playground::World::nice      <----- 这是调用了impl World中的nice()，而不是impl Nice for World中的nice()。这是因为前者的优先级比后者高(与定义次序无关)
                                                    如果想要调用impl Nice for World中的nice()，则需要这么做：<World as Nice>::nice(&w);。完全限定语法
        leaq	7(%rsp), %rdi
        callq	<playground::World as playground::Nice>::change
        callq	<playground::World as playground::Nice>::consume
        popq	%rax
        retq

    优先级：impl <struct>   >  impl <trait> for <struct>

    let mut w = World;

    w.good();

    w.nice();
    w.change();
    w.consume();

    */

    /*

    如果某个struct为2个及以上的trait实现相同的方法，则报错比如：
        multiple applicable items in scope
        multiple `nice` foundrustcClick for full compiler diagnostic
        main.rs(47, 5): candidate #1 is defined in an impl of the trait `Nice` for the type `Bad`
        main.rs(54, 5): candidate #2 is defined in an impl of the trait `Nicer` for the type `Bad`
        main.rs(71, 5): disambiguate the method for candidate #1: `Nice::nice(&b)`
        main.rs(71, 5): disambiguate the method for candidate #2: `Nicer::nice(&b)`

    let mut b = Bad;

    b.nice();
    b.change();
    b.consume();

    */
}

/*

下面两种实现的方法性能一样:
- 为 n 个类型实现某个trait，就会生成 n 组方法。
  比如这里就是：为Good和Hello实现 Nice trait，一共生成了 2*3=6 组方法
- 而普通的impl <某个structure>也会生成一个方法

  playground::main:
    pushq	%rax
    leaq	6(%rsp), %rdi
    callq	<playground::Hello as playground::Nice>::nice
    leaq	6(%rsp), %rdi
    callq	<playground::Hello as playground::Nice>::change
    callq	<playground::Hello as playground::Nice>::consume
    leaq	7(%rsp), %rdi
    callq	playground::Good::good
    leaq	7(%rsp), %rdi
    callq	<playground::Good as playground::Nice>::nice
    leaq	7(%rsp), %rdi
    callq	<playground::Good as playground::Nice>::change
    callq	<playground::Good as playground::Nice>::consume
    popq	%rax
    retq

*/
