pub fn execute(_serial: i32) -> ((i32, i32, i32, i32), (i32, i32, i32, i32)) {
    (calculate(_serial,3,3), calculate(_serial,0 ,299))
}

fn calculate(_serial: i32, _min_square_size: i32 ,_max_square_size: i32) -> (i32, i32, i32, i32) {
    let mut board: Vec<Vec<i32>> = vec![vec![0; 300]; 300];
    (0..300).for_each(|i| (0..300).for_each(|j| {
        board[i][j] = fuel_power(i as i32,j as i32,_serial);
    }));

    let mut max_fuel = (0,0,0,0); // x,y, size, value

    for square_size in _min_square_size..=_max_square_size{
        for x in 0..300 - square_size {
            for y in 0..300 - square_size {
                let mut total_fuel_power = 0;
                for delta_x in 0..square_size{
                    for delta_y in 0..square_size{
                        total_fuel_power += board[(x+delta_x) as usize][(y+delta_y) as usize];
                    }
                }
                if total_fuel_power > max_fuel.3 {
                    max_fuel = (x,y,square_size,total_fuel_power);
                }
            }
        }
    }
    max_fuel
}

fn fuel_power(_x:i32, _y:i32, _serial: i32) -> i32{
    let rack_id = _x + 10;
    let tmp = (rack_id * _y + _serial) * rack_id;
    return ((tmp / 100) % 10 ) - 5;
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_1(){
        assert_eq!(4,fuel_power(3,5,8));
    }
}

