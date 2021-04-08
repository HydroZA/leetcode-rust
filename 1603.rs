/*
Design a parking system for a parking lot. The parking lot has three kinds of parking spaces: big, medium, and small, with a fixed number of slots for each size.

Implement the ParkingSystem class:
    ParkingSystem(int big, int medium, int small) Initializes object of the ParkingSystem class. The number of slots for each parking space are given as part of the constructor.
    bool addCar(int carType) Checks whether there is a parking space of carType for the car that wants to get into the parking lot. carType can be of three kinds: big, medium, or small, which are represented by 1, 2, and 3 respectively. A car can only park in a parking space of its carType. If there is no space available, return false, else park the car in that size space and return true.

Runtime: 16 ms, faster than 88.64% of Rust online submissions for Design Parking System.
Memory Usage: 2.4 MB, less than 100.00% of Rust online submissions for Design Parking System.
*/

struct ParkingSystem 
{
    big: i32,
    medium: i32,
    small: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self
    {
        ParkingSystem
        {
            big,
            medium,
            small
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool 
    {
        match car_type
        {
            1 => if self.big <= 0 
                    { false } else { self.big -= 1; true },
            2 => if self.medium <= 0 
                    { false } else { self.medium -= 1; true },
            3 => if self.small <= 0 
                    { false } else { self.small -= 1; true },
            _ => false
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */