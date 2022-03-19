import Piece from './piece.js';

class Rook extends Piece {
  constructor(x, y, color) {
    super(x, y, color);
  }

  static createMoves(piece, game) {
    let moves = [];
    const { x, y } = piece;
    let i = x;
    while (game.legalMove([++i, y], piece)) {
      moves.push([i, y]);
      if (!game.board.isEmpty(i, y))
        break;
    }
    i = x;
    while (game.legalMove([--i, y], piece)) {
      moves.push([i, y]);
      if (!game.board.isEmpty(i, y))
        break;
    }
    i = y;
    while (game.legalMove([x, ++i], piece)) {
      moves.push([x, i]);
      if (!game.board.isEmpty(x, i))
        break;
    }
    i = y;
    while (game.legalMove([x, --i], piece)) {
      moves.push([x, i]);
      if (!game.board.isEmpty(x, i))
        break;
    }
    return moves;
  }

  generateMoves(game) {
    return Rook.createMoves(this, game);
  }

  serialize() {
    return this.color == Colors.WHITE ? 'R' : 'r';
  }
}

export default Rook;