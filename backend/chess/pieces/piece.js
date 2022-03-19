class Piece {
  constructor(x, y, color) {
    this.color = color;
    this.x = x;
    this.y = y;
    this.hasBeenMoved = false;
  }

  static fromSerialized(serialized) {
    if (serialized === " ") {
      return null;
    }
    color = (serialized.toUpperCase() === serialized) ? Colors.WHITE : Colors.BLACK;

    serialized = serialized.toLowerCase();
    switch (serialized) {
//      case "p":
//        return new Pawn(color);
      case "r":
        return new Rook(color);
      case "n":
        return new Knight(color);
      case "b":
        return new Bishop(color);
      case "q":
        return new Queen(color);
      case "k":
        return new King(color);
      default:
        throw new Error("Invalid serialized piece");
    }
  }
}

export default Piece;