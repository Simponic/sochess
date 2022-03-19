import Piece from './piece.js';
import Colors from '../colors.js';

class Knight extends Piece {
  constructor(x, y, color) {
    super(x, y, color);
  }

  static createMoves(piece, game) {
    return [
      [piece.x + 2, piece.y + 1],
      [piece.x + 2, piece.y - 1],
      [piece.x - 2, piece.y + 1],
      [piece.x - 2, piece.y - 1],
      [piece.x + 1, piece.y + 2],
      [piece.x + 1, piece.y - 2],
      [piece.x - 1, piece.y + 2],
      [piece.x - 1, piece.y - 2],
    ].filter((move) => game.legalMove(move, piece));
  }

  generateMoves(game) {
    return Knight.createMoves(this, game);
  }

  serialize() {
    return this.color == Colors.WHITE ? 'N' : 'n';
  }
}

export default Knight;