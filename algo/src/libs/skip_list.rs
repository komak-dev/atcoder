
/* skip_list {{{ */
mod skip_list {

    use rand::Rng;
    use std::{
        fmt::Debug,
        ops::{Index, IndexMut},
        alloc::{alloc, dealloc, Layout},
        ptr::{null_mut, read, write},
    };

    const MAX_LEVEL: usize = 20;



    /* {{{ Node */
    struct Node<T> {
        value: *mut T,
        next: [*mut Self; MAX_LEVEL],
        skip: [usize; MAX_LEVEL],
    }

    impl<T> Node<T> {
        const LAYOUT: Layout = Layout::new::<Self>();
        const LAYOUT_T: Layout = Layout::new::<T>();

        fn alloc(value: T) -> *mut Self {
            unsafe {
                let ptr = alloc(Self::LAYOUT) as *mut Self;
                (*ptr).value = alloc(Self::LAYOUT_T) as *mut T;
                write((*ptr).value, value);
                ptr
            }
        }

        fn dealloc(ptr: *mut Self) -> T {
            unsafe {
                let value = read(&*((*ptr).value));
                dealloc((*ptr).value as *mut u8, Self::LAYOUT_T);
                dealloc(ptr as *mut u8, Self::LAYOUT);
                value
            }
        }
    }
    /* }}} */



    /* {{{ List */
    pub struct List<T> {
        head: Node<T>,
        len: usize,
        rng: rand::rngs::ThreadRng,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                head: Node {
                    value: null_mut(),
                    next: [null_mut(); MAX_LEVEL],
                    skip: [1; MAX_LEVEL],
                },
                len: 0,
                rng: rand::thread_rng(),
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        fn gen_level(&mut self) -> usize {
            let mut level = 1;
            while self.rng.gen_bool(0.5) && level < MAX_LEVEL {
                level += 1;
            }
            level
        }

        pub fn insert(&mut self, mut index: usize, element: T) {
            if index > self.len {
                panic!("index out of bounds");
            }

            self.len += 1;

            let new_node = Node::alloc(element);
            let new_level = self.gen_level();

            let mut cur = &mut self.head as *mut Node<T>;

            for l in (0..MAX_LEVEL).rev() {
                unsafe {
                    while (*cur).skip[l] <= index {
                        index -= (*cur).skip[l];
                        cur = (*cur).next[l];
                    }
                    if l < new_level {
                        (*new_node).next[l] = (*cur).next[l];
                        (*cur).next[l] = new_node;
                        (*new_node).skip[l] = (*cur).skip[l] - index;
                        (*cur).skip[l] = index + 1;
                    } else {
                        (*cur).skip[l] += 1;
                    }
                }
            }
        }

        pub fn remove(&mut self, mut index: usize) -> T {
            if index >= self.len {
                panic!("index out of bounds");
            }

            self.len -= 1;

            let mut cur = &mut self.head as *mut Node<T>;
            index += 1;

            for l in (0..MAX_LEVEL).rev() {
                unsafe {
                    while (*cur).skip[l] < index {
                        index -= (*cur).skip[l];
                        cur = (*cur).next[l];
                    }
                    if (*cur).skip[l] == index {
                        let next = (*cur).next[l];
                        (*cur).next[l] = (*next).next[l];
                        (*cur).skip[l] += (*next).skip[l] - 1;
                        if l == 0 {
                            return Node::dealloc(next);
                        }
                    } else {
                        (*cur).skip[l] -= 1;
                    }
                }
            }

            unreachable!()
        }

        pub fn push_back(&mut self, element: T) {
            self.insert(self.len, element);
        }

        pub fn push_front(&mut self, element: T) {
            self.insert(0, element);
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                Some(self.remove(self.len - 1))
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            match self.len {
                0 => None,
                _ => Some(self.remove(0)),
            }
        }

        pub fn first(&self) -> Option<&T> {
            match self.len {
                0 => None,
                _ => Some(&self[0]),
            }
        }

        pub fn last(&self) -> Option<&T> {
            match self.len {
                0 => None,
                _ => Some(&self[self.len - 1]),
            }
        }

        pub fn iter(&self) -> Iter<T> {
            Iter {
                _target: self,
                cur: &self.head,
            }
        }

        pub fn iter_mut(&mut self) -> IterMut<T> {
            let cur = &mut self.head as *mut Node<T>;
            IterMut {
                _target: self,
                cur,
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur = self.head.next[0];
            while !cur.is_null() {
                unsafe {
                    let next = (*cur).next[0];
                    Node::dealloc(cur);
                    cur = next;
                }
            }
        }
    }

    impl<T: Debug> Debug for List<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }


    pub struct Iter<'a, T> {
        _target: &'a List<T>,
        cur: *const Node<T>,
    }

    pub struct IterMut<'a, T> {
        _target: &'a List<T>,
        cur: *mut Node<T>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if unsafe { (*self.cur).next[0].is_null() } {
                None
            } else {
                self.cur = unsafe { (*self.cur).next[0] };
                Some(unsafe { &*(*self.cur).value })
            }
        }
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            if unsafe { (*self.cur).next[0].is_null() } {
                None
            } else {
                self.cur = unsafe { (*self.cur).next[0] };
                Some(unsafe { &mut *(*self.cur).value })
            }
        }
    }

    impl<'a, T> IntoIterator for &'a List<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut List<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }

    impl<T> Index<usize> for List<T> {
        type Output = T;

        fn index(&self, mut index: usize) -> &Self::Output {
            if index >= self.len {
                panic!("index out of bounds");
            }

            let mut cur = &self.head as *const Node<T>;
            index += 1;

            for l in (0..MAX_LEVEL).rev() {
                unsafe {
                    while (*cur).skip[l] <= index {
                        index -= (*cur).skip[l];
                        cur = (*cur).next[l];
                    }
                }
            }

            unsafe { &*(*cur).value }
        }
    }

    impl<T> IndexMut<usize> for List<T> {
        fn index_mut(&mut self, mut index: usize) -> &mut Self::Output {
            if index >= self.len {
                panic!("index out of bounds");
            }

            let mut cur = &mut self.head as *mut Node<T>;
            index += 1;

            for l in (0..MAX_LEVEL).rev() {
                unsafe {
                    while (*cur).skip[l] <= index {
                        index -= (*cur).skip[l];
                        cur = (*cur).next[l];
                    }
                }
            }

            unsafe { &mut *(*cur).value }
        }
    }

    impl<T> FromIterator<T> for List<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = List::new();
            for element in iter {
                list.push_back(element);
            }
            list
        }
    }
    /* }}} */



    /* {{{ SortedList */
    pub struct SortedList<T: Ord> {
        list: List<T>,
    }

    impl<T: Ord> SortedList<T> {
        pub fn new() -> Self {
            SortedList {
                list: List::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.list.len()
        }

        pub fn lower_bound(&self, element: &T) -> usize {
            let mut index = 0;
            let mut cur = &self.list.head as *const Node<T>;

            for l in (0..MAX_LEVEL).rev() {
                unsafe {
                    while !(*cur).next[l].is_null() && *(*(*cur).next[l]).value < *element {
                        index += (*cur).skip[l];
                        cur = (*cur).next[l];
                    }
                }
            }

            index
        }

        pub fn upper_bound(&self, element: &T) -> usize {
            let mut index = 0;
            let mut cur = &self.list.head as *const Node<T>;

            for l in (0..MAX_LEVEL).rev() {
                unsafe {
                    while !(*cur).next[l].is_null() && *(*(*cur).next[l]).value <= *element {
                        index += (*cur).skip[l];
                        cur = (*cur).next[l];
                    }
                }
            }

            index
        }

        pub fn contains(&self, element: &T) -> bool {
            self.lower_bound(element) != self.upper_bound(element)
        }

        pub fn count(&self, element: &T) -> usize {
            self.upper_bound(element) - self.lower_bound(element)
        }

        pub fn insert(&mut self, element: T) {
            self.list.insert(self.lower_bound(&element), element);
        }

        pub fn remove(&mut self, element: &T) -> bool {
            if !self.contains(element) {
                return false;
            }
            self.list.remove(self.lower_bound(element));
            true
        }

        pub fn first(&self) -> Option<&T> {
            self.list.first()
        }

        pub fn last(&self) -> Option<&T> {
            self.list.last()
        }

        pub fn pop_first(&mut self) -> Option<T> {
            self.list.pop_front()
        }

        pub fn pop_last(&mut self) -> Option<T> {
            self.list.pop_back()
        }

        pub fn iter(&self) -> Iter<T> {
            self.list.iter()
        }
    }


    impl<T: Debug + Ord> Debug for SortedList<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    impl<'a, T: Ord> IntoIterator for &'a SortedList<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<T: Ord> Index<usize> for SortedList<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.list[index]
        }
    }

    impl<T: Ord> FromIterator<T> for SortedList<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = SortedList::new();
            for element in iter {
                list.insert(element);
            }
            list
        }
    }
    /* }}} */

}
/* }}} */
