import Piece from './piece.js';
import Colors from '../colors.js';

class Pawn extends Piece {
  constructor(x, y, color) {
    super(x, y, color);
    this.direction = color == Colors.WHITE ? -1 : 1;
    this.lastMoveWasTwo = false;
  }

  static createMoves(piece, game) {
    let moves = [[piece.x, piece.y + piece.direction]];
    if (!this.hasBeenMoved) {
      moves.push([piece.x, piece.y + 2 * piece.direction]);
    }
    const leftCaptureLegal = game.legalMove([piece.x - 1, piece.y + piece.direction], piece);
    const rightCaptureLegal = game.legalMove([piece.x + 1, piece.y + piece.direction], piece);
    if (leftCaptureLegal && (!game.board.isEmpty(piece.x - 1, piece.y + piece.direction) || game.board.board[piece.y][piece.x-1]?.lastMoveWasTwo)) {
      moves.push([piece.x - 1, piece.y + piece.direction]);
    }
    if (rightCaptureLegal && (!game.board.isEmpty(piece.x + 1, piece.y + piece.direction) || game.board.board[piece.y][piece.x+1]?.lastMoveWasTwo)) {
      moves.push([piece.x + 1, piece.y + piece.direction]);
    }
    return moves;
  }

  generateMoves(game) {
    return Pawn.createMoves(this, game);
  }

  serialize() {
    return this.color == Colors.WHITE ? 'P' : 'p';
  }
}

export default Pawn;