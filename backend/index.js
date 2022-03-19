import http from 'http';
import app from './app.js';
import { Server } from "socket.io";
import jwt from "jsonwebtoken";
import { v4 as uuidv4 } from "uuid";
import Board from './chess/board.js';
import Game from './chess/game.js';
import Colors from './chess/colors.js';
import Status from './chess/game_status.js';

const server = http.createServer(app);
const io = new Server(server, {
  cors: {
    origin: "*",
    methods: ["GET", "POST"],
  }
});

const { API_PORT } = process.env;
const port = process.env.PORT || API_PORT;

const games = {};
const user_games = {};

io.use(function(socket, next){
  if (socket.handshake.query && socket.handshake.query.token){
    jwt.verify(socket.handshake.query.token, process.env.TOKEN_KEY, function(err, decoded) {
      if (err) return next(new Error('Authentication error'));
      socket.decoded = decoded;
      next();
    });
  }
  else {
    next(new Error('Authentication error'));
  }    
})
.on('connection', function(socket) {
  console.log(games);
  socket.on('create', function(body) {
    if (!user_games[socket.decoded.user_id]) {
      const id = uuidv4();
      const board = new Board();
      const game = new Game(board, {
        whitePlayer: body.color === Colors.WHITE ? socket.decoded.user_id : null,
        blackPlayer: body.color === Colors.BLACK ? socket.decoded.user_id : null,
        totalTime: body.totalTime,
        increment: body.increment,
        delay: body.delay,
        status: Status.WAITING_FOR_OPPONENT,
      });
      games[id] = game;
      user_games[socket.decoded.user_id] = id;
      socket.color = body.color;
      socket.join(id);
      socket.emit('created', id);
      return;
    }
    socket.emit('error', 'You are already in a game');
  });

  socket.on('join', function({ id }) {
    if (user_games[socket.decoded.user_id] !== undefined) {
      socket.emit('error', 'You are already in a game');
      return;
    }
    if (!games[id]) {
      socket.emit('error', 'Game not found');
      return;
    }
    const game = games[id];
    if (game.whitePlayer && game.blackPlayer) {
      socket.emit('error', 'Game is full');
      return;
    } else if (game.whitePlayer && !game.blackPlayer) {
      game.blackPlayer = socket.decoded.user_id;
      socket.color = Colors.WHITE;
    } else if (!game.whitePlayer && game.blackPlayer) {
      game.whitePlayer = socket.decoded.user_id;
      socket.color = Colors.WHITE;
    }
    user_games[socket.decoded.user_id] = id;
    games[id].status = Status.IN_PROGRESS;
    socket.join(id);
  });

  socket.on('move', function ({from, to}) {
    try {
      const game_id = user_games[socket.decoded.user_id];
      if (!game_id) {
        socket.emit('error', 'You are not in a game');
        return;
      }
      const game = games[game_id];
      if (game.status == Status.IN_PROGRESS) {
        const [x, y] = from;
        if(game.board.isSameColor(x, y, socket.color))
        {
          console.log(socket.color);
          game.move(from, to);
          socket.to(user_games[socket.decoded.user_id]).emit('update', game.board.serialize());
          return;
        }
        socket.emit('error', 'You are not allowed to move that piece');
        return;
      }
      socket.emit('error', 'Game is not in progress; wait for another player');
    } catch (e) {
      socket.emit('error', 'Error in move');
      socket.to(user_games[socket.decoded.user_id]).emit('update', game.board.serialize());
    }
  });
});

server.listen(port, () => {
  console.log(`Server running on port ${port}`);
});