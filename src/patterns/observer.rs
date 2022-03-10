pub struct Observer<'a, T> {
    pub on_change: Box<dyn FnMut(&T) + 'a>,
}

impl<'a, T> Observer<'a, T> {
    pub fn new(on_change: impl FnMut(&T) + 'a) -> Observer<'a, T> {
        Observer {
            on_change: Box::new(on_change),
        }
    }
}

impl<'a, T> IObserve<T> for Observer<'a, T> {
    fn on_change(&mut self, data: &T) {
        (self.on_change)(data);
    }
}

pub trait IListen<'a, T> {
    fn subscribe(&mut self, observer: impl IObserve<T> + 'a);
    fn unsubscribe(&mut self, observer: impl IObserve<T> + 'a);
}

pub trait INotify<T> {
    fn notify(&mut self, data: &T);
}

pub trait IObserve<T> {
    fn on_change(&mut self, data: &T);
}

pub struct Observable<'a, T> {
    observers: Vec<Box<dyn IObserve<T> + 'a>>,
}

impl<'a, T> Observable<'a, T> {
    pub fn new() -> Observable<'a, T> {
        Observable {
            observers: Vec::new(),
        }
    }
}

impl<'a, T> IListen<'a, T> for Observable<'a, T> {
    fn subscribe(&mut self, observer: impl IObserve<T> + 'a) {
        self.observers.push(Box::new(observer));
    }

    fn unsubscribe(&mut self, _observer: impl IObserve<T> + 'a) {
        unimplemented!();
    }
}

impl<'a, T> INotify<T> for Observable<'a, T> {
    fn notify(&mut self, data: &T) {
        for observer in self.observers.iter_mut() {
            observer.on_change(data);
        }
    }
}
