type Counter = @{ x: int };
let Counter() = memcpy(@{ x: 0 });

let up(self: Counter) = { let self.x + 1 };
let down(self: Counter) = { let self.x - 1 };
let reset(self: Counter, value: int) = { let self.x = value };
let get(self: Counter) = self.x;

let obj = Counter();
obj.up(); obj.up(); obj.up();
obj.reset(obj.get() * 10);
obj.down(); obj.down();

obj.get()
