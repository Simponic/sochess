import King from './pieces/king.js';
import Pawn from './pieces/pawn.js';
import _ from 'lodash';
import Colors from './colors.js';
import Status from './game_status.js';

class Game {
  constructor(board, {whitePlayer, blackPlayer, totalTime, delay, increment, status}=spec) {
    this.board = board;
    this.whitePlayer = whitePlayer;
    this.blackPlayer = blackPlayer;

    this.whiteTime = totalTime;
    this.blackTime = totalTime;
    this.totalTime = totalTime;
    this.delay = delay;
    this.increment = increment;

    this.winner = null;
    this.status = status || Status.IN_PROGRESS;
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
    // TODO: check if there are any legal moves for the given color
    const allMoves = this.board.getPieces().filter((x) => x.color == color).map((piece) => piece.generateMoves(this));
    let boardOriginal = _.cloneDeep(this.board);
    console.log(boardOriginal);
    return true;
  }

  updateState() {
    this.board.getPieces().filter((piece) => piece.color !== this.turn && piece instanceof Pawn).map((x) => x.lastMoveWasTwo = false);
    this.swapTurn();
  } 

  move(coord1, coord2) {
    const [x1, y1] = coord1;
    const [x2, y2] = coord2;
    const piece1 = this.board.board[y1][x1];
    const piece2 = this.board.board[y2][x2];
    if (piece1 && piece1.color === this.turn) {
      const piece1Moves = piece1.generateMoves(this);
      if (!piece1Moves.some(([x, y]) => x === x2 && y === y2)) {
        return this.board;
      }

      if (piece1 instanceof Pawn) {
        if (Math.abs(y1 - y2) === 2) {
          piece1.lastMoveWasTwo = true;
        } else if (Math.abs(y1 - y2) === 1 && Math.abs(x1 - x2) === 1) {
          // En passant
          this.board.board[y1][x2] = null;
        }
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
      this.updateState();
    }
    return this.board;
  }
}

export default Game;