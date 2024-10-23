#[derive(Debug, Default)]
pub struct ServerOwnBoard {
    board: [[ServerOwnTile; 10]; 10],
    ships_placed: bool,
}

#[derive(Debug, Default)]
pub struct ServerEnemyBoard {

}

#[derive(Debug, Default)]
pub struct ServerOwnTile {

}

#[derive(Debug, Default)]
pub struct ServerEnemyTile {

}