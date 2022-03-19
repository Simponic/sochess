import Piece from './piece.js';
import Bishop from './bishop.js';
import Rook from './rook.js';

class Queen extends Piece {
  constructor(x, y, color) {
    super(x, y, color);
  }

  static createMoves(piece, game) {
    return [...Bishop.createMoves(piece, game), ...Rook.createMoves(piece, game)];
  }

  generateMoves(game) {
    return Queen.createMoves(this, game);
  }

  serialize() {
    return this.color == Colors.WHITE ? 'Q' : 'q';
  }
}

export default Queen;