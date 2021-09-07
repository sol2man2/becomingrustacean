fn first() {
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();

    println!("v: {:?}", v);
    println!("u: {:?}", u);
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);

    // A full range clears the vector
    v.drain(..);
    println!("v: {:?}", v);
    assert_eq!(v, &[]);
}

fn drain() {
    // 컨테이너를 소비하지 않고 컨테이너 밖으로 데이터 추출
    // drain은 iterator이다.
    use std::mem;
    let mut vec = vec![Box::new(9); 4];
    // println!(": {:?}", vec);

    {
        // start draining, vec can no longer be accessed
        let mut drainer = vec.drain(..);

        // pull out two elements and immediately drop them
        drainer.next();
        drainer.next();

        // get rid of drainer, but don't call its destructor
        mem::forget(drainer);
    }

    // Oops, vec[0] was dropped, we're reading a pointer into free'd memory!
    // println!("{}", vec[0]);
    // mem::forget은 safe 코드지만 실행하면 누수 발생
}

fn rc() {
    // Rc는 그냥보면 전혀 proxy 같지 않다.

    use std::alloc;
    use std::ptr;
    struct Rc<T> {
        ptr: *mut RcBox<T>,
    }
    struct RcBox<T> {
        data: T,
        ref_count: usize,
    }
    impl<T> Rc<T> {
        fn new(data: T) -> Self {
            unsafe {
                // Wouldn't it be nice if heap::allocate worked like this?
                let ptr = heap::allocate::<RcBox<T>>();
                ptr::write(
                    ptr,
                    RcBox {
                        data: data,
                        ref_count: 1,
                    },
                );
                Rc { ptr: ptr }
            }
        }
        fn clone(&self) -> Self {
            unsafe {
                (*self.ptr).ref_count += 1;
            }
            Rc { ptr: self.ptr }
        }
    }
    impl<T> Drop for Rc<T> {
        fn drop(&mut self) {
            unsafe {
                (*self.ptr).ref_count -= 1;
                if (*self.ptr).ref_count == 0 {
                    // drop the data and then free it
                    ptr::read(self.ptr);
                    heap::deallocate(self.ptr);
                }
            }
        }
    }
}

// thread::scoped API는 부모 스택을 참조하여 부화한다.
// 공유 데이터가 범위를 빠져나가기 전에 부모는 thread에 join수행

// pub fn scoped<'a, F>(f: F) -> JoinGuard<'a>
//     where F: FnOnce() + Send + 'a

fn join_guard() {
    use crossbeam::scope;
    use std::mem;
    use std::thread;
    let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    {
        let guards = vec![];
        for x in &mut data {
            // Move the mutable reference into the closure, and execute
            // it on a different thread. The closure has a lifetime bound
            // by the lifetime of the mutable reference `x` we store in it.
            // The guard that is returned is in turn assigned the lifetime
            // of the closure, so it also mutably borrows `data` as `x` did.
            // This means we cannot access `data` until the guard goes away.
            let guard = thread::scoped(move || {
                *x *= 2;
            });
            // store the thread's guard for later
            guards.push(guard);
        }
        // All guards are dropped here, forcing the threads to join
        // (this thread blocks here until the others terminate).
        // Once the threads join, the borrow expires and the data becomes
        // accessible again in this thread.
    }
    // data is definitely mutated here.

    let mut data = Box::new(0);
    {
        let guard = thread::scoped(|| {
            // This is at best a data race. At worst, it's also a use-after-free.
            *data += 1;
        });
        // Because the guard is forgotten, expiring the loan without blocking this
        // thread.
        mem::forget(guard);
    }
    // So the Box is dropped here while the scoped thread may or may not be trying
    // to access it.
}

fn main() {
    println!("Ch6-3!");
    // Leaking
    // OBRM은 RAII와 같아서 메모리 관리를 잊어도 된다.
    // rust가 메모리 관리를 잘한다고 믿기에 누수에 놀랄 것이다.
    // 제거할 수 없는 메모리를 누수로 보는 이상을 누수로 보자
    // 프로그램 초기에 대량의 메모리를 잡고 사용하지 않을 때
    // mem::forget은 전달된 메모리를 소비하고 destructor를 호출하지 않는다.
    // safe코드는 안전하리라 보지만 unsafe는 destructor에 의존할 수 없다.
    // 간단한 메모리 누수를 겪도라도 크게 중요하지 않다.
    // 그러나 destructor와 주의할 것은 proxy type이다.
    // vec::Drain, Rc, therad::scoped::JoinGuard 등

    first();
    drain();
    rc();
    join_guard();
}
