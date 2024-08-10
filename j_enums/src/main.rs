enum Direction {
    _North,
    _East,
    _West,
    _South,
}

fn main() {
    let my_direction = Direction::_North;
    let new_direction = my_direction; // No error, because direction is copy
    // move_around(my_direction);     // this will throw an error 
    move_around(new_direction);
}

fn move_around(_direction: Direction) {
    // implements logic to move a character around
}
