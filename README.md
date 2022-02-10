# wasm-ludo
School project built in **WASM**. Frontend is built in **yew** and server in **actix**.

## Setup 

### Client
- **install dependencies**

  - ```cargo install trunk```
  - ```npm install -D tailwindcss``` or ```yarn global add --dev tailwindcss```
  - ```cargo install just```

- **running the client** 
  - ```just run``` (opens up two new terminals with listeners - **tailwind** and **trunk**)

### Server

- **running the server**
  - ```cargo run```

### Environment

- server runs on ```localhost:8080``` and the client runs on ```localhost:3000```

## Rules

- The game is played in a clockwise order, green player starts.
- Each player starts with four pieces in their starting corner. Player who manages to get all pieces to finish wins the game.
- To promote a piece (get it onto the board from the starting corner), the player has throw a six.
- When a player throws a six, he gets a bonus throw. If he throws a six again, he gets another bonus throw. If he manages to get a third six in a row, he gets 'punished' - his total throw is equal to zero and his move gets skipped. Otherwise the total throw is equal to sum of individual throws (e.g. 6+6+3). Player can decide whether he wants to promote a piece or move his other pieces on the board - if he is able to.
- Player is only able to move (or promote) his piece if he doesn't get blocked by his own piece. If a player wants to move his piece to a position X, and position X is:
    - already occupied by his own piece, this move is invalid.
    - empty, the player can move to that position.
    - occupied by opponent's piece, the player can move to that position. Opponent's piece will be removed from the board and placed in opponent's starting corner.

- If a player has no valid moves (can't move/promote a piece), he gets skipped and it's the next player's turn.
- Each player has a so-called 'home column'. Home column consists of five fields in front of the finish. Player's home column can only be reached by that player, and therefore it is a safe spot.
- To reach the finish, player has to throw the exact number - if our piece is right in front of the finish, we have to throw exactly a one.