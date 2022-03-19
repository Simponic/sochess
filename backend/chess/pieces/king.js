import Piece from './piece.js';
import Queen from './queen.js';

class King extends Piece {
  constructor(x, y, color) {
    super(x, y, color);
  }

  static createMoves(piece, game) {
    return [...Queen.createMoves(piece, game)].filter(([x,y]) => Math.abs(piece.x - x) <= 1 && Math.abs(piece.y - y) <= 1);
  }  

  generateMoves(game) {
    return King.createMoves(this, game);
  }

  inCheck(board) {
    return board.inCheck(this.color);
  }

  serialize() {
    return this.color == Colors.WHITE ? 'K' : 'k';
  }
}

export default King;