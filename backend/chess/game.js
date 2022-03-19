import King from './pieces/king.js';
import _ from 'lodash';
import Colors from './colors.js';

const Status = {
  STALEMATE: 'stalemate',
  CHECKMATE: 'checkmate',
  RESIGNED: 'resigned',
  IN_PROGRESS: 'in progress',
}

class Game {
  constructor(board, whitePlayer, blackPlayer) {
    this.board = board;
    this.whitePlayer = whitePlayer;
    this.blackPlayer = blackPlayer;
    this.winner = null;
    this.status = Status.IN_PROGRESS;
    this.turn = Colors.WHITE;
  }

  legalMove([x, y], piece) {
    return [x,y].every((n) => n >= 0 && n < this.board.dimension) && (this.board.isEmpty(x, y) || !this.board.isSameColor(x, y, piece.color));
  }

  swapTurn() {
    this.turn = this.turn === Colors.WHITE ? Colors.BLACK : Colors.WHITE;
  }

  inCheck(color) {
    const pieces = this.board.getPieces();
    const king = pieces.find((piece) => piece instanceof King && piece.color === color);
    return pieces.find((piece) => piece.color !== color && piece.generateMoves(this).some(([x, y]) => x === king.x && y === king.y));
  }

  canMove(color) {
    const allMoves = this.board.getPieces().filter((x) => x.color == color).map((piece) => piece.generateMoves(this));
    let boardOriginal = _.cloneDeep(this.board);
    console.log(boardOriginal);
    return true;
  }

  move(coord1, coord2) {
    const [x1, y1] = coord1;
    const [x2, y2] = coord2;
    const piece1 = this.board.board[y1][x1];
    const piece2 = this.board.board[y2][x2];

    const piece1Moves = piece1.generateMoves(this);
    if (!piece1Moves.some(([x, y]) => x === x2 && y === y2)) {
      return this.board;
    }

    this.board.board[y1][x1] = null;
    this.board.board[y2][x2] = piece1;
    piece1.x = x2; piece1.y = y2;

    if (this.inCheck(this.turn)) {
      this.board.board[y1][x1] = piece1;
      this.board.board[y2][x2] = piece2;
      piece1.x = x1; piece1.y = y1;
      return this.board;
    }

    piece1.hasBeenMoved = true;
    this.swapTurn();

    return this.board;
  }
}

export default Game;