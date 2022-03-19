import Piece from './piece.js';

class Bishop extends Piece {
  constructor(x, y, color) {
    super(x, y, color);
  }

  static createMoves(piece, game) {
    let moves = [];
    const { x, y } = piece;
    let i = x;
    while (game.legalMove([++i, y+(x-i)], piece)) {
      moves.push([i, y+(x-i)]);
      if (!game.board.isEmpty(i, y+(x-i)))
        break;
    }
    i = x;
    while (game.legalMove([--i, y-(x-i)], piece)) {
      moves.push([i, y-(x-i)]);
      if (!game.board.isEmpty(i, y-(x-i)))
        break;
    }
    i = y;
    while (game.legalMove([x+(y-(++i)), i], piece)) {
      moves.push([x+(y-i), i]);
      if (!game.board.isEmpty(x+(y-i), i))
        break;
    }
    i = y;
    while (game.legalMove([x-(y-(++i)), i], piece)) {
      moves.push([x-(y-i), i]);
      if (!game.board.isEmpty(x-(y-i), i))
        break;
    }
    return moves;
  }

  generateMoves(game) {
    return Bishop.createMoves(this, game);
  }

  serialize() {
    return this.color == Colors.WHITE ? 'B' : 'b';
  }
}

export default Bishop;