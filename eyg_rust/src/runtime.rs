use std::collections::VecDeque;

pub struct OrderedRuntime<S> {
    pub system: S
}

impl<S> OrderedRuntime<S> {

    pub fn new(system: S) -> Self {
        Self{system: system}
    }

    pub fn dispatch(mut self, mail: crate::envelope::Mail<S>) -> Self {
        let mut queue = VecDeque::new();
        for m in mail {
            queue.push_back(m)
        }

        while let Some(e) = queue.pop_front() {
            let (next, new_system) = e.deliver(self.system);
            self.system = new_system;
            for n in next {
                queue.push_back(n)
            }
        }
        self
    }
}
