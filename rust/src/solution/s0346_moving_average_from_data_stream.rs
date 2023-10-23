struct MovingAverage {
    size: i32,
    queue: Vec<i32>,
    sum: i32,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            size,
            queue: Vec::new(),
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        let mut sum = self.sum;
        if self.queue.len() == self.size as usize {
            let head = self.queue.remove(0);
            self.sum -= head;
        }
        self.queue.push(val);
        self.sum += val;
        println!("sum: {}", self.sum);
        self.sum as f64 / self.queue.len() as f64
    }
}
