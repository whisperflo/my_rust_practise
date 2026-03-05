// pub不写就不能被外部访问
// 普通函数
pub fn hello() {
    println!("hello main.rs, I'm method.rs")
}

// 方法s
pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法 &self表示借用当前的Circle结构体
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn print(&self) {
        println!("x:{},y:{},radius:{}", self.x, self.y, self.radius)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width
    }
}
