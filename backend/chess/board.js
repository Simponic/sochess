import Colors from './colors.js';
import Knight from './pieces/knight.js';
import Rook from './pieces/rook.js';
import Bishop from './pieces/bishop.js';
import Queen from './pieces/queen.js';
import King from './pieces/king.js';
import Pawn from './pieces/pawn.js';

class Board {
  constructor(board, dimension=8) {
    this.dimension = dimension;
    this.board = board === undefined ? [
      [(new Rook(0, 0, Colors.BLACK)), (new Knight(1, 0, Colors.BLACK)), (new Bishop(2, 0, Colors.BLACK)), (new Queen(3, 0, Colors.BLACK)), (new King(4, 0, Colors.BLACK)), (new Bishop(5, 0, Colors.BLACK)), (new Knight(6, 0, Colors.BLACK)), (new Rook(7, 0, Colors.BLACK))],
      [null, null, null, null, null, null, null, null],
      [null, null, null, null, null, null, null, null],
      [null, null, (new Pawn(2, 3, Colors.BLACK)), (new Pawn(3, 3, Colors.WHITE)), null, null, null, null],
      [null, null, null, null, null, null, null, null],
      [null, null, null, null, null, null, null, null],
      [null, null, null, null, null, null, null, null],
      [(new Rook(0, 7, Colors.WHITE)), (new Knight(1, 7, Colors.WHITE)), (new Bishop(2, 7, Colors.WHITE)), (new Queen(3, 7, Colors.WHITE)), (new King(4, 7, Colors.WHITE)), (new Bishop(5, 7, Colors.WHITE)), (new Knight(6, 7, Colors.WHITE)), (new Rook(7, 7, Colors.WHITE))],
    ] : board;
  }

  isEmpty(x, y) {
    return !(this.board[y][x]);
  }  

  isSameColor(x, y, color) {
    console.log(x, y, color);
    return this.board[y][x]?.color === color;
  }

  getPieces() {
    return this.board.flat().filter((x) => x);
  }  

  serialize() {
    return this.board.map((row) => row.map((piece) => piece ? piece.serialize() : " "));
  }

  static boardGridFromSerialized(serializedBoard) {
    return serializedBoard.map((row) => row.map(Piece.fromSerialized));
  }
}

export default Board;