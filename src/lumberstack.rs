pub trait Runnable {
    fn run_job(&self);
}

pub struct Lumberstack {
    pub run_items: Vec<Box<dyn Runnable>>
}

impl Lumberstack {
    pub fn new() -> Lumberstack {
        Lumberstack { run_items: Vec::new() }
    }

    pub fn queue<T: Runnable + 'static>(&mut self, task: Option<T>) -> &mut Self {
        if let Some(runnable) = task  {
            self.run_items.push(Box::new(runnable));
        }
        self
    }

    pub fn process(&mut self) {
        let run_items = &self.run_items;
        run_items.iter().for_each(|item| {
            item.run_job();
        });
    }
}
